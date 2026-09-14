// 配置管理: 路径 + 启动设置 + 持久化到 %APPDATA%/com.ethanwu.yunzai/config.json

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppPaths {
    /// Redis server 可执行文件 (默认 D:\TRSSYUNZAI\redis-windows-7.0.4\redis-server.exe)
    pub redis_exe: String,
    /// Redis 配置文件
    pub redis_conf: String,
    /// Yunzai 后端目录 (含 app.js)
    pub yunzai_dir: String,
    /// Yunzai 用的 node.exe (nvm 管理的 v20.11.1)
    pub yunzai_node: String,
    /// NapCat 目录 (含 launcher.bat)
    pub napcat_dir: String,
    /// NapCat 日志目录
    pub napcat_log_dir: String,
    /// Yunzai 日志目录
    pub yunzai_log_dir: String,
}

impl Default for AppPaths {
    fn default() -> Self {
        // 占位符默认值 - 用户首次启动后通过 UI 修改为真实路径
        Self {
            redis_exe: r"C:\TRSSYUNZAI\redis-windows-7.0.4\redis-server.exe".into(),
            redis_conf: r"C:\TRSSYUNZAI\redis-windows-7.0.4\redis.conf".into(),
            yunzai_dir: r"C:\TRSSYUNZAI\Yunzai-Bot".into(),
            yunzai_node: r"C:\Users\<USERNAME>\AppData\Local\nvm\v20.11.1\node.exe".into(),
            napcat_dir: r"C:\NapCat.Shell".into(),
            napcat_log_dir: r"C:\NapCat.Shell\napcat\logs".into(),
            yunzai_log_dir: r"C:\TRSSYUNZAI\Yunzai-Bot\logs".into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    /// 关窗行为: minimize_to_tray (默认) | exit
    pub close_action: String,
    /// 开机自启
    pub autostart: bool,
    /// 启动后自动拉起所有服务
    pub auto_start_on_launch: bool,
    /// 启动 NapCat 时是否以管理员权限 (Yunzai 不需要,NapCat 注入需要)
    pub napcat_elevated: bool,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            close_action: "minimize_to_tray".into(),
            autostart: false,
            auto_start_on_launch: false,
            napcat_elevated: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppConfig {
    pub paths: AppPaths,
    pub settings: AppSettings,
}

impl AppConfig {
    pub fn config_path(app: &tauri::AppHandle) -> anyhow::Result<PathBuf> {
        use tauri::Manager;
        let dir = app.path().app_config_dir()?;
        if !dir.exists() {
            std::fs::create_dir_all(&dir)?;
        }
        Ok(dir.join("config.json"))
    }

    pub fn load(path: &std::path::Path) -> Self {
        if path.exists() {
            match std::fs::read_to_string(path) {
                Ok(s) => match serde_json::from_str::<AppConfig>(&s) {
                    Ok(c) => {
                        log::info!("已加载配置: {}", path.display());
                        return c;
                    }
                    Err(e) => log::warn!("配置解析失败,使用默认: {e}"),
                },
                Err(e) => log::warn!("配置读取失败,使用默认: {e}"),
            }
        }
        Self::default()
    }

    pub fn save(&self, path: &std::path::Path) -> anyhow::Result<()> {
        let s = serde_json::to_string_pretty(self)?;
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(path, s)?;
        log::info!("配置已保存: {}", path.display());
        Ok(())
    }
}
