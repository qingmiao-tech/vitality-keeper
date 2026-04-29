# GitHub Releases 自动更新发布说明

## 当前方案

项目已接入 Tauri 官方 `updater` 插件，更新源默认指向：

`https://github.com/qingmiao-tech/vitality-keeper/releases/latest/download/latest.json`

这意味着后续版本发布时，需要把 Tauri 构建产出的 updater 产物一起上传到 GitHub Release。

## 本地密钥

- 公钥：已经写入 `src-tauri/tauri.conf.json`
- 私钥：建议保存在当前构建机或 CI Secret 中，不要提交到仓库
- 当前机器私钥默认路径：`%USERPROFILE%/.tauri/vitality-keeper.key`
- 当前机器密码文件路径：`%USERPROFILE%/.tauri/vitality-keeper-key-password.txt`

## 发布步骤

1. 设置环境变量：

```powershell
$env:TAURI_SIGNING_PRIVATE_KEY=Get-Content "$env:USERPROFILE/.tauri/vitality-keeper.key" -Raw
$env:TAURI_SIGNING_PRIVATE_KEY_PASSWORD=Get-Content "$env:USERPROFILE/.tauri/vitality-keeper-key-password.txt"
```

2. 执行构建：

```powershell
npm run build:release
```

3. 在 `src-tauri/target/release/bundle/` 中收集安装包和 updater 产物。

其中 `latest.json` 会由 `scripts/generate-latest-json.mjs` 自动生成到：

`src-tauri/target/release/bundle/updater/latest.json`

至少需要上传：

- Windows 安装包，例如 `*-setup.exe` 或 `*.msi`
- 对应的签名文件 `*.sig`
- `latest.json`

4. 在 GitHub 上创建对应版本的 Release，并上传以上文件。

## 为什么很多开源项目也这么做

这是 Tauri、Electron、Flutter 桌面应用里都很常见的一类发布路径：

- GitHub Releases 负责托管安装包和版本元数据
- 应用内负责检查版本、下载更新和触发安装
- CI 或本地发版脚本负责生成签名和 `latest.json`

优点是实现成本低、对开源项目友好、发布记录清晰，也方便外部用户直接下载历史版本。
