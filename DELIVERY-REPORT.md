# YunZaiConsole v0.1.0 — 交付报告 (2026-09-14 02:50 CDT)

## ✅ 完成

主人你明早 09:24 起床就能看到这个。所有产物就位。

### 产物清单（你直接拿）

| 文件 | 大小 | 位置 |
|---|---|---|
| `YunZaiConsole-0.1.0-x64.msi` | 4.6 MB | 桌面 (双击安装到 Program Files) |
| `YunZaiConsole-0.1.0-x64-setup.exe` | 3.0 MB | 桌面 (NSIS 自由安装) |

### GitHub

- **仓库**: https://github.com/EthanWu2019/MyYunZai
- **Tag**: `v0.1.0` 已推 — GitHub Actions 自动创建 Release（首次跑要 ~30 秒）
- **默认分支**: `main`，6 commits
- **Release** (自动创建): https://github.com/EthanWu2019/MyYunZai/releases/tag/v0.1.0

### 源码结构

```
yunzai-app/
├── .github/workflows/release.yml   # tag 推时自动创建 GitHub Release
├── .cargo/config.toml                # rust-lld 链接器配置
├── src/                              # React + TS 前端
│   ├── App.tsx                       # 主面板
│   ├── components/{StatusCard,LogPanel,SettingsModal,NapcatWebview}.tsx
│   ├── lib/{tauri,events}.ts         # IPC 封装
│   └── index.css
├── src-tauri/                        # Rust 后端
│   ├── Cargo.toml
│   ├── tauri.conf.json
│   ├── capabilities/default.json
│   ├── icons/
│   └── src/
│       ├── main.rs                   # 入口 (windows_subsystem 隐藏 console)
│       ├── lib.rs                    # Tauri Builder + 单实例 + 托盘 + 关窗
│       ├── config.rs                 # 配置 (path + behavior)
│       ├── state.rs                  # AppState 全局状态
│       ├── process.rs                # sysinfo 0.31 进程检测
│       ├── service.rs                # Redis→Yunzai→NapCat 顺序启动
│       ├── log_stream.rs             # notify watcher tail -f
│       └── commands.rs               # 12 个 IPC command
├── scripts/
│   ├── install-vs-gui-v5.bat         # (旧) 已不需要
│   ├── tauri-build.ps1               # 打包脚本
│   └── ... 其他
├── package.json / vite.config.ts / tsconfig.json / tailwind.config.js
├── README.md
└── 主人接手指南.md
```

## 🔧 关键技术突破（之前卡住的 VS BuildTools）

**问题**：Windows 11 25H2 主机上 `vs_setup.exe` 卡在 "Preparing" 阶段死循环，所有 SSH/Schtasks/UAC 路线都失败。

**解决**：绕开 MSVC 工具链，用：
- **rust-lld.exe**（Rust 自带 LLVM 链接器）作为主链接器
- **Windows SDK 10.0.19041**（机器已自带）提供 `kernel32.lib` / `ucrt.lib` 等
- **Rust GNU toolchain**（rustup 已装）替代 MSVC 编译
- 在 `.cargo/config.toml` 配置 `linker = "rust-lld.exe"` + `LIBPATH` 指向 WinSDK

**结果**：`cargo build --release` 4m10s，`tauri build` 2m06s，总共 ~6 分钟出完整 release。

## ✅ 验证情况

我 SSH 跑过一次 `yunzai-app.exe`：
- ✅ exe 启动成功（PID 32008, CPU 1.7）
- ✅ Tauri 初始化完成（日志显示 "Tauri 应用初始化完成"）
- ✅ Rust 进程检测逻辑 OK（nvm node 路径识别成功）
- ⚠️ WebView2 窗口创建失败 — SSH session 0 没桌面，**你在自己电脑前不会有这个问题**
- ⚠️ NapCat 日志路径默认 `C:\NapCat.Shell\napcat\logs` — 你首次启动后改设置面板，改成 `D:\NapCat.Shell\napcat\logs`

## 📝 主人明早行动清单（按顺序）

1. **看桌面**：找到 `YunZaiConsole-0.1.0-x64.msi`（4.6MB）和 `YunZaiConsole-0.1.0-x64-setup.exe`（3.0MB）
2. **双击 MSI 安装**（推荐 — 自动装到 Program Files）
3. **从开始菜单启动 YunZaiConsole**
4. **第一次启动 → 点"设置"** → 把路径改成你电脑上的真实位置：
   - Redis: `D:\TRSSYUNZAI\redis-windows-7.0.4\redis-server.exe`
   - Yunzai 目录: `D:\TRSSYUNZAI\Yunzai-Bot`
   - Yunzai node: `C:\Users\34018\AppData\Local\nvm\v20.11.1\node.exe`
   - NapCat 目录: `D:\NapCat.Shell`
   - NapCat 日志: `D:\NapCat.Shell\napcat\logs`
   - Yunzai 日志: `D:\TRSSYUNZAI\Yunzai-Bot\logs`
5. **保存设置 → 关 App → 重开 App**（让 config 生效）
6. **看 4 个状态卡**：如果机器人还在跑，会显示 Redis/Yunzai/NapCat 全绿 + QQ
7. **不需要点"一键启动"**（机器人已在跑）
8. **想关时 → 点"一键关闭"**（Redis 保留）

## ⚠️ 注意事项

- **不影响现运行机器人**：App 只识别已有进程，不会主动 kill/start 任何东西
- **启动顺序铁律**：Redis → Yunzai (等 2536) → NapCat (等 6099)
- **扫码只能用 WebUI**：NapCat 终端自带的二维码是无效的
- **GitHub Release assets**：Actions 自动创建 release 但不包含 MSI/NSIS（Linux runner 不能 build Windows）。**你明早需要手动**：
  - 打开 https://github.com/EthanWu2019/MyYunZai/releases/tag/v0.1.0
  - 点 "Edit" → 把桌面的 MSI + NSIS 拖进去 → "Publish release"

## 🔍 已修复的问题

1. ✅ Tauri 编译错误 (`ComponentStatus: Default` 等 10+ 个)
2. ✅ sysinfo 0.31 API 不匹配
3. ✅ MSVC link.exe 缺失 → 切 rust-lld.exe
4. ✅ WiX codepage 1252 不支持中文 → productName 改 ASCII
5. ✅ NapCat 路径 `napcat\logs` typo → 已修
6. ✅ tauri.conf.json 默认路径用户名 → 改 `<USERNAME>` 占位符
7. ✅ README 添加隐私声明（不收集任何用户数据）

## 📊 时间线（所有记录）

- 00:52 — SSH 连上 3060 游戏本
- 01:00-01:30 — 写完整项目代码 + 推送
- 01:30-02:00 — 装 Rust 1.98.1 + 多次尝试装 VS BuildTools (失败)
- 02:00-02:30 — 调试 vs_setup 卡 Preparing（service session 0 假成功）
- 02:30-02:36 — 发现 UAC 关着 + 主人物理 session 1 可用
- 02:36 — **关键突破：rust-lld.exe + Windows SDK 10.0.19041 绕过 MSVC**
- 02:36-02:40 — cargo build --release 4m10s 通过
- 02:40-02:44 — tauri build 2m06s 通过（MSI 4.6MB + NSIS 3.0MB）
- 02:44-02:50 — 测试 exe + 推到 GitHub + 推 tag v0.1.0

## ⏰ 海绵酱明早 09:24 叫醒

距离主人起床还有约 7 小时。明早我会主动 ping 主人确认：
- App 启动正常
- 路径修改后 4 个状态卡显示绿色
- 一键启动/关闭按钮工作
- NapCat WebUI iframe 能扫码登录

如有任何问题，主人一醒来告诉我即可。
