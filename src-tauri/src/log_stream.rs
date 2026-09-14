// 日志流: 用 notify watcher tail -f Yunzai / NapCat 日志,推送到前端

use crate::config::AppPaths;
use notify::{RecursiveMode, Watcher};
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::PathBuf;
use std::sync::mpsc::channel;
use std::time::Duration;
use tauri::{AppHandle, Emitter};

/// 启动后台线程监听 yunzai_log_dir + napcat_log_dir
pub fn spawn_watchers(app: AppHandle, paths: AppPaths) {
    // Yunzai 日志: logs/command.YYYY-MM-DD.log (每日轮转)
    let yunzai_dir = PathBuf::from(&paths.yunzai_log_dir);
    let napcat_dir = PathBuf::from(&paths.napcat_log_dir);

    let app1 = app.clone();
    std::thread::spawn(move || {
        if let Err(e) = watch_dir(&app1, "yunzai", &yunzai_dir) {
            log::warn!("yunzai 日志监听失败: {e}");
        }
    });

    std::thread::spawn(move || {
        if let Err(e) = watch_dir(&app, "napcat", &napcat_dir) {
            log::warn!("napcat 日志监听失败: {e}");
        }
    });
}

fn watch_dir(app: &AppHandle, tag: &str, dir: &std::path::Path) -> anyhow::Result<()> {
    if !dir.exists() {
        log::warn!("日志目录不存在,跳过监听: {}", dir.display());
        return Ok(());
    }
    let (tx, rx) = channel::<notify::Result<notify::Event>>();
    let mut watcher = notify::recommended_watcher(move |res| {
        let _ = tx.send(res);
    })?;
    watcher.watch(dir, RecursiveMode::NonRecursive)?;

    // 维护每个文件的读取位置
    let mut file_positions: std::collections::HashMap<PathBuf, u64> =
        std::collections::HashMap::new();

    loop {
        match rx.recv_timeout(Duration::from_secs(2)) {
            Ok(Ok(event)) => {
                for path in event.paths.iter().filter(|p| {
                    p.extension().map(|e| e == "log").unwrap_or(false)
                }) {
                    tail_file(app, tag, path, &mut file_positions);
                }
            }
            Ok(Err(e)) => log::warn!("[{tag}] 监听错误: {e}"),
            Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {
                // 周期扫一下,防止漏报 (notify 在某些场景会丢事件)
                if let Ok(entries) = std::fs::read_dir(dir) {
                    for entry in entries.flatten() {
                        let p = entry.path();
                        if p.extension().map(|e| e == "log").unwrap_or(false) {
                            tail_file(app, tag, &p, &mut file_positions);
                        }
                    }
                }
            }
            Err(e) => log::warn!("[{tag}] recv: {e}"),
        }
    }
}

fn tail_file(
    app: &AppHandle,
    tag: &str,
    path: &std::path::Path,
    positions: &mut std::collections::HashMap<PathBuf, u64>,
) {
    let Ok(mut f) = File::open(path) else { return };
    let Ok(len) = f.metadata().map(|m| m.len()) else { return };
    let last = positions.get(path).copied().unwrap_or(0);

    // 文件被截断/轮转了 → 从头读
    if len < last {
        positions.insert(path.to_path_buf(), 0);
    }

    let new_pos = positions.get(path).copied().unwrap_or(0);
    if len <= new_pos {
        return;
    }

    if f.seek(SeekFrom::Start(new_pos)).is_err() {
        return;
    }
    let mut buf = Vec::with_capacity((len - new_pos).min(64 * 1024) as usize);
    if f.read_to_end(&mut buf).is_err() {
        return;
    }
    positions.insert(path.to_path_buf(), len);

    let text = String::from_utf8_lossy(&buf);
    let file_name = path
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("unknown");
    let _ = app.emit(
        "log:append",
        LogEvent {
            source: tag.to_string(),
            file: file_name.to_string(),
            text: text.into_owned(),
        },
    );
}

#[derive(serde::Serialize, Clone)]
struct LogEvent {
    source: String,
    file: String,
    text: String,
}
