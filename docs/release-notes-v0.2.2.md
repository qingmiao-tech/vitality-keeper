# v0.2.2 版本更新说明

元气守恒·筑基令 v0.2.2 是一次面向已安装 Windows 版本的热修复发布，重点解决“休息页按钮空白、重复点击开始休息后卡死、视频加载前倒计时误显示 00:00”这几类问题。

## Highlights

- 休息页与控制中心不再依赖外部图标脚本，安装版在离线或网络不稳定时也能正常显示底部按钮图标。
- “开始筑基休息”在当前已经处于休息流程时改为幂等处理，重复点击不会重新刷坏当前视频状态。
- 视频时长元数据尚未返回时，休息页倒计时会使用当前轮次的预计时长兜底，不再直接显示 `00:00`。
- 将应用、安装包与 updater 元数据统一提升至 `v0.2.2`。

## Downloads

- `vitality-keeper_0.2.2_x64-setup.exe`
- `vitality-keeper_0.2.2_x64_en-US.msi`

## Release Notes

- 本版本继续使用 GitHub Releases 与 `latest.json` 作为自动更新分发源。
- Windows 安装包、签名文件与 `latest.json` 会一起发布，用于应用内版本检测和更新安装。
- 如果你之前遇到休息页底部按钮变成空胶囊、反复点击开始休息后视频不播放或页面假死，这一版就是针对这些问题的热修复。

## English Summary

Vitality Keeper v0.2.2 is a Windows hotfix release that bundles a local icon fallback for the break page, makes “start break now” idempotent during an active break session, and adds a countdown fallback while video metadata is still loading.
