# GitHub Releases Auto-update Notes

## Current Setup

The app now uses Tauri's official updater plugin and points to:

`https://github.com/qingmiao-tech/vitality-keeper/releases/latest/download/latest.json`

Every future release should upload the generated updater artifacts together with the installer packages.

## Signing Keys

- The public key is committed in `src-tauri/tauri.conf.json`
- Keep the private key only on the build machine or in CI secrets
- Default private key path on this machine: `%USERPROFILE%/.tauri/vitality-keeper.key`
- Default password file path on this machine: `%USERPROFILE%/.tauri/vitality-keeper-key-password.txt`

## Release Flow

1. Export the signing environment variables:

```powershell
$env:TAURI_SIGNING_PRIVATE_KEY=Get-Content "$env:USERPROFILE/.tauri/vitality-keeper.key" -Raw
$env:TAURI_SIGNING_PRIVATE_KEY_PASSWORD=Get-Content "$env:USERPROFILE/.tauri/vitality-keeper-key-password.txt"
```

2. Build the app:

```powershell
npm run build:release
```

3. Collect the installer packages and updater artifacts from `src-tauri/target/release/bundle/`.

`latest.json` is generated automatically by `scripts/generate-latest-json.mjs` at:

`src-tauri/target/release/bundle/updater/latest.json`

Upload at least:

- The Windows installer, such as `*-setup.exe` or `*.msi`
- The matching `*.sig` signature file
- `latest.json`

4. Create the matching GitHub Release and upload those files.

## Why Open-source Desktop Apps Often Use This

This is a common pattern across Tauri, Electron, and Flutter desktop projects:

- GitHub Releases hosts installers and version metadata
- The app handles in-app version checks, downloads, and installation
- CI or local release scripts generate the signatures and `latest.json`

It keeps the release pipeline affordable for open-source maintainers and makes past releases easy to browse and download.
