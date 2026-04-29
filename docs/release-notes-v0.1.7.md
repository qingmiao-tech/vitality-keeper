# v0.1.7 版本更新说明

元气守恒·筑基令 v0.1.7 聚焦于“版本可见、更新可达、发布可维护”。

## Highlights

- 控制中心新增当前版本号展示，不再需要到安装包或仓库页确认版本。
- 新增 GitHub Releases 更新检查能力，可在应用内查看新版本状态。
- 新增启动时自动检查更新开关，配置已下沉到 Rust + store。
- 接入 Tauri 官方 updater，支持下载签名安装包并触发安装。
- 新增 `latest.json` 生成脚本，方便后续 release 直接产出自动更新元数据。

## Downloads

- `vitality-keeper_0.1.7_x64-setup.exe`
- `vitality-keeper_0.1.7_x64_en-US.msi`
- `vitality-keeper_0.1.7_x64-setup.exe.sig`
- `vitality-keeper_0.1.7_x64_en-US.msi.sig`
- `latest.json`

## Release Notes

- 本版本开始，GitHub Release 需要同时上传安装包、`.sig` 签名文件和 `latest.json`。
- 控制中心中的更新说明会优先展示 `CHANGELOG.md` 对应版本内容。
- 当前预构建安装包仍优先支持 Windows x64。

## English Summary

Vitality Keeper v0.1.7 adds in-app version visibility, GitHub Releases based update checks, a persisted auto-check preference, and release tooling for signed updater assets plus `latest.json`. Windows x64 remains the primary prebuilt target.
