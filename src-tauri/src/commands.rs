// Tauri IPC commands: 暴露给前端调用

use crate::bootstrap::{extract_embedded_napcat, qq_recommendation, verify_setup, QqRecommendation, SetupVerification};
use crate::config::{AppConfig, AppPaths, AppSettings};
use crate::plugin_index::{owner_installed_plugins, PluginInfo};
use crate::setup::run_full_setup;
use crate::state::StatusSnapshot;
use crate::AppState;
use std::path::PathBuf;
use tauri::{AppHandle, Emitter, Manager};
use tauri_plugin_autostart::ManagerExt;

#[tauri::command]
pub fn get_config(state: tauri::State<AppState>) -> AppConfig {
    state.config.lock().clone()
}

#[tauri::command]
pub fn update_paths(
    app: AppHandle,
    state: tauri::State<AppState>,
    paths: AppPaths,
    settings: Option<AppSettings>,
) -> Result<(), String> {
    {
        let mut cfg = state.config.lock();
        cfg.paths = paths;
        if let Some(s) = settings {
            cfg.settings = s;
        }
    }
    state.save_config(&app).map_err(|e| e.to_string())?;
    log::info!("配置已更新");
    Ok(())
}

#[tauri::command]
pub fn get_status(state: tauri::State<AppState>) -> StatusSnapshot {
    state.poll_process_snapshot()
}

#[tauri::command]
pub fn start_all(app: AppHandle, state: tauri::State<AppState>) -> Result<StatusSnapshot, String> {
    let _ = state; // 不直接用,改在线程里再拿
    let app_clone = app.clone();
    std::thread::spawn(move || {
        // 在线程里再拿 AppState,避免 borrow check
        let state: tauri::State<AppState> = app_clone.state();
        if let Err(e) = crate::service::start_all(state.inner()) {
            log::error!("启动失败: {e:?}");
            let _ = app_clone.emit("start:failed", e.to_string());
        } else {
            let _ = app_clone.emit("start:ok", ());
        }
    });
    let state: tauri::State<AppState> = app.state();
    Ok(state.poll_process_snapshot())
}

#[tauri::command]
pub fn stop_bot(app: AppHandle, state: tauri::State<AppState>) -> Result<StatusSnapshot, String> {
    let _ = state;
    let app_clone = app.clone();
    std::thread::spawn(move || {
        let state: tauri::State<AppState> = app_clone.state();
        if let Err(e) = crate::service::stop_bot(state.inner()) {
            log::error!("关闭失败: {e:?}");
            let _ = app_clone.emit("stop:failed", e.to_string());
        } else {
            let _ = app_clone.emit("stop:ok", ());
        }
    });
    let state: tauri::State<AppState> = app.state();
    Ok(state.poll_process_snapshot())
}

#[tauri::command]
pub fn open_napcat_webui(app: AppHandle) -> Result<(), String> {
    tauri_plugin_opener::OpenerExt::opener(&app)
        .open_url("http://localhost:6099/webui/", None::<&str>)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn reveal_in_explorer(app: AppHandle, path: String) -> Result<(), String> {
    tauri_plugin_opener::OpenerExt::opener(&app)
        .reveal_item_in_dir(&path)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn read_log_tail(state: tauri::State<AppState>, source: String, lines: usize) -> Result<String, String> {
    let paths = state.paths();
    let dir: PathBuf = if source == "yunzai" {
        PathBuf::from(&paths.yunzai_log_dir)
    } else if source == "napcat" {
        PathBuf::from(&paths.napcat_log_dir)
    } else {
        return Err(format!("未知 source: {source}"));
    };

    // 找最新一个 .log 文件
    let latest = std::fs::read_dir(&dir)
        .map_err(|e| format!("读目录失败: {e}"))?
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path()
                .extension()
                .map(|x| x == "log")
                .unwrap_or(false)
        })
        .max_by_key(|e| e.metadata().ok().and_then(|m| m.modified().ok()))
        .ok_or_else(|| "无日志文件".to_string())?;

    let content = std::fs::read_to_string(latest.path()).map_err(|e| e.to_string())?;
    let tail: Vec<&str> = content.lines().rev().take(lines).collect();
    let mut s: Vec<&str> = tail;
    s.reverse();
    Ok(s.join("\n"))
}

#[tauri::command]
pub fn is_autostart_enabled(app: AppHandle) -> Result<bool, String> {
    app.autolaunch().is_enabled().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn set_autostart(app: AppHandle, enable: bool) -> Result<(), String> {
    let mgr = app.autolaunch();
    if enable {
        mgr.enable().map_err(|e| e.to_string())?;
    } else {
        mgr.disable().map_err(|e| e.to_string())?;
    }
    log::info!("自启已 {}", if enable { "开启" } else { "关闭" });
    Ok(())
}

#[tauri::command]
pub fn minimize_to_tray(window: tauri::Window) -> Result<(), String> {
    window.hide().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn quit_app(app: AppHandle) {
    app.exit(0);
}

// tauri::State<AppState>::inner() 返回 &AppState,无需自定义
// 这里只是文档说明 tauri::State 自动实现 Deref<Target = T>

// AppState 字段都是 Arc<Mutex<...>>,自动 Send + Sync (无需 unsafe impl)
unsafe impl Send for AppState {}
unsafe impl Sync for AppState {}

#[tauri::command]
pub fn run_setup(
    app: AppHandle,
    paths: AppPaths,
    selected_plugins: Vec<String>,
) -> Result<(), String> {
    let app_clone = app.clone();
    std::thread::spawn(move || {
        if let Err(e) = run_full_setup(app_clone.clone(), paths, selected_plugins) {
            log::error!("setup 失败: {e:?}");
            let _ = app_clone.emit("setup:failed", e.to_string());
        } else {
            let _ = app_clone.emit("setup:complete", ());
        }
    });
    Ok(())
}

#[tauri::command]
pub fn get_available_plugins() -> Vec<PluginInfo> {
    owner_installed_plugins()
}

#[tauri::command]
pub fn check_setup_status(install_root: String) -> SetupVerification {
    crate::bootstrap::verify_setup(&install_root)
}

#[tauri::command]
pub fn extract_embedded_napcat_cmd(target_dir: String) -> Result<(), String> {
    crate::bootstrap::extract_embedded_napcat(&target_dir).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_qq_recommendation() -> QqRecommendation {
    qq_recommendation()
}

#[tauri::command]
pub fn install_plugin(app: AppHandle, plugin_name: String, install_root: String) -> Result<(), String> {
    let app_clone = app.clone();
    std::thread::spawn(move || {
        match crate::setup::install_single_plugin(&app_clone, &plugin_name, &install_root) {
            Ok(_) => {
                let _ = app_clone.emit("plugin:installed", plugin_name);
            }
            Err(e) => {
                let _ = app_clone.emit("plugin:install_failed", e.to_string());
            }
        }
    });
    Ok(())
}

#[tauri::command]
pub fn remove_plugin(install_root: String, plugin_name: String) -> Result<(), String> {
    crate::setup::remove_single_plugin(&install_root, &plugin_name).map_err(|e| e.to_string())
}
