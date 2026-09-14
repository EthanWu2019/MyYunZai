// 全局状态: 配置 + 启动子进程的 PID 记录

use crate::config::{AppConfig, AppPaths};
use crate::process::ProcessService;
use parking_lot::Mutex;
use serde::Serialize;
use std::sync::Arc;

#[derive(Clone, Debug, Serialize, Default)]
pub struct ComponentStatus {
    #[serde(default)]
    pub running: bool,
    #[serde(default)]
    pub pid: Option<u32>,
}

#[derive(Clone, Debug, Serialize, Default)]
pub struct StatusSnapshot {
    pub redis: ComponentStatus,
    pub yunzai: ComponentStatus,
    pub napcat: ComponentStatus,
    pub qq: ComponentStatus,
    pub webui_connected: bool,
    pub websocket_connected: bool,
    pub timestamp: i64,
}

#[derive(Default)]
pub struct AppState {
    pub config: Arc<Mutex<AppConfig>>,
    pub proc: Arc<ProcessService>,
    pub children_pid: Arc<Mutex<ChildPids>>,
}

/// 我们拉起的子进程 PID (用于精准杀进程树)
#[derive(Default, Clone, Debug)]
pub struct ChildPids {
    pub redis: Option<u32>,
    pub yunzai_node: Option<u32>,
    pub napcat_main: Option<u32>,
}

impl AppState {
    pub fn load_from_disk(&self, app: &tauri::AppHandle) {
        match AppConfig::config_path(app) {
            Ok(p) => {
                let cfg = AppConfig::load(&p);
                *self.config.lock() = cfg;
            }
            Err(e) => log::warn!("无法解析 config path: {e}"),
        }
    }

    pub fn save_config(&self, app: &tauri::AppHandle) -> anyhow::Result<()> {
        let p = AppConfig::config_path(app)?;
        let cfg = self.config.lock().clone();
        cfg.save(&p)
    }

    pub fn paths(&self) -> AppPaths {
        self.config.lock().paths.clone()
    }

    pub fn poll_process_snapshot(&self) -> StatusSnapshot {
        let names: Vec<String> = vec![
            "redis-server.exe".into(),
            "node.exe".into(),
            "NapCatWinBootMain.exe".into(),
            "QQ.exe".into(),
        ];

        let mut snap = StatusSnapshot::default();
        snap.redis.running = self.proc.is_running("redis-server.exe");
        snap.redis.pid = self.proc.find_pid("redis-server.exe");

        // Yunzai 用 nvm node 启动,进程名是 node.exe,但要排除 QQ.exe / redis / 我们自己
        snap.yunzai.running = self
            .proc
            .is_running_with_cmdline("node.exe", "app.js");
        snap.yunzai.pid = self.proc.find_pid_with_cmdline("node.exe", "app.js");

        snap.napcat.running = self.proc.is_running("NapCatWinBootMain.exe");
        snap.napcat.pid = self.proc.find_pid("NapCatWinBootMain.exe");

        snap.qq.running = self.proc.is_running("QQ.exe");
        snap.qq.pid = self.proc.find_pid("QQ.exe");

        // WebUI 在 6099 端口 (NapCat)
        snap.webui_connected = check_port_listening(6099);
        // Yunzai 连 NapCat 通过 2536 端口的 WS
        // 这里简化: 当 Yunzai 和 NapCat 都在跑时,假设 WS 已连 (实际可能有 Race)
        snap.websocket_connected = check_port_listening(2536);

        snap.timestamp = chrono::Local::now().timestamp_millis();
        let _ = names; // 防止未使用警告
        snap
    }
}

fn check_port_listening(port: u16) -> bool {
    #[cfg(windows)]
    {
        use std::net::Ipv4Addr;
        use std::net::TcpStream;
        use std::time::Duration;
        TcpStream::connect_timeout(
            &(Ipv4Addr::LOCALHOST, port).into(),
            Duration::from_millis(150),
        )
        .is_ok()
    }
    #[cfg(not(windows))]
    {
        false
    }
}
