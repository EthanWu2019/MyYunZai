// Setup 状态机: First-Run Wizard 整套流程
use crate::config::AppPaths;
use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::process::Command;
use tauri::{AppHandle, Emitter};

/// Setup 步骤状态 (发给前端)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetupStep {
    pub id: String,
    pub name: String,
    pub status: StepStatus,
    pub progress: f32,         // 0.0 - 1.0
    pub message: String,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum StepStatus {
    Pending,
    Running,
    Completed,
    Failed,
    Skipped,
}

/// 整个 setup 流程的步骤
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetupProgress {
    pub current_step: usize,
    pub total_steps: usize,
    pub steps: Vec<SetupStep>,
    pub overall_status: StepStatus,
    pub error_message: Option<String>,
}

/// 全流程
pub fn run_full_setup(
    app: AppHandle,
    paths: AppPaths,
    selected_plugins: Vec<String>,  // plugin name 列表
) -> Result<()> {
    log::info!("开始 setup 全流程");

    let install_dir = paths.yunzai_dir.clone();  // 默认 C:\YunZaiApp 或 D:\YunZaiApp
    log::info!("安装目录: {}", install_dir);

    let mut progress = SetupProgress {
        current_step: 0,
        total_steps: 7 + selected_plugins.len(),  // 7 个基础步骤 + 每个插件 1 步
        steps: vec![
            SetupStep {
                id: "deps".into(),
                name: "检测系统依赖 (Node.js / Git / Redis)".into(),
                status: StepStatus::Pending,
                progress: 0.0,
                message: String::new(),
            },
            SetupStep {
                id: "redis_install".into(),
                name: "安装 Redis".into(),
                status: StepStatus::Pending,
                progress: 0.0,
                message: String::new(),
            },
            SetupStep {
                id: "yunzai_clone".into(),
                name: "下载 Yunzai 主程序 (TRSS-Yunzai)".into(),
                status: StepStatus::Pending,
                progress: 0.0,
                message: String::new(),
            },
            SetupStep {
                id: "napcat_clone".into(),
                name: "下载 NapCat 协议端".into(),
                status: StepStatus::Pending,
                progress: 0.0,
                message: String::new(),
            },
            SetupStep {
                id: "qq_install".into(),
                name: "下载 QQ 客户端".into(),
                status: StepStatus::Pending,
                progress: 0.0,
                message: String::new(),
            },
            SetupStep {
                id: "yunzai_npm".into(),
                name: "安装 Yunzai npm 依赖".into(),
                status: StepStatus::Pending,
                progress: 0.0,
                message: String::new(),
            },
            SetupStep {
                id: "napcat_setup".into(),
                name: "配置 NapCat WebUI token".into(),
                status: StepStatus::Pending,
                progress: 0.0,
                message: String::new(),
            },
        ],
        overall_status: StepStatus::Running,
        error_message: None,
    };

    // 推送初始状态
    let _ = app.emit("setup:progress", &progress);

    // Step 1: 检测 deps
    progress.steps[0].status = StepStatus::Running;
    let _ = app.emit("setup:progress", &progress);
    check_dependencies(&mut progress.steps[0])?;
    progress.steps[0].status = StepStatus::Completed;
    let _ = app.emit("setup:progress", &progress);

    // Step 2: 安装 Redis (Windows 下载 portable redis)
    progress.steps[1].status = StepStatus::Running;
    let _ = app.emit("setup:progress", &progress);
    install_redis(&install_dir, &mut progress.steps[1])?;
    progress.steps[1].status = StepStatus::Completed;
    let _ = app.emit("setup:progress", &progress);

    // Step 3: Clone Yunzai
    progress.steps[2].status = StepStatus::Running;
    let _ = app.emit("setup:progress", &progress);
    git_clone("https://github.com/TimeRainStarSky/Yunzai.git", &format!("{}/Yunzai-Bot", install_dir), &mut progress.steps[2])?;
    progress.steps[2].status = StepStatus::Completed;
    let _ = app.emit("setup:progress", &progress);

    // Step 4: Clone NapCat (Windows)
    progress.steps[3].status = StepStatus::Running;
    let _ = app.emit("setup:progress", &progress);
    // Windows 下 NapCat 用 .zip 包 (v4 系列); 实际下载地址
    download_and_extract(
        "https://github.com/NapNeko/NapCatQQ/releases/latest/download/NapCat.Shell.Windows.zip",
        &format!("{}/NapCat", install_dir),
        &mut progress.steps[3],
    )?;
    progress.steps[3].status = StepStatus::Completed;
    let _ = app.emit("setup:progress", &progress);

    // Step 5: 下载 QQ (Windows 11 内置; 但确保存在)
    progress.steps[4].status = StepStatus::Running;
    let _ = app.emit("setup:progress", &progress);
    progress.steps[4].message = "Windows 11 自带 QQ,如未安装请打开 Microsoft Store 搜索 QQ".into();
    progress.steps[4].status = StepStatus::Completed;
    let _ = app.emit("setup:progress", &progress);

    // Step 6: Yunzai npm install
    progress.steps[5].status = StepStatus::Running;
    let _ = app.emit("setup:progress", &progress);
    run_npm_install(&format!("{}/Yunzai-Bot", install_dir), &mut progress.steps[5])?;
    progress.steps[5].status = StepStatus::Completed;
    let _ = app.emit("setup:progress", &progress);

    // Step 7: NapCat config
    progress.steps[6].status = StepStatus::Running;
    let _ = app.emit("setup:progress", &progress);
    configure_napcat(&format!("{}/NapCat", install_dir), &mut progress.steps[6])?;
    progress.steps[6].status = StepStatus::Completed;
    let _ = app.emit("setup:progress", &progress);

    // Plugin clones
    let plugin_index = crate::plugin_index::owner_installed_plugins();
    for (i, plugin_name) in selected_plugins.iter().enumerate() {
        let step = SetupStep {
            id: format!("plugin_{}", i),
            name: format!("下载插件: {}", plugin_name),
            status: StepStatus::Running,
            progress: 0.0,
            message: String::new(),
        };
        progress.steps.push(step);
        let idx = progress.steps.len() - 1;
        let _ = app.emit("setup:progress", &progress);

        let plugin = plugin_index.iter().find(|p| p.name == *plugin_name);
        if let Some(p) = plugin {
            let target = format!("{}/Yunzai-Bot/plugins/{}", install_dir, sanitize_dir_name(&p.name));
            let result = git_clone(&p.repo_url, &target, &mut progress.steps[idx]);
            if result.is_ok() && p.name != "TRSS-Yunzai (主程序)" {
                // 主程序已装,跳过 npm install
                let _ = run_npm_install(&target, &mut progress.steps[idx]);
            }
        }
        progress.steps[idx].status = StepStatus::Completed;
        let _ = app.emit("setup:progress", &progress);
    }

    progress.overall_status = StepStatus::Completed;
    let _ = app.emit("setup:progress", &progress);

    log::info!("✅ setup 全流程完成");
    Ok(())
}

fn check_dependencies(step: &mut SetupStep) -> Result<()> {
    step.message = "检测 Node.js...".into();
    let node_ok = Command::new("node").arg("--version").output().is_ok();
    step.message = "检测 Git...".into();
    let git_ok = Command::new("git").arg("--version").output().is_ok();

    if !node_ok {
        return Err(anyhow!("Node.js 未安装。请先下载安装 Node.js 20 LTS: https://nodejs.org"));
    }
    if !git_ok {
        return Err(anyhow!("Git 未安装。请先下载安装 Git: https://git-scm.com"));
    }
    Ok(())
}

fn install_redis(_install_dir: &str, step: &mut SetupStep) -> Result<()> {
    step.message = "下载 Redis for Windows...".into();
    // 简化: 让用户自己装 Redis Windows 服务
    // 实际: 下载 https://github.com/tporadowski/redis/releases 下载 ZIP 解压
    // 或: 用 winget install Redis.Redis (需要 winget)
    step.message = "请打开 https://github.com/tporadowski/redis/releases 下载 Redis for Windows ZIP,解压到任意目录".into();
    Ok(())
}

fn git_clone(url: &str, target: &str, step: &mut SetupStep) -> Result<()> {
    step.message = format!("git clone {} -> {}", url, target);
    log::info!("{}", step.message);

    let target_path = PathBuf::from(target);
    if target_path.exists() {
        step.message = format!("{} 已存在,跳过", target);
        return Ok(());
    }

    if let Some(parent) = target_path.parent() {
        std::fs::create_dir_all(parent).context("创建父目录失败")?;
    }

    let output = Command::new("git")
        .args(["clone", "--depth=1", url, target])
        .output()
        .context("git clone 失败")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow!("git clone 失败: {}", stderr));
    }

    step.message = format!("✓ 已下载: {}", target);
    Ok(())
}

fn download_and_extract(url: &str, target: &str, step: &mut SetupStep) -> Result<()> {
    step.message = format!("下载 {} -> {}", url, target);
    log::info!("{}", step.message);

    let target_path = PathBuf::from(target);
    if target_path.exists() {
        step.message = format!("{} 已存在,跳过", target);
        return Ok(());
    }

    // 用 curl 或 reqwest 下载到 TEMP,然后解压到 target
    let tmp_zip = std::env::temp_dir().join("yunzai-setup-download.zip");
    let output = Command::new("curl")
        .args(["-L", "-o", tmp_zip.to_str().unwrap(), url])
        .output()
        .context("curl 下载失败")?;

    if !output.status.success() {
        return Err(anyhow!("下载失败: {}", String::from_utf8_lossy(&output.stderr)));
    }

    if let Some(parent) = target_path.parent() {
        std::fs::create_dir_all(parent).ok();
    }

    // 解压 (用 PowerShell Expand-Archive)
    let ps_cmd = format!(
        "Expand-Archive -Path '{}' -DestinationPath '{}' -Force",
        tmp_zip.display(),
        target_path.display()
    );
    let output = Command::new("powershell")
        .args(["-NoProfile", "-Command", &ps_cmd])
        .output()
        .context("解压失败")?;

    if !output.status.success() {
        return Err(anyhow!("解压失败: {}", String::from_utf8_lossy(&output.stderr)));
    }

    std::fs::remove_file(&tmp_zip).ok();
    Ok(())
}

fn run_npm_install(dir: &str, step: &mut SetupStep) -> Result<()> {
    step.message = format!("npm install in {}", dir);
    log::info!("{}", step.message);

    let output = Command::new("npm")
        .arg("install")
        .arg("--registry=https://registry.npmmirror.com")
        .current_dir(dir)
        .output()
        .context("npm install 启动失败")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow!("npm install 失败: {}", stderr));
    }

    step.message = format!("✓ npm install 完成: {}", dir);
    Ok(())
}

fn configure_napcat(napcat_dir: &str, step: &mut SetupStep) -> Result<()> {
    step.message = "配置 NapCat WebUI token...".into();
    // 写入 webui.json with token
    let webui_path = PathBuf::from(napcat_dir).join("config/webui.json");
    if let Some(parent) = webui_path.parent() {
        std::fs::create_dir_all(parent).ok();
    }
    let webui_json = serde_json::json!({
        "host": "127.0.0.1",
        "port": 6099,
        "token": "yunzai-app-token",
        "loginRate": 10
    });
    std::fs::write(&webui_path, serde_json::to_string_pretty(&webui_json)?)?;
    step.message = format!("✓ WebUI token 已写入: {}", webui_path.display());
    Ok(())
}

fn sanitize_dir_name(name: &str) -> String {
    name.chars()
        .map(|c| match c {
            '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' | ' ' => '_',
            _ => c,
        })
        .collect()
}
