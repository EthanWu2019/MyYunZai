// 服务管理: Redis → Yunzai → NapCat 启动链 (严格顺序)

use crate::state::AppState;
use anyhow::{anyhow, Result};
use std::path::Path;
use std::process::{Child, Command, Stdio};

#[cfg(windows)]
use std::os::windows::process::CommandExt;

/// Windows 创建进程标志: CREATE_NO_WINDOW (隐藏 cmd 弹窗)
#[cfg(windows)]
const CREATE_NO_WINDOW: u32 = 0x0800_0000;

/// 检查 Redis 是否已监听 6379 (说明已经在跑)
fn is_redis_already_running(_state: &AppState) -> bool {
    use std::net::Ipv4Addr;
    use std::net::TcpStream;
    use std::time::Duration;
    TcpStream::connect_timeout(
        &(Ipv4Addr::LOCALHOST, 6379).into(),
        Duration::from_millis(200),
    )
    .is_ok()
}

/// 启动 Redis (后台,守护进程已经在 redis.conf 里 daemonize yes)
pub fn start_redis(state: &AppState) -> Result<()> {
    if is_redis_already_running(state) {
        log::info!("Redis 已在运行,跳过启动");
        return Ok(());
    }
    let paths = state.paths();
    let redis_exe = Path::new(&paths.redis_exe);
    let redis_conf = Path::new(&paths.redis_conf);

    if !redis_exe.exists() {
        return Err(anyhow!("redis-server.exe 不存在: {}", paths.redis_exe));
    }
    if !redis_conf.exists() {
        return Err(anyhow!("redis.conf 不存在: {}", paths.redis_conf));
    }

    // 删掉旧的 pid 文件 (防 Redis 拒绝启动)
    let pid_file = redis_exe.parent().map(|p| p.join("redis.pid"));
    if let Some(pf) = &pid_file {
        if pf.exists() {
            let _ = std::fs::remove_file(pf);
            log::info!("已清理旧 redis.pid");
        }
    }

    let mut cmd = Command::new(redis_exe);
    cmd.arg(redis_conf)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null());
    #[cfg(windows)]
    cmd.creation_flags(CREATE_NO_WINDOW);

    let child: Child = cmd.spawn().map_err(|e| anyhow!("启动 Redis 失败: {e}"))?;
    let pid = child.id();
    state.children_pid.lock().redis = Some(pid);
    log::info!("✅ Redis 已启动 (PID={})", pid);

    // 等待 Redis 6379 端口监听
    wait_port_listening(6379, 5_000)?;
    log::info!("Redis 端口 6379 已就绪");
    Ok(())
}

/// 启动 Yunzai 后端 (用 nvm 管理的 node)
pub fn start_yunzai(state: &AppState) -> Result<()> {
    let paths = state.paths();
    let node_exe = Path::new(&paths.yunzai_node);
    let yunzai_dir = Path::new(&paths.yunzai_dir);

    if !node_exe.exists() {
        return Err(anyhow!(
            "node.exe 不存在: {} (检查 nvm 路径或配置)",
            paths.yunzai_node
        ));
    }
    if !yunzai_dir.exists() {
        return Err(anyhow!("Yunzai 目录不存在: {}", paths.yunzai_dir));
    }
    if !yunzai_dir.join("app.js").exists() {
        return Err(anyhow!(
            "Yunzai 入口 app.js 不存在,目录不对: {}",
            paths.yunzai_dir
        ));
    }

    // Windows 下 node 必须从 Yunzai 目录启动 (否则找不到 node_modules 路径)
    let mut cmd = Command::new(node_exe);
    cmd.current_dir(yunzai_dir)
        .arg("app.js")
        .env("FORCE_COLOR", "0")
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null());
    #[cfg(windows)]
    cmd.creation_flags(CREATE_NO_WINDOW);

    let child = cmd.spawn().map_err(|e| anyhow!("启动 Yunzai 失败: {e}"))?;
    let pid = child.id();
    state.children_pid.lock().yunzai_node = Some(pid);
    log::info!("✅ Yunzai 已启动 (PID={} node={})", pid, paths.yunzai_node);

    // 等待 Yunzai 真正起来 (通过端口 2536 listen)
    wait_port_listening(2536, 15_000).map_err(|e| {
        anyhow!(
            "Yunzai 启动后未在 2536 端口监听 (检查 logs/command.{}.log): {e}",
            chrono::Local::now().format("%Y-%m-%d")
        )
    })?;
    log::info!("Yunzai WebSocket 端口 2536 已就绪 (等待 NapCat)");
    Ok(())
}

/// 启动 NapCat (走 launcher.bat,因为 bat 内部要从注册表读 QQ 路径并注入 Hook)
pub fn start_napcat(state: &AppState) -> Result<()> {
    let paths = state.paths();
    let bat = Path::new(&paths.napcat_dir).join("launcher.bat");
    if !bat.exists() {
        return Err(anyhow!(
            "NapCat launcher.bat 不存在: {}",
            bat.display()
        ));
    }

    // 走 cmd.exe 调 launcher.bat(才能让 bat 内部的 set / if / goto 生效)
    let mut cmd = Command::new("cmd.exe");
    cmd.current_dir(&paths.napcat_dir)
        .arg("/c")
        .arg(&bat)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null());
    #[cfg(windows)]
    cmd.creation_flags(CREATE_NO_WINDOW);

    let child = cmd.spawn().map_err(|e| anyhow!("启动 NapCat 失败: {e}"))?;
    let pid = child.id();
    state.children_pid.lock().napcat_main = Some(pid);
    log::info!("✅ NapCat launcher.bat 已启动 (PID={})", pid);

    // 等待 NapCat WebUI 端口 6099 listen
    wait_port_listening(6099, 30_000).map_err(|e| {
        anyhow!("NapCat 启动后未在 6099 端口监听 (WebUI 准备超时): {e}")
    })?;
    log::info!("NapCat WebUI 端口 6099 已就绪");

    // 注意: NapCat 的主进程是 NapCatWinBootMain.exe, cmd.exe 拉起后会退出
    // 所以真正应该记录 NapCatWinBootMain.exe 的 PID
    std::thread::sleep(std::time::Duration::from_millis(500));
    if let Some(nc_pid) = state.proc.find_pid("NapCatWinBootMain.exe") {
        state.children_pid.lock().napcat_main = Some(nc_pid);
        log::info!("检测到 NapCatWinBootMain.exe (PID={})", nc_pid);
    }
    Ok(())
}

/// 顺序启动: Redis → Yunzai (等 2536) → NapCat (等 6099)
pub fn start_all(state: &AppState) -> Result<()> {
    log::info!("======== 开始启动 Yunzai 链 ========");
    start_redis(state)?;
    start_yunzai(state)?;
    start_napcat(state)?;
    log::info!("======== Yunzai 链启动完成 ========");
    Ok(())
}

/// 关闭 Yunzai + NapCat (Redis 保留)
pub fn stop_bot(state: &AppState) -> Result<()> {
    log::info!("======== 开始关闭 Yunzai + NapCat (Redis 保留) ========");

    // 1. 杀 NapCat 主进程 + QQ 实例 (按 cmd 子树: NapCatWinBootMain + 它拉起的 QQ.exe)
    let napcat_pid = state.children_pid.lock().napcat_main;
    if let Some(pid) = napcat_pid {
        if state.proc.kill_pid(pid) {
            log::info!("已杀 NapCatWinBootMain (PID={})", pid);
        }
    }
    // 兜底:按进程名扫一遍
    let killed_nc = state.proc.kill_by_name("NapCatWinBootMain.exe");
    let killed_qq = state.proc.kill_by_name("QQ.exe");
    log::info!("兜底杀: NapCatWinBootMain={} 个, QQ.exe={} 个", killed_nc, killed_qq);

    // 2. 杀 Yunzai (node.exe 带 app.js cmdline 的)
    let yunzai_pid = state.children_pid.lock().yunzai_node;
    if let Some(pid) = yunzai_pid {
        if state.proc.kill_pid(pid) {
            log::info!("已杀 Yunzai node (PID={})", pid);
        }
    }
    // 兜底:扫所有带 app.js 的 node.exe
    self_referential_kill_yunzai(state);

    // 3. Redis 不杀
    log::info!("Redis 保留运行 (主人偏好)");

    state.children_pid.lock().redis = None;
    state.children_pid.lock().yunzai_node = None;
    state.children_pid.lock().napcat_main = None;

    log::info!("======== Yunzai + NapCat 已关闭 ========");
    Ok(())
}

fn self_referential_kill_yunzai(state: &AppState) {
    state.proc.snapshot(|sys| {
        for (pid, proc_) in sys.processes() {
            let cmd: String = proc_
                .cmd()
                .join(std::ffi::OsStr::new(" "))
                .to_string_lossy()
                .into_owned();
            if cmd.to_lowercase().contains("app.js") {
                log::info!("兜底杀 Yunzai (PID={}, cmd={})", pid, cmd);
                proc_.kill();
            }
        }
    });
}

/// 等待本地端口可连接
fn wait_port_listening(port: u16, timeout_ms: u64) -> Result<()> {
    use std::net::Ipv4Addr;
    use std::net::TcpStream;
    use std::time::{Duration, Instant};

    let deadline = Instant::now() + Duration::from_millis(timeout_ms);
    let addr = (Ipv4Addr::LOCALHOST, port).into();

    while Instant::now() < deadline {
        if TcpStream::connect_timeout(&addr, Duration::from_millis(150)).is_ok() {
            return Ok(());
        }
        std::thread::sleep(Duration::from_millis(200));
    }
    Err(anyhow!("端口 {port} 在 {timeout_ms}ms 内未监听"))
}
