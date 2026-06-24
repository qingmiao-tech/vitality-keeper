# v0.2.3 版本更新说明

Vitality Keeper v0.2.3 是一次面向发布链路的版本提升，重点补齐 macOS 自动打包流程，并统一同步控制中心、安装包与 updater 的版本元数据。

## Highlights

- 新增 GitHub Actions macOS 发布工作流，可在 `macos-latest` 上为 Apple Silicon 和 Intel 自动构建 `.app` 与 `.dmg`。
- 补充 macOS 发布说明文档，明确 ad-hoc 测试签名与 Apple 正式签名所需 secrets。
- 将应用版本、安装包版本、控制中心版本展示与 updater 元数据统一提升至 `v0.2.3`。

## Downloads

- `vitality-keeper_0.2.3_x64-setup.exe`
- `vitality-keeper_0.2.3_x64_en-US.msi`
- `Vitality Keeper.app`
- `Vitality Keeper.dmg`

## Release Notes

- Windows 侧继续通过本地构建产出安装包，并配合 `latest.json` 作为应用内更新源。
- macOS 侧改为通过 GitHub Actions 在 macOS runner 上生成 `.app` 与 `.dmg`，避免在 Windows 环境下无法直接构建 Apple 安装产物的问题。
- 如果仓库未配置 Apple 证书，macOS 工作流会自动回退到 ad-hoc 签名，适合内部测试与验证流程。

## English Summary

Vitality Keeper v0.2.3 adds a dedicated GitHub Actions workflow for macOS packaging, aligns app and installer metadata with the new release number, and keeps the Windows updater pipeline in sync with the new version.
