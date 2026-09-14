# 海绵酱控制台 (YunZai APP)

**GitHub**: [github.com/EthanWu2019/MyYunZai](https://github.com/EthanWu2019/MyYunZai)

把 **HYZL 启动器 + Redis + TRSS Yunzai 后端 + NapCat 协议端 + WebUI 扫码** 整合到一个 Tauri 2 桌面应用。

不再需要分别打开 HYZL.exe / launcher.bat / Edge 浏览器 —— 一个 App 全搞定。

## ⚠️ 重要：本 App 不是 Yunzai 全量包

**这是 launcher/console，不是 Yunzai 本身。**

主人每次只装一个 4.5 MB MSI / 5 MB DMG，就能控制整个 Yunzai 机器人，但**机器上必须先有**：
- TRSS Yunzai 后端 (`Yunzai-Bot/`)
- NapCat 协议端
- Redis
- Node.js（Yunzai 用）
- QQ 客户端（NapCat 注入用）

**为啥不打包 Yunzai 进去？**：
- Yunzai + 全部插件 = **3-5 GB**（node_modules + oicq + plugins + data）
- 主人已有装好的（HYZL.exe 那一套）
- 每个用户机器路径不同，打包一份给所有人用不现实
- Launcher 模式：跟你的 HYZL.exe 一样的思路（HYZL 也不是 Yunzai 本身）

**全量包适合**：自己写一个新机器人部署脚本 → 自己机器 OK
**Launcher 适合**：已有机器的快速控制 → 我的设计

如果要"全量打包给朋友用"，需要另写 `yunzai-full-pack` 工程（npm + oicq + 全部插件 + redis 源 + QQ 装脚本），不在本 repo 范围。

## ⚠️ 隐私与数据声明

**本项目不包含任何用户敏感信息**：
- ❌ **不收集/不上传** 任何机器人 token、QQ 账号、Discord token、API key
- ❌ **不修改** `D:\TRSSYUNZAI\`、`D:\NapCat.Shell\`、`E:\QQ\` 等任何原机器人文件
- ✅ 配置路径默认是占位符 (`C:\Users\<USERNAME>\...`)，用户首次启动后通过设置面板填写真实路径
- ✅ 所有配置存在 `%APPDATA%/com.ethanwu.yunzai/config.json`（**用户本地**，不上传 GitHub）

**主人在自己电脑上的使用**：
1. 第一次启动 App 后点"设置" → 把路径改成你电脑上的真实位置 → 保存
2. 默认 username 占位符 `<USERNAME>` 会提醒你改成自己的 Windows 用户名（一般是 `34018` 或 `Administrator`）

## 架构

```
[本 App (Tauri 2)]                          
  ├─ Rust 后端 (进程管理 + 配置 + 日志流)
  │   ├─ 启动链: Redis → Yunzai(nvm node) → NapCat(launcher.bat)
  │   ├─ 关闭链: Yunzai + NapCat (Redis 保留)
  │   ├─ 进程检测: sysinfo 0.31 扫描 PID + cmdline
  │   ├─ 日志流: notify watcher tail -f logs/ 推送到前端
  │   └─ 状态轮询: 2 秒一次 → status:update 事件
  │
  └─ React 前端 (Vite + TS + Tailwind + framer-motion)
      ├─ 概览页: 4 个状态卡 (Redis/Yunzai/NapCat/QQ) + 日志面板 + 一键启停
      ├─ WebUI 页: iframe 嵌 http://localhost:6099/webui/ 直接扫码
      └─ 设置页: 路径编辑 + 行为 (开机自启 / 关窗最小化 / 自动启动)

外部依赖 (本 App 不打包它们,直接调用现有安装):
  ├─ D:\TRSSYUNZAI\redis-windows-7.0.4\redis-server.exe
  ├─ D:\TRSSYUNZAI\Yunzai-Bot\app.js
  ├─ C:\Users\34018\AppData\Local\nvm\v20.11.1\node.exe (主人用的是 nvm 管理的 node)
  └─ D:\NapCat.Shell\launcher.bat
```

### 启动顺序(严格)

```
1. Redis        — daemonize 模式,不弹窗
   ↓ 等待 127.0.0.1:6379 listen (5s)
2. Yunzai       — node app.js,等待 2536 端口 listen (15s)
   ↓ 此时 WebSocket 客户端已启动,等待 NapCat 反向连接
3. NapCat       — launcher.bat (cmd.exe 调起)
   ↓ 等待 6099 端口 listen (30s)
   ↓ 此时 WebUI 可扫码登录
```

**为什么必须先启 Yunzai?** NapCat 是 WebSocket **Server** 端,Yunzai 是 **Client**。
Client 必须先准备好才能接收 Server 反向连接。

### 关闭顺序

```
1. NapCat WinBootMain + 它拉起的 QQ.exe ×3  (杀进程树)
2. Yunzai node.exe (按 cmdline 含 app.js 兜底扫)
3. Redis 保留 (主人偏好)
```

### 扫码登录

NapCat 自己的终端会弹一个二维码 —— **那是无效的,不要扫**。
只能在 `http://localhost:6099/webui/` 里扫码。本 App 的 WebUI 标签页直接 iframe 嵌这个地址。

## 工具链要求

- Windows 10/11
- Node.js 20.x (开发用,运行不需要)
- Rust 1.77+ (开发用)
- VS BuildTools 17.x + MSVC + Windows SDK 10.0.26100
- WebView2 Runtime (Win11 23H2+ 自带)

### ⚠️ SSH 静默装 VS BuildTools 假成功坑 (实战教训)

SSH service session 0 跑 `vs_setup.exe --quiet --wait` 会**假成功退出**，实际永远卡在 "Preparing" 阶段。CPU 0.4% 死循环。

schtasks /rl HIGHEST 也不行（SSH 没桌面 session,vs_setup 立即 background 后退出）。

**正解**：主人在自己电脑面前双击桌面上的 **`安装VSBuildTools.bat`**（已自动放到 C:\Users\34018\Desktop\）：
- 弹 UAC → 点"是"
- 自动后台安装 5-15 分钟
- 装完弹窗"Press Enter to close"
- 关窗后我会远程检测到 `link.exe` 出现，继续打包

**主人装好 VS 后**：双击桌面 **`打包YunZaiAPP.bat`**，自动跑完所有步骤，产出 `.msi + .exe` 安装包，弹窗打开产物目录。

## 开发

```bash
# 安装依赖
npm install

# 第一次 cargo fetch
cd src-tauri && cargo fetch && cd ..

# 启动开发模式 (主人面前双击 scripts/start-dev.bat 最稳)
npm run tauri dev

# 打包 (产 .msi + nsis .exe)
npm run tauri build
# 产物: src-tauri/target/release/bundle/msi/*.msi
#       src-tauri/target/release/bundle/nsis/*.exe
```

## 配置

第一次启动会在 `%APPDATA%/com.ethanwu.yunzai/config.json` 创建配置。

修改路径: 设置面板 → 编辑 → 保存。重启 App 生效。

## 故障排查

| 现象 | 排查 |
|---|---|
| 点"一键启动"卡在 Yunzai | 看实时日志,找 `[ERRO]` 行;最常见是 nvm node 路径不对或 Redis 没起来 |
| WebUI 标签页空白 | NapCat 没启动 (端口 6099 不 listen);点概览页检查 NapCat 状态 |
| 关窗后 App 还在 | 主人偏好"最小化到托盘";右键托盘图标 → 退出 |
| Redis 仍然被杀 | 不应该发生;Redis 用的是 `daemonize yes`,不会跟 App 生命周期绑定 |
| NapCat 扫码登录失败 | 别扫 NapCat 终端自带二维码;必须扫 WebUI 页面里的 |
| 自启失效 | 检查注册表 `HKCU\Software\Microsoft\Windows\CurrentVersion\Run` 是否有 `com.ethanwu.yunzai` |

## 致谢

- TRSS Yunzai — [github.com/TimeRainStarSky/Yunzai](https://github.com/TimeRainStarSky/Yunzai)
- NapCat — [github.com/NapNeko/NapCatQQ](https://github.com/NapNeko/NapCatQQ)
- Tauri 2 — [tauri.app](https://tauri.app)
- framer-motion / tailwindcss / sysinfo 等所有依赖作者

