# Architecture

Vitality Keeper is built with Tauri v2, Rust, and vanilla HTML/CSS/JavaScript. The product stays intentionally small: break scheduling, local video guidance, multi-display break windows, and system tray controls.

## Core Modules

- Rust backend: countdown timer, tray menu, multi-window creation, autostart, config persistence, and idle detection.
- Dashboard frontend: settings, local media selection, language switching, themes, and playlist preview.
- Break window: local video playback, countdown, progress line, pause/mute/fullscreen synchronization.
- Config layer: `tauri-plugin-store` is the source of truth, avoiding frontend/backend state drift.

## Break Flow

1. The Rust timer tracks the focus interval.
2. When the interval ends, the backend starts one break flow and selects the current video.
3. A frameless break window is created for each active display.
4. The primary break window publishes playback progress and control state.
5. Other break windows subscribe to the same media state and stay synchronized.
6. Finish, skip, and postpone actions close the break windows through the backend.

## Tradeoffs

- The rewrite keeps only the long-break flow and intentionally removes short-break branching.
- Local videos are first-class; user media stays on the device.
- Tauri asset protocol is used for local media access and avoids fragile local-file playback workarounds.
- Windows is the primary packaged platform for the first release. macOS and Linux remain source-build targets until packaging is validated.
