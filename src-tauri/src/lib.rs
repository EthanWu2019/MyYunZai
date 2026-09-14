// 核心库入口: Tauri Builder + 单实例 + 托盘 + 关窗拦截 + 状态轮询

mod bootstrap;
mod commands;
mod config;
mod log_stream;
mod plugin_index;
mod process;
mod service;
mod setup;
mod state;

use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Emitter, Manager, WindowEvent,
};
use tauri_plugin_autostart::MacosLauncher;

pub use state::AppState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // 初始化日志
    let _ = env_logger::Builder::from_env(
        env_logger::Env::default().default_filter_or("info,tauri=warn,yunzai_app_lib=debug"),
    )
    .try_init();

    log::info!("🧽 海绵酱控制台启动");

    tauri::Builder::default()
        // 单实例：第二次启动会聚焦已有窗口
        .plugin(tauri_plugin_single_instance::init(|app, _argv, _cwd| {
            log::info!("检测到重复启动,聚焦主窗口");
            if let Some(win) = app.get_webview_window("main") {
                let _ = win.show();
                let _ = win.unminimize();
                let _ = win.set_focus();
            }
        }))
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            Some(vec!["--minimized"]),
        ))
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![
            commands::get_config,
            commands::update_paths,
            commands::get_status,
            commands::start_all,
            commands::stop_bot,
            commands::open_napcat_webui,
            commands::reveal_in_explorer,
            commands::read_log_tail,
            commands::is_autostart_enabled,
            commands::set_autostart,
            commands::minimize_to_tray,
            commands::quit_app,
            commands::run_setup,
            commands::get_available_plugins,
            commands::check_setup_status,
            commands::extract_embedded_napcat_cmd,
            commands::get_qq_recommendation,
            commands::install_plugin,
            commands::remove_plugin,
        ])
        .setup(|app| {
            // 加载配置 (从 %APPDATA%/com.ethanwu.yunzai/config.json)
            let app_state: tauri::State<AppState> = app.state();
            app_state.load_from_disk(app.handle());

            // 托盘菜单
            let show_item =
                MenuItem::with_id(app, "show", "显示主窗口", true, None::<&str>)?;
            let hide_item =
                MenuItem::with_id(app, "hide", "隐藏到托盘", true, None::<&str>)?;
            let napcat_item =
                MenuItem::with_id(app, "napcat", "打开 NapCat WebUI", true, None::<&str>)?;
            let quit_item =
                MenuItem::with_id(app, "quit", "退出控制台", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&show_item, &hide_item, &napcat_item, &quit_item])?;

            let _tray = TrayIconBuilder::with_id("main-tray")
                .tooltip("海绵酱控制台 · YunZai")
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .show_menu_on_left_click(false)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "show" => {
                        if let Some(win) = app.get_webview_window("main") {
                            let _ = win.show();
                            let _ = win.unminimize();
                            let _ = win.set_focus();
                        }
                    }
                    "hide" => {
                        if let Some(win) = app.get_webview_window("main") {
                            let _ = win.hide();
                        }
                    }
                    "napcat" => {
                        let _ = tauri_plugin_opener::OpenerExt::opener(app).open_url("http://localhost:6099/webui/", None::<&str>);
                    }
                    "quit" => {
                        app.exit(0);
                    }
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        let app = tray.app_handle();
                        if let Some(win) = app.get_webview_window("main") {
                            let _ = win.show();
                            let _ = win.unminimize();
                            let _ = win.set_focus();
                        }
                    }
                })
                .build(app)?;

            // 关窗拦截 → 默认最小化到托盘 (主人偏好)
            if let Some(win) = app.get_webview_window("main") {
                let win_handle = win.clone();
                win.on_window_event(move |event| {
                    if let WindowEvent::CloseRequested { api, .. } = event {
                        let cfg = win_handle
                            .app_handle()
                            .state::<AppState>()
                            .config
                            .lock()
                            .clone();
                        if cfg.settings.close_action == "exit" {
                            api.prevent_close();
                            win_handle.app_handle().exit(0);
                        } else {
                            api.prevent_close();
                            let _ = win_handle.hide();
                            log::info!("窗口已最小化到托盘 (右键托盘图标可退出)");
                        }
                    }
                });
            }

            // 启动后台进程状态轮询 (2 秒一次)
            let app_handle = app.handle().clone();
            std::thread::spawn(move || {
                use std::time::Duration;
                loop {
                    std::thread::sleep(Duration::from_secs(2));
                    let state: tauri::State<AppState> = app_handle.state();
                    let snapshot = state.poll_process_snapshot();
                    let _ = app_handle.emit("status:update", &snapshot);
                }
            });

            // 启动日志流 (notify watcher tail -f)
            let app_handle2 = app.handle().clone();
            std::thread::spawn(move || {
                let state: tauri::State<AppState> = app_handle2.state();
                let paths = state.config.lock().paths.clone();
                log_stream::spawn_watchers(app_handle2.clone(), paths);
            });

            log::info!("✅ Tauri 应用初始化完成");
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("Tauri 启动失败");
}
