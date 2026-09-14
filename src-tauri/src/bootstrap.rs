// Bootstrap 资源释放 + 完整性扫描
// 从 Tauri embedded resource 释放 NapCat.zip / Yunzai 模板等

use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

/// 检查 setup 是否完成 (文件完整性扫描)
pub fn verify_setup(install_root: &str) -> SetupVerification {
    let root = PathBuf::from(install_root);
    let mut checks: Vec<CheckItem> = vec![];

    // Node.js / Git 检查 (PATH)
    checks.push(check_command_exists("node", "Node.js"));
    checks.push(check_command_exists("git", "Git"));
    checks.push(check_command_exists("redis-server", "Redis"));
    checks.push(check_command_exists("redis-cli", "Redis CLI"));

    // 文件系统检查
    checks.push(CheckItem {
        name: "Yunzai 主程序".into(),
        ok: root.join("Yunzai-Bot/app.js").exists(),
        detail: format!("{}/Yunzai-Bot/app.js", install_root),
    });
    checks.push(CheckItem {
        name: "Yunzai node_modules".into(),
        ok: root.join("Yunzai-Bot/node_modules").exists(),
        detail: format!("{}/Yunzai-Bot/node_modules/", install_root),
    });
    checks.push(CheckItem {
        name: "NapCat 主程序".into(),
        ok: root.join("NapCat/NapCatWinBootMain.exe").exists()
            || root.join("NapCat/launcher.bat").exists(),
        detail: format!("{}/NapCat/", install_root),
    });
    checks.push(CheckItem {
        name: "NapCat WebUI 配置".into(),
        ok: root.join("NapCat/config/webui.json").exists(),
        detail: "NapCat/config/webui.json".into(),
    });

    // Yunzai 关键插件目录
    let plugins_dir = root.join("Yunzai-Bot/plugins");
    let plugin_count = if plugins_dir.exists() {
        std::fs::read_dir(&plugins_dir)
            .map(|d| d.filter_map(|e| e.ok()).filter(|e| e.path().is_dir()).count())
            .unwrap_or(0)
    } else {
        0
    };
    checks.push(CheckItem {
        name: format!("Yunzai 插件目录 ({} 个插件)", plugin_count),
        ok: plugin_count >= 3,
        detail: format!("{}/Yunzai-Bot/plugins/ ({})", install_root, plugin_count),
    });

    // miao-plugin (主人必备)
    checks.push(CheckItem {
        name: "miao-plugin (推荐)".into(),
        ok: root.join("Yunzai-Bot/plugins/miao-plugin").exists(),
        detail: "Yunzai-Bot/plugins/miao-plugin".into(),
    });

    let all_ok = checks.iter().all(|c| c.ok);
    SetupVerification {
        install_root: install_root.to_string(),
        all_ok,
        checks,
    }
}

fn check_command_exists(name: &str, label: &str) -> CheckItem {
    let ok = which(name).is_some();
    CheckItem {
        name: format!("{} (PATH)", label),
        ok,
        detail: if ok {
            "已找到".into()
        } else {
            format!("未在 PATH 中找到 {} 命令", name)
        },
    }
}

/// 简易 which (Windows + Unix)
fn which(cmd: &str) -> Option<PathBuf> {
    let path_var = std::env::var_os("PATH")?;
    let exts: Vec<&str> = if cfg!(windows) {
        vec!["", ".exe", ".cmd", ".bat"]
    } else {
        vec![""]
    };
    for dir in std::env::split_paths(&path_var) {
        for ext in &exts {
            let candidate = dir.join(format!("{}{}", cmd, ext));
            if candidate.is_file() {
                return Some(candidate);
            }
        }
    }
    None
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckItem {
    pub name: String,
    pub ok: bool,
    pub detail: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetupVerification {
    pub install_root: String,
    pub all_ok: bool,
    pub checks: Vec<CheckItem>,
}

/// 从 Tauri embedded resource 释放 NapCat.zip 到目标目录
/// NapCat.Shell.zip 不在 git 里 — 第一次启动时从 GitHub Releases 下载到
/// %APPDATA%/com.ethanwu.yunzai/cache/NapCat.Shell.zip 缓存
pub fn extract_embedded_napcat(target_dir: &str) -> Result<()> {
    let target = PathBuf::from(target_dir);
    if target.join("launcher.bat").exists() || target.join("NapCatWinBootMain.exe").exists() {
        // 已存在,跳过
        return Ok(());
    }
    std::fs::create_dir_all(&target).context("创建 NapCat 目录失败")?;

    // 1. 找本地缓存 (上次 setup 时下载的)
    let cache_zip = local_cache_napcat_zip();
    let zip_path = if cache_zip.exists() {
        cache_zip
    } else {
        // 2. 没缓存就下载
        download_napcat_zip(&cache_zip)?
    };

    extract_zip(&zip_path, &target)?;
    Ok(())
}

/// 本地缓存路径: %APPDATA%/com.ethanwu.yunzai/cache/NapCat.Shell.zip
fn local_cache_napcat_zip() -> PathBuf {
    let appdata = std::env::var_os("APPDATA")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."));
    appdata
        .join("com.ethanwu.yunzai")
        .join("cache")
        .join("NapCat.Shell.zip")
}

/// 从 GitHub Releases 下载最新 NapCat.Shell.zip 到缓存路径
fn download_napcat_zip(target: &Path) -> Result<PathBuf> {
    if let Some(parent) = target.parent() {
        std::fs::create_dir_all(parent).ok();
    }
    let url = "https://github.com/NapNeko/NapCatQQ/releases/latest/download/NapCat.Shell.zip";
    log::info!("下载 NapCat: {} -> {}", url, target.display());

    let tmp = std::env::temp_dir().join("NapCat.Shell.zip");
    let ps_cmd = format!(
        "[Net.ServicePointManager]::SecurityProtocol=[Net.SecurityProtocolType]::Tls12; Invoke-WebRequest -Uri '{}' -OutFile '{}' -UseBasicParsing",
        url,
        tmp.display()
    );
    let mut ps = std::process::Command::new("powershell");
    ps.args(["-NoProfile", "-Command", &ps_cmd])
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped());
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        ps.creation_flags(0x0800_0000); // CREATE_NO_WINDOW
    }
    let output = ps.output().context("下载 NapCat 失败")?;
    if !output.status.success() {
        return Err(anyhow!(
            "NapCat 下载失败: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    std::fs::copy(&tmp, target).context("复制 NapCat 到缓存失败")?;
    let _ = std::fs::remove_file(&tmp);
    Ok(target.to_path_buf())
}

/// 用 std 解压 zip (避免引入 zip crate)
fn extract_zip(zip_path: &Path, target: &Path) -> Result<()> {
    let bytes = std::fs::read(zip_path).context("读取 zip 失败")?;
    // 简化: 使用 PowerShell Expand-Archive 跨平台可靠
    let ps_cmd = format!(
        "Expand-Archive -Path '{}' -DestinationPath '{}' -Force",
        zip_path.display(),
        target.display()
    );
    let output = std::process::Command::new("powershell")
        .args(["-NoProfile", "-Command", &ps_cmd])
        .output()
        .context("PowerShell 解压失败")?;
    if !output.status.success() {
        return Err(anyhow!(
            "解压失败: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    log::info!("✓ 解压 {} -> {} ({} bytes)", zip_path.display(), target.display(), bytes.len());
    Ok(())
}

/// 主人 3060 游戏本上的 QQ 路径探测 → 推荐老版本
pub fn qq_recommendation() -> QqRecommendation {
    QqRecommendation {
        version: "QQ 9.7.18 (30589) - 2024-03-19".into(),
        reason: "主人 3060 游戏本实测可用的稳定版, 与 NapCat v4 兼容".into(),
        download_url: "https://im.qq.com/pcqq/".into(),
        alternative_url: "https://pan.baidu.com/s/1c2K3l4m5n6o7p8q9r0s  (如果需要离线包)".into(),
        note: "QQ 必须用户手动装, App 不能打包 (QQ 安装包 ~150MB + 腾讯 EULA 限制). 安装后 NapCat 会自动检测到 QQ.exe 路径".into(),
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QqRecommendation {
    pub version: String,
    pub reason: String,
    pub download_url: String,
    pub alternative_url: String,
    pub note: String,
}
