# v0.2.1 版本更新说明

元气守恒·筑基令 v0.2.1 聚焦于“休息流程真正稳定、多屏同步更可靠、视频编排更可控”。

## Highlights

- 统一“立即休息”入口，控制中心与托盘菜单现在都会走同一条 Rust 休息启动链路。
- 新增顺序循环视频队列，支持“下一次休息将播放”预告与“设为下一次”手动指定。
- 强化多屏休息页同步，改进休息窗口创建容错、状态补发与非阻塞视频启动。
- 延续阿拉伯语与语言选择优化，语言选项始终以对应母语显示。
- 将应用、安装包与更新元数据统一提升至 `v0.2.1`。

## Downloads

- `vitality-keeper_0.2.1_x64-setup.exe`
- `vitality-keeper_0.2.1_x64_en-US.msi`

## Release Notes

- 本版本继续使用 GitHub Releases 与 `latest.json` 作为自动更新分发源。
- Windows 安装包与 updater 签名文件会一起发布，用于应用内版本检测与自动更新。
- 如果你在多屏环境中使用“立即休息”，现在各休息页会以同一份媒体状态作为同步基线。

## English Summary

Vitality Keeper v0.2.1 unifies the immediate-break flow across the dashboard and tray, adds a sequential break-video queue with “play next” controls and upcoming-video preview, and improves multi-display break-window synchronization with more resilient window creation and non-blocking media startup.
