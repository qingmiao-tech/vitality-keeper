# Changelog

## v0.2.1 - 2026-05-08

- Reworked the break-session entry flow so both the dashboard action and the tray action use the same Rust-backed start-break path.
- Added sequential video queue playback with persisted `next_video_path`, dashboard “play next” controls, and next-break preview messaging.
- Refined break-window synchronization for multi-display playback, including more resilient break window creation and non-blocking media startup.
- Added Arabic copy refinements, native-script language labels, and system settings polish for shortcut, language, copy-style, and update sections.
- Promoted the Windows app, installer, and updater metadata to `v0.2.1` in preparation for the next GitHub release.

## v0.1.9 - 2026-05-07

- Added Arabic interface support with RTL layout handling across the dashboard, break window, and system-level labels.
- Refined the language picker so each language always displays in its own native script, regardless of the current UI language.
- Reworked the system settings page with clearer grouping for startup, language/copy, and update-related controls.
- Improved the global shortcut flow with editable recording UI, runtime registration feedback, and persisted shortcut configuration.
- Promoted the Windows app and installer version metadata to `v0.1.9` and refreshed the bundled release artifacts.

## v0.1.8 - 2026-04-30

- Added a real global shortcut registration for `Ctrl + Alt + Q`, plus a runtime status badge and manual reset action in the control center.
- Added a configurable copy style switch with a default public tone and an optional Dao-inspired “归元 / Ritual” narrative layer.
- Applied the selected copy style across both the dashboard and the break window so the language stays consistent during recovery.
- Refined the system settings visuals for language, shortcut status, and copy style controls in both dark and light themes.
- Extended the Rust-backed config model and release bundle metadata to carry the new shortcut and copy-style behavior cleanly.

## v0.1.7 - 2026-04-29

- Added dashboard version display and a dedicated update status panel.
- Added GitHub Releases based update checks and installer handoff through Tauri updater.
- Added persisted auto-check update preference in the shared Rust-backed config store.
- Added signed updater manifest generation so Windows release assets can be published consistently.
- Refined dashboard copy, status messaging, and release documentation for the new update flow.

## v0.1.6 - 2026-04-28

- Prepared the first open-source release of Vitality Keeper.
- Promoted the Tauri rewrite to the repository root.
- Updated project metadata, package identity, and MIT license.
- Reworked the Chinese and English README for public users and contributors.
- Refined Chinese product copy while keeping the "筑基令" brand tone.
- Planned Windows release assets: NSIS installer, MSI package, and source archive.
