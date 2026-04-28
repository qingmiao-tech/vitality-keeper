@echo off
set PATH=C:\Users\Administrator\.rustup\toolchains\stable-x86_64-pc-windows-msvc\bin;%PATH%
echo [Vitality Keeper] cargo found at:
where cargo
echo.
echo [Vitality Keeper] Starting dev server...
npx tauri dev
