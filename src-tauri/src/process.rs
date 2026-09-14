// 进程检测 / 杀进程 (sysinfo 0.31)

use parking_lot::Mutex;
use std::sync::Arc;
use sysinfo::{Pid, ProcessRefreshKind, RefreshKind, System};

pub struct ProcessService {
    sys: Arc<Mutex<System>>,
}

impl Default for ProcessService {
    fn default() -> Self {
        Self::new()
    }
}

impl ProcessService {
    pub fn new() -> Self {
        let sys = System::new_with_specifics(
            RefreshKind::new().with_processes(ProcessRefreshKind::everything()),
        );
        Self {
            sys: Arc::new(Mutex::new(sys)),
        }
    }

    fn refresh(&self) {
        let mut sys = self.sys.lock();
        sys.refresh_processes(sysinfo::ProcessesToUpdate::All);
    }

    pub fn is_running(&self, name: &str) -> bool {
        self.find_pid(name).is_some()
    }

    pub fn find_pid(&self, name: &str) -> Option<u32> {
        self.refresh();
        let sys = self.sys.lock();
        let target = name.to_ascii_lowercase();
        for (pid, proc_) in sys.processes() {
            let Some(exe) = proc_.exe() else { continue };
            let Some(fname) = exe.file_name().and_then(|s| s.to_str()) else {
                continue;
            };
            if fname.to_ascii_lowercase() == target {
                return Some(pid.as_u32());
            }
        }
        None
    }

    pub fn find_pid_with_cmdline(&self, name: &str, cmd_substr: &str) -> Option<u32> {
        self.refresh();
        let sys = self.sys.lock();
        let target = name.to_ascii_lowercase();
        let needle = cmd_substr.to_ascii_lowercase();
        for (pid, proc_) in sys.processes() {
            let Some(exe) = proc_.exe() else { continue };
            let Some(fname) = exe.file_name().and_then(|s| s.to_str()) else {
                continue;
            };
            if fname.to_ascii_lowercase() != target {
                continue;
            }
            let cmd = proc_.cmd().join(std::ffi::OsStr::new(" ")).to_string_lossy().to_ascii_lowercase();
            if cmd.contains(&needle) {
                return Some(pid.as_u32());
            }
        }
        None
    }

    pub fn is_running_with_cmdline(&self, name: &str, cmd_substr: &str) -> bool {
        self.find_pid_with_cmdline(name, cmd_substr).is_some()
    }

    /// 让 service.rs 直接拿 sys 锁做 cmdline 扫描 (兜底杀 Yunzai 用)
    pub fn snapshot<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&System) -> R,
    {
        self.refresh();
        let sys = self.sys.lock();
        f(&sys)
    }

    /// 按名字杀进程 (basename match)
    pub fn kill_by_name(&self, name: &str) -> usize {
        self.refresh();
        let target = name.to_ascii_lowercase();
        let sys = self.sys.lock();
        let mut killed = 0usize;
        for (pid, proc_) in sys.processes() {
            let Some(exe) = proc_.exe() else { continue };
            let Some(fname) = exe.file_name().and_then(|s| s.to_str()) else {
                continue;
            };
            if fname.to_ascii_lowercase() != target {
                continue;
            }
            if proc_.kill() {
                killed += 1;
                log::info!("已杀进程: {} (PID={})", fname, pid);
            }
        }
        killed
    }

    /// 按 PID 杀进程 (我们拉起的子进程)
    pub fn kill_pid(&self, pid: u32) -> bool {
        self.refresh();
        let sys = self.sys.lock();
        if let Some(p) = sys.process(Pid::from_u32(pid)) {
            p.kill()
        } else {
            false
        }
    }
}
