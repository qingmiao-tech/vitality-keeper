# macOS Release Workflow

仓库已提供 GitHub Actions 工作流：

- 文件位置：`.github/workflows/macos-release.yml`
- 作用：在 `macos-latest` runner 上构建 Vitality Keeper 的 macOS `app` 与 `dmg` 发布包

## 触发方式

1. 手动触发

- 进入 GitHub 仓库的 `Actions`
- 选择 `macOS Release`
- 点击 `Run workflow`
- 可选填写：
  - `release_tag`
  - `release_name`
  - `release_draft`
  - `prerelease`

2. 推送标签自动触发

- 推送形如 `v0.2.3` 的标签
- 工作流会自动开始构建并上传 Release 资产

## 构建产物

- Apple Silicon：`aarch64-apple-darwin`
- Intel：`x86_64-apple-darwin`
- Bundles：`.app`、`.dmg`

## Secrets 说明

### 最低可运行

如果不配置 Apple 证书，工作流会构建未公证的测试包，但仍需要 Tauri updater 签名 secrets：

- `TAURI_SIGNING_PRIVATE_KEY`
- `TAURI_SIGNING_PRIVATE_KEY_PASSWORD`
- 适合内部测试
- 外部分发时，macOS 仍可能提示安全限制

### 正式分发建议配置

如需更正式的外部分发，请配置以下 secrets：

- `APPLE_CERTIFICATE`
- `APPLE_CERTIFICATE_PASSWORD`
- `APPLE_ID`
- `APPLE_PASSWORD`
- `APPLE_TEAM_ID`

说明：

- `APPLE_CERTIFICATE` 为 `.p12` 证书的 base64 内容
- `APPLE_PASSWORD` 建议使用 Apple app-specific password
- `APPLE_TEAM_ID` 为 Apple Developer Team ID

## 官方限制

Tauri 官方要求 macOS bundle 在 Mac 环境中构建，因此本工作流使用 GitHub 的 macOS runner 来完成打包。

参考：

- <https://v2.tauri.app/distribute/macos-application-bundle/>
- <https://v2.tauri.app/distribute/pipelines/github/>
- <https://v2.tauri.app/distribute/sign/macos/>
