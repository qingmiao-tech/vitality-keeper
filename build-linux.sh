#!/usr/bin/env bash
# ============================================================
#  Vitality Keeper v3 — Linux (Ubuntu/Debian) Build Script
#  Run this on an Ubuntu 22.04+ machine to produce .deb and .AppImage
# ============================================================
set -euo pipefail

echo "=== [1/4] Installing system dependencies ==="
sudo apt-get update
sudo apt-get install -y \
  build-essential \
  libgtk-3-dev \
  libwebkit2gtk-4.1-dev \
  libappindicator3-dev \
  librsvg2-dev \
  patchelf \
  curl \
  wget \
  file \
  libssl-dev \
  libayatana-appindicator3-dev

echo "=== [2/4] Ensuring Rust toolchain ==="
if ! command -v rustup &> /dev/null; then
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
  source "$HOME/.cargo/env"
fi
rustup update stable

echo "=== [3/4] Ensuring Node.js & npm ==="
if ! command -v node &> /dev/null; then
  curl -fsSL https://deb.nodesource.com/setup_20.x | sudo -E bash -
  sudo apt-get install -y nodejs
fi

echo "=== [4/4] Building Vitality Keeper ==="
npm install
npm run build

echo ""
echo "==========================================="
echo "  Build complete!"
echo "  Output artifacts:"
echo "    src-tauri/target/release/bundle/deb/"
echo "    src-tauri/target/release/bundle/appimage/"
echo "==========================================="
