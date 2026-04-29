# 元气守恒·筑基令

元气守恒·筑基令（Vitality Keeper）是一款轻量级桌面休息提醒工具。它用 Rust + Tauri 常驻后台，在合适的时间打开全屏休息页，并播放你选择的本地拉伸、健身或养生视频，帮助久坐工作者把休息真正做完。

[English README](./README.en.md)

## 为什么做它

很多休息提醒工具只是在屏幕角落弹一下通知，很容易被忽略。元气守恒·筑基令更像一个温和但坚定的节律守护器：平时安静待在系统托盘里，时间到了就进入“筑基休息”，用全屏视频把注意力从工作里拉出来。

## 核心功能

- 本地视频引导：选择单个视频或一个文件夹，休息时自动播放。
- 多屏幕同步：多个显示器会进入同一轮休息流程，播放、暂停、静音和全屏状态保持一致。
- 两种结束策略：可以跟随视频自然结束，也可以按固定休息时长循环播放。
- 完整全屏模式：支持铺满屏幕，也支持完整显示视频内容，适配超宽屏和竖屏。
- 系统托盘：可从托盘快速打开控制中心、开始休息、跳过或推迟休息。
- 推迟限制：同一轮休息最多推迟 3 次，避免无限拖延。
- 开机自启：可随系统启动自动在后台运行。
- 中英文界面：默认简体中文，也提供英文界面。

## 截图

> 当前截图会随 `v0.1.7` 发布一起同步。

![控制中心总览](./docs/images/dashboard-overview.png)

![本地视频设置](./docs/images/dashboard-media.png)

![筑基休息页面](./docs/images/break-window.png)

## 下载

当前公开版本优先提供 Windows 安装包：

- `vitality-keeper_0.1.7_x64-setup.exe`
- `vitality-keeper_0.1.7_x64_en-US.msi`

请到 [GitHub Releases](https://github.com/qingmiao-tech/vitality-keeper/releases) 下载最新版本。

macOS 和 Linux 目前建议从源码构建。Tauri 本身支持跨平台，后续会逐步补齐自动化构建产物。

## 从源码运行

需要提前安装：

- Node.js 18+
- Rust stable
- Windows 上建议安装 Visual Studio Build Tools，包含 MSVC 工具链

```bash
npm install
npm run dev
```

Windows 开发环境也可以直接运行：

```bat
dev.bat
```

## 打包构建

```bash
npm run build
```

如果需要生成 GitHub 自动更新所需的签名与 `latest.json`，请使用：

```bash
npm run build:release
```

Windows 产物会生成在：

```text
src-tauri/target/release/bundle/
```

Linux 可参考：

```bash
./build-linux.sh
```

## 项目结构

```text
.
├── src/              # 控制中心、休息页和前端配置桥接
├── src-tauri/        # Rust 后端、系统托盘、窗口管理和打包配置
├── docs/             # 架构、产品审阅和发布说明
├── README.md         # 中文说明
└── README.en.md      # English README
```

## 设计取向

这个项目保留了“筑基令”的东方健康隐喻，但所有功能说明都尽量保持清楚直接。我们希望它既有一点记忆点，也能让第一次打开仓库的人马上知道：这是一个轻量、可本地化、可自定义视频的休息提醒工具。

## 贡献

欢迎提交 issue 和 pull request。比较适合参与的方向：

- macOS / Linux 打包验证
- 更稳定的 DND 和全局快捷键支持
- 更多语言翻译
- 更好的无障碍和键盘操作体验
- 更适合公开分发的默认视频素材建议

## 许可证

本项目使用 [MIT License](./LICENSE) 开源。
