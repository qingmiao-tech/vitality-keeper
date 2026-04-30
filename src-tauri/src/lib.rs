use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::sync::Mutex;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tauri::menu::{Menu, MenuItem, SubmenuBuilder};
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
use tauri::{Emitter, Manager, PhysicalPosition, PhysicalSize, WebviewUrl, WebviewWindowBuilder};
use tauri_plugin_global_shortcut::{Code, Modifiers, ShortcutState};
use tauri_plugin_store::StoreExt;
use tauri_plugin_updater::UpdaterExt;

const APP_CONFIG_STORE_PATH: &str = "settings.json";
const APP_CONFIG_KEY: &str = "app-config";
const DEFAULT_WORK_MINUTES: u64 = 45;
const DEFAULT_BREAK_MINUTES: u64 = 5;
const DEFAULT_POSTPONE_SECONDS: u64 = 5 * 60;
const DEFAULT_LANGUAGE: &str = "zh-CN";
const MAX_POSTPONES_PER_FLOW: u8 = 3;
const WINDOWS_APP_USER_MODEL_ID: &str = "com.kingmo.vitality-keeper";

fn current_unix_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64
}

#[cfg(windows)]
fn get_idle_time() -> u64 {
    use std::mem;
    use windows::Win32::System::SystemInformation::GetTickCount;
    use windows::Win32::UI::Input::KeyboardAndMouse::{GetLastInputInfo, LASTINPUTINFO};

    let mut info = LASTINPUTINFO {
        cbSize: mem::size_of::<LASTINPUTINFO>() as u32,
        dwTime: 0,
    };

    unsafe {
        let _ = GetLastInputInfo(&mut info);
        let current_tick = GetTickCount();
        if info.dwTime > 0 && current_tick >= info.dwTime {
            return (current_tick - info.dwTime) as u64 / 1000;
        }
    }

    0
}

#[cfg(not(windows))]
fn get_idle_time() -> u64 {
    0
}

#[cfg(windows)]
fn apply_windows_app_identity() {
    use windows::core::HSTRING;
    use windows::Win32::UI::Shell::SetCurrentProcessExplicitAppUserModelID;

    let _ = unsafe {
        SetCurrentProcessExplicitAppUserModelID(&HSTRING::from(WINDOWS_APP_USER_MODEL_ID))
    };
}

#[cfg(not(windows))]
fn apply_windows_app_identity() {}

fn apply_app_icon_to_window<R: tauri::Runtime>(
    app: &tauri::AppHandle<R>,
    window: &tauri::WebviewWindow<R>,
) {
    if let Some(icon) = app.default_window_icon() {
        let _ = window.set_icon(icon.clone());
    }
}

fn sync_autostart_state<R: tauri::Runtime>(
    app: &tauri::AppHandle<R>,
    enabled: bool,
) -> Result<bool, String> {
    use tauri_plugin_autostart::ManagerExt;

    let manager = app.autolaunch();
    if enabled {
        manager.enable().map_err(|error| error.to_string())?;
    } else {
        manager.disable().map_err(|error| error.to_string())?;
    }

    manager.is_enabled().map_err(|error| error.to_string())
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
struct AppConfig {
    strict_mode: bool,
    fullscreen: bool,
    fullscreen_fit_mode: String,
    autostart_enabled: bool,
    auto_check_updates_enabled: bool,
    dnd_enabled: bool,
    natural_breaks_enabled: bool,
    work_minutes: u64,
    break_minutes: u64,
    video_source: String,
    break_strategy: String,
    disabled_videos: Vec<String>,
    custom_video_order: Vec<String>,
    theme: String,
    language: String,
    copy_style: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            strict_mode: false,
            fullscreen: true,
            fullscreen_fit_mode: "cover".to_string(),
            autostart_enabled: true,
            auto_check_updates_enabled: true,
            dnd_enabled: false,
            natural_breaks_enabled: true,
            work_minutes: DEFAULT_WORK_MINUTES,
            break_minutes: DEFAULT_BREAK_MINUTES,
            video_source: String::new(),
            break_strategy: "video".to_string(),
            disabled_videos: Vec::new(),
            custom_video_order: Vec::new(),
            theme: "dao".to_string(),
            language: DEFAULT_LANGUAGE.to_string(),
            copy_style: "balanced".to_string(),
        }
    }
}

impl AppConfig {
    fn normalized(mut self) -> Self {
        self.work_minutes = self.work_minutes.clamp(5, 180);
        self.break_minutes = self.break_minutes.clamp(1, 60);
        self.break_strategy = if self.break_strategy == "fixed" {
            "fixed".to_string()
        } else {
            "video".to_string()
        };
        self.fullscreen_fit_mode = if self.fullscreen_fit_mode == "contain" {
            "contain".to_string()
        } else {
            "cover".to_string()
        };
        self.theme = if self.theme == "light" {
            "light".to_string()
        } else {
            "dao".to_string()
        };
        self.language = if self.language == "en-US" {
            "en-US".to_string()
        } else {
            DEFAULT_LANGUAGE.to_string()
        };
        self.copy_style = if self.copy_style == "tao" {
            "tao".to_string()
        } else {
            "balanced".to_string()
        };
        self.video_source = self.video_source.trim().to_string();
        self
    }
}

fn is_english(config: &AppConfig) -> bool {
    config.language == "en-US"
}

fn app_display_name(config: &AppConfig) -> &'static str {
    if is_english(config) {
        "Vitality Keeper"
    } else {
        "元气守恒·筑基令"
    }
}

fn tray_timer_label(config: &AppConfig, mins: u64, secs: u64) -> String {
    if is_english(config) {
        format!("⏱ Next break: {:02}:{:02}", mins, secs)
    } else {
        format!("⏱ 距离休息: {:02}:{:02}", mins, secs)
    }
}

fn tray_show_label(config: &AppConfig) -> &'static str {
    if is_english(config) {
        "Dashboard"
    } else {
        "控制中心"
    }
}

fn tray_start_break_label(config: &AppConfig) -> &'static str {
    if is_english(config) {
        "Start Break"
    } else {
        "开始筑基休息"
    }
}

fn tray_skip_break_label(config: &AppConfig) -> &'static str {
    if is_english(config) {
        "Skip Current Break"
    } else {
        "跳过本次休息"
    }
}

fn tray_postpone_menu_label(config: &AppConfig, active: bool, remaining_times: u8) -> String {
    if !active {
        if is_english(config) {
            "Postpone Break".to_string()
        } else {
            "推迟休息".to_string()
        }
    } else if remaining_times > 0 {
        if is_english(config) {
            format!("Postpone Break ({} left)", remaining_times)
        } else {
            format!("推迟休息 (剩余 {} 次)", remaining_times)
        }
    } else if is_english(config) {
        format!("Postpone Break (limit {})", MAX_POSTPONES_PER_FLOW)
    } else {
        format!("推迟休息 (上限 {} 次)", MAX_POSTPONES_PER_FLOW)
    }
}

fn tray_postpone_option_label(config: &AppConfig, minutes: u64) -> String {
    if is_english(config) {
        format!("{} min", minutes)
    } else {
        format!("{} 分钟", minutes)
    }
}

fn postpone_limit_reached_message(config: &AppConfig) -> String {
    if is_english(config) {
        format!(
            "This break flow can be postponed at most {} times.",
            MAX_POSTPONES_PER_FLOW
        )
    } else {
        format!("同一轮休息流程最多只能推迟 {} 次。", MAX_POSTPONES_PER_FLOW)
    }
}

fn no_pending_break_message(config: &AppConfig) -> String {
    if is_english(config) {
        "There is no active break flow to postpone.".to_string()
    } else {
        "当前没有可推迟的休息流程。".to_string()
    }
}

fn tray_quit_label(config: &AppConfig) -> &'static str {
    if is_english(config) {
        "Quit"
    } else {
        "退出"
    }
}

struct TimerState {
    time_remaining: Mutex<u64>,
    is_paused: Mutex<bool>,
    work_duration: Mutex<u64>,
    break_duration: Mutex<u64>,
}

struct AppConfigState {
    current: Mutex<AppConfig>,
}

struct ShortcutRegistrationState {
    reset_cycle_registered: Mutex<bool>,
}

#[derive(Clone, Copy, Debug)]
struct BreakFlowSession {
    active: bool,
    postpone_count: u8,
}

impl Default for BreakFlowSession {
    fn default() -> Self {
        Self {
            active: true,
            postpone_count: 0,
        }
    }
}

struct BreakFlowState {
    current: Mutex<BreakFlowSession>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
struct BreakMediaState {
    video_path: String,
    muted: bool,
    paused: bool,
    fullscreen: bool,
    fullscreen_fit_mode: String,
    current_time: f64,
    break_strategy: String,
    break_duration_seconds: f64,
    session_started_at_ms: u64,
    updated_at_ms: u64,
}

impl Default for BreakMediaState {
    fn default() -> Self {
        Self {
            video_path: String::new(),
            muted: false,
            paused: false,
            fullscreen: true,
            fullscreen_fit_mode: "cover".to_string(),
            current_time: 0.0,
            break_strategy: "video".to_string(),
            break_duration_seconds: 0.0,
            session_started_at_ms: 0,
            updated_at_ms: 0,
        }
    }
}

impl BreakMediaState {
    fn normalized(mut self) -> Self {
        self.video_path = self.video_path.trim().to_string();
        self.current_time = if self.current_time.is_finite() {
            self.current_time.max(0.0)
        } else {
            0.0
        };
        self.fullscreen_fit_mode = if self.fullscreen_fit_mode == "contain" {
            "contain".to_string()
        } else {
            "cover".to_string()
        };
        self.break_strategy = if self.break_strategy == "fixed" {
            "fixed".to_string()
        } else {
            "video".to_string()
        };
        self.break_duration_seconds = if self.break_duration_seconds.is_finite() {
            self.break_duration_seconds.max(0.0)
        } else {
            0.0
        };
        if self.updated_at_ms == 0 {
            self.updated_at_ms = current_unix_ms();
        }
        self
    }
}

struct BreakMediaStateStore {
    current: Mutex<BreakMediaState>,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct RuntimeInfo {
    version: String,
    reset_cycle_shortcut_registered: bool,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct UpdateCheckResult {
    available: bool,
    version: String,
    current_version: String,
    notes: String,
    published_at: String,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct UpdateInstallResult {
    version: String,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct UpdateProgressPayload {
    stage: String,
    version: String,
    percent: Option<f64>,
}

fn resolve_window_label(label: Option<String>) -> String {
    let resolved = label.unwrap_or_else(|| "main".to_string());
    let trimmed = resolved.trim();
    if trimmed.is_empty() {
        "main".to_string()
    } else {
        trimmed.to_string()
    }
}

fn resolve_window_by_label(
    app: &tauri::AppHandle,
    label: Option<String>,
) -> Result<tauri::WebviewWindow, String> {
    let resolved_label = resolve_window_label(label);
    app.get_webview_window(&resolved_label)
        .ok_or_else(|| format!("未找到窗口: {}", resolved_label))
}

fn get_break_flow_session(app: &tauri::AppHandle) -> BreakFlowSession {
    let state = app.state::<BreakFlowState>();
    let session = *state.current.lock().unwrap();
    session
}

fn ensure_break_flow_active(app: &tauri::AppHandle) -> BreakFlowSession {
    let state = app.state::<BreakFlowState>();
    let mut session = state.current.lock().unwrap();
    if !session.active {
        *session = BreakFlowSession {
            active: true,
            postpone_count: 0,
        };
    }
    *session
}

fn reset_break_flow_state(app: &tauri::AppHandle) {
    let state = app.state::<BreakFlowState>();
    *state.current.lock().unwrap() = BreakFlowSession::default();
}

fn close_break_windows(app: &tauri::AppHandle) {
    for (label, window) in app.webview_windows() {
        if label.starts_with("break_window_") {
            let _ = window.close();
        }
    }
}

fn is_break_window_showing(app: &tauri::AppHandle) -> bool {
    if app
        .webview_windows()
        .keys()
        .any(|label| label.starts_with("break_window_"))
    {
        return true;
    }

    let break_state = app.state::<BreakMediaStateStore>();
    let media_state = break_state.current.lock().unwrap();
    media_state.session_started_at_ms > 0
}

fn clear_break_media_state(app: &tauri::AppHandle) {
    let break_state = app.state::<BreakMediaStateStore>();
    *break_state.current.lock().unwrap() = BreakMediaState::default();
}

fn start_break_now(app: &tauri::AppHandle) {
    ensure_break_flow_active(app);
    let state = app.state::<TimerState>();
    *state.time_remaining.lock().unwrap() = 0;
}

fn finalize_break_flow(app: &tauri::AppHandle) {
    let state = app.state::<TimerState>();
    let work_duration = *state.work_duration.lock().unwrap();
    *state.time_remaining.lock().unwrap() = work_duration;
    *state.is_paused.lock().unwrap() = false;

    clear_break_media_state(app);
    reset_break_flow_state(app);
    close_break_windows(app);
    let _ = app.emit("timer-tick", work_duration);
}

fn reset_focus_cycle(app: &tauri::AppHandle) {
    finalize_break_flow(app);
    let state = app.state::<TimerState>();
    let work_duration = *state.work_duration.lock().unwrap();
    let _ = app.emit("focus-cycle-reset", work_duration);
}

fn postpone_current_break(app: &tauri::AppHandle, delay_seconds: u64) -> Result<(), String> {
    let config = app.state::<AppConfigState>().current.lock().unwrap().clone();
    let break_flow_state = app.state::<BreakFlowState>();
    let mut session = break_flow_state.current.lock().unwrap();

    if !session.active {
        return Err(no_pending_break_message(&config));
    }

    if session.postpone_count >= MAX_POSTPONES_PER_FLOW {
        return Err(postpone_limit_reached_message(&config));
    }

    session.postpone_count += 1;
    drop(session);

    let timer_state = app.state::<TimerState>();
    *timer_state.time_remaining.lock().unwrap() = delay_seconds;

    clear_break_media_state(app);
    close_break_windows(app);

    Ok(())
}

fn sync_timer_state(app: &tauri::AppHandle, config: &AppConfig, reset_to_work_duration: bool) {
    let timer_state = app.state::<TimerState>();
    let work_duration = config.work_minutes * 60;
    let break_duration = config.break_minutes * 60;

    *timer_state.work_duration.lock().unwrap() = work_duration;
    *timer_state.break_duration.lock().unwrap() = break_duration;

    let mut time_remaining = timer_state.time_remaining.lock().unwrap();
    if reset_to_work_duration || *time_remaining > work_duration {
        *time_remaining = work_duration;
    }
}

fn load_or_init_app_config(app: &tauri::AppHandle) -> Result<AppConfig, String> {
    let store = app
        .store(APP_CONFIG_STORE_PATH)
        .map_err(|error| error.to_string())?;

    let loaded_config = match store.get(APP_CONFIG_KEY) {
        Some(value) => serde_json::from_value::<AppConfig>(value)
            .map_err(|error| error.to_string())?
            .normalized(),
        None => AppConfig::default(),
    };

    store.set(
        APP_CONFIG_KEY,
        serde_json::to_value(&loaded_config).map_err(|error| error.to_string())?,
    );
    store.save().map_err(|error| error.to_string())?;

    Ok(loaded_config)
}

fn persist_app_config(app: &tauri::AppHandle, config: &AppConfig) -> Result<(), String> {
    let store = app
        .store(APP_CONFIG_STORE_PATH)
        .map_err(|error| error.to_string())?;

    store.set(
        APP_CONFIG_KEY,
        serde_json::to_value(config).map_err(|error| error.to_string())?,
    );
    store.save().map_err(|error| error.to_string())
}

fn replace_runtime_config(app: &tauri::AppHandle, config: AppConfig, reset_timer: bool) -> Result<AppConfig, String> {
    let mut normalized = config.normalized();
    if let Ok(registered) = sync_autostart_state(app, normalized.autostart_enabled) {
        normalized.autostart_enabled = registered;
    }

    persist_app_config(app, &normalized)?;
    sync_timer_state(app, &normalized, reset_timer);

    let state = app.state::<AppConfigState>();
    *state.current.lock().unwrap() = normalized.clone();

    Ok(normalized)
}

fn resolve_playable_videos(config: &AppConfig) -> Vec<String> {
    if config.video_source.is_empty() {
        return Vec::new();
    }

    let videos = get_video_list(config.video_source.clone());
    if videos.is_empty() {
        return videos;
    }

    let disabled_videos: HashSet<&str> = config.disabled_videos.iter().map(String::as_str).collect();
    let mut enabled_videos: Vec<String> = videos
        .iter()
        .filter(|path| !disabled_videos.contains(path.as_str()))
        .cloned()
        .collect();

    if enabled_videos.is_empty() {
        enabled_videos = videos.clone();
    }

    let custom_order: HashMap<&str, usize> = config
        .custom_video_order
        .iter()
        .enumerate()
        .map(|(index, path)| (path.as_str(), index))
        .collect();

    enabled_videos.sort_by(|left, right| {
        let left_index = custom_order.get(left.as_str()).copied().unwrap_or(usize::MAX);
        let right_index = custom_order.get(right.as_str()).copied().unwrap_or(usize::MAX);

        left_index.cmp(&right_index).then_with(|| left.cmp(right))
    });

    enabled_videos
}

fn select_break_video(config: &AppConfig) -> String {
    use std::time::{SystemTime, UNIX_EPOCH};

    let videos = resolve_playable_videos(config);
    if videos.is_empty() {
        return String::new();
    }

    let seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
        / 60;

    videos[(seed as usize) % videos.len()].clone()
}

fn create_break_media_state(config: &AppConfig) -> BreakMediaState {
    let session_started_at_ms = current_unix_ms();
    BreakMediaState {
        video_path: select_break_video(config),
        fullscreen: config.fullscreen,
        fullscreen_fit_mode: config.fullscreen_fit_mode.clone(),
        break_strategy: config.break_strategy.clone(),
        break_duration_seconds: if config.break_strategy == "fixed" {
            (config.break_minutes * 60) as f64
        } else {
            0.0
        },
        session_started_at_ms,
        updated_at_ms: session_started_at_ms,
        ..BreakMediaState::default()
    }
}

fn replace_break_media_state(app: &tauri::AppHandle, state: BreakMediaState) -> Result<BreakMediaState, String> {
    let normalized = state.normalized();
    let state_store = app.state::<BreakMediaStateStore>();
    *state_store.current.lock().unwrap() = normalized.clone();
    app.emit("break-media-state-changed", normalized.clone())
        .map_err(|error| error.to_string())?;
    Ok(normalized)
}

#[tauri::command]
fn get_time_remaining(state: tauri::State<TimerState>) -> u64 {
    *state.time_remaining.lock().unwrap()
}

#[tauri::command]
fn get_app_config(state: tauri::State<AppConfigState>) -> AppConfig {
    state.current.lock().unwrap().clone()
}

#[tauri::command]
fn update_app_config(app: tauri::AppHandle, config: AppConfig) -> Result<AppConfig, String> {
    replace_runtime_config(&app, config, false)
}

#[tauri::command]
fn get_autostart_enabled(app: tauri::AppHandle) -> Result<bool, String> {
    use tauri_plugin_autostart::ManagerExt;

    app.autolaunch()
        .is_enabled()
        .map_err(|error| error.to_string())
}

#[tauri::command]
fn get_runtime_info(app: tauri::AppHandle) -> RuntimeInfo {
    let shortcut_state = app.state::<ShortcutRegistrationState>();
    let reset_cycle_shortcut_registered = *shortcut_state.reset_cycle_registered.lock().unwrap();
    RuntimeInfo {
        version: app.package_info().version.to_string(),
        reset_cycle_shortcut_registered,
    }
}

#[tauri::command]
async fn check_for_updates(app: tauri::AppHandle) -> Result<UpdateCheckResult, String> {
    let current_version = app.package_info().version.to_string();
    let update = app
        .updater()
        .map_err(|error| error.to_string())?
        .check()
        .await
        .map_err(|error| error.to_string())?;

    if let Some(update) = update {
        return Ok(UpdateCheckResult {
            available: true,
            version: update.version.clone(),
            current_version: update.current_version.clone(),
            notes: update.body.clone().unwrap_or_default(),
            published_at: update
                .date
                .map(|value| value.to_string())
                .unwrap_or_default(),
        });
    }

    Ok(UpdateCheckResult {
        available: false,
        version: current_version.clone(),
        current_version,
        notes: String::new(),
        published_at: String::new(),
    })
}

#[tauri::command]
async fn install_update(app: tauri::AppHandle) -> Result<UpdateInstallResult, String> {
    let Some(update) = app
        .updater()
        .map_err(|error| error.to_string())?
        .check()
        .await
        .map_err(|error| error.to_string())?
    else {
        return Err("当前没有可安装的新版本。".to_string());
    };

    let version = update.version.clone();
    let progress_app = app.clone();
    let progress_version = version.clone();
    let finished_app = app.clone();
    let finished_version = version.clone();
    let mut downloaded = 0_u64;

    update
        .download_and_install(
            move |chunk_length, content_length| {
                downloaded = downloaded.saturating_add(chunk_length as u64);
                let percent = content_length.and_then(|total| {
                    if total == 0 {
                        None
                    } else {
                        Some((downloaded as f64 / total as f64) * 100.0)
                    }
                });

                let _ = progress_app.emit(
                    "update-download-progress",
                    UpdateProgressPayload {
                        stage: "downloading".to_string(),
                        version: progress_version.clone(),
                        percent,
                    },
                );
            },
            move || {
                let _ = finished_app.emit(
                    "update-download-progress",
                    UpdateProgressPayload {
                        stage: "installing".to_string(),
                        version: finished_version.clone(),
                        percent: Some(100.0),
                    },
                );
            },
        )
        .await
        .map_err(|error| error.to_string())?;

    Ok(UpdateInstallResult { version })
}

#[tauri::command]
fn get_break_media_state(state: tauri::State<BreakMediaStateStore>) -> BreakMediaState {
    state.current.lock().unwrap().clone()
}

#[tauri::command]
fn update_break_media_state(
    app: tauri::AppHandle,
    state: BreakMediaState,
) -> Result<BreakMediaState, String> {
    replace_break_media_state(&app, state)
}

#[tauri::command]
fn hide_main_window(app: tauri::AppHandle) -> Result<(), String> {
    let window = resolve_window_by_label(&app, Some("main".to_string()))?;
    window.hide().map_err(|error| error.to_string())
}

#[tauri::command]
fn minimize_main_window(app: tauri::AppHandle) -> Result<(), String> {
    let window = resolve_window_by_label(&app, Some("main".to_string()))?;
    window.minimize().map_err(|error| error.to_string())
}

#[tauri::command]
fn toggle_main_fullscreen(app: tauri::AppHandle) -> Result<bool, String> {
    let window = resolve_window_by_label(&app, Some("main".to_string()))?;
    let next_fullscreen = !window
        .is_fullscreen()
        .map_err(|error| error.to_string())?;
    window
        .set_fullscreen(next_fullscreen)
        .map_err(|error| error.to_string())?;
    Ok(next_fullscreen)
}

#[tauri::command]
fn control_window(
    app: tauri::AppHandle,
    action: String,
    label: Option<String>,
) -> Result<Option<bool>, String> {
    let window = resolve_window_by_label(&app, label)?;

    match action.as_str() {
        "hide" => {
            window.hide().map_err(|error| error.to_string())?;
            Ok(None)
        }
        "minimize" => {
            window.minimize().map_err(|error| error.to_string())?;
            Ok(None)
        }
        "toggle-fullscreen" => {
            let next_fullscreen = !window
                .is_fullscreen()
                .map_err(|error| error.to_string())?;
            window
                .set_fullscreen(next_fullscreen)
                .map_err(|error| error.to_string())?;
            Ok(Some(next_fullscreen))
        }
        _ => Err(format!("不支持的窗口操作: {}", action)),
    }
}

#[tauri::command]
fn close_breaks(app: tauri::AppHandle) {
    finalize_break_flow(&app);
}

#[tauri::command]
fn postpone_break(app: tauri::AppHandle, delay_seconds: Option<u64>) -> Result<(), String> {
    postpone_current_break(&app, delay_seconds.unwrap_or(DEFAULT_POSTPONE_SECONDS))
}

#[tauri::command]
fn reset_break_cycle(app: tauri::AppHandle) {
    reset_focus_cycle(&app);
}

#[tauri::command]
fn update_timer_settings(
    app: tauri::AppHandle,
    work_mins: u64,
    break_mins: u64,
) -> Result<AppConfig, String> {
    let state = app.state::<AppConfigState>();
    let mut config = state.current.lock().unwrap().clone();
    config.work_minutes = work_mins;
    config.break_minutes = break_mins;
    drop(state);

    replace_runtime_config(&app, config, false)
}

#[tauri::command]
fn get_video_list(source_path: String) -> Vec<String> {
    use std::fs;
    use std::path::Path;

    if source_path.trim().is_empty() {
        return Vec::new();
    }

    let path = Path::new(&source_path);
    let mut videos = Vec::new();

    if path.is_file() {
        if let Some(path_str) = path.to_str() {
            let lower = path_str.to_ascii_lowercase();
            if lower.ends_with(".mp4") || lower.ends_with(".webm") {
                videos.push(path_str.to_string());
            }
        }
    } else if path.is_dir() {
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries.flatten() {
                if let Some(path_str) = entry.path().to_str() {
                    let lower = path_str.to_ascii_lowercase();
                    if lower.ends_with(".mp4") || lower.ends_with(".webm") {
                        videos.push(path_str.to_string());
                    }
                }
            }
        }
    }

    videos.sort();
    videos
}

#[tauri::command]
fn get_random_video(source_path: String) -> String {
    use std::time::{SystemTime, UNIX_EPOCH};

    let videos = get_video_list(source_path);
    if videos.is_empty() {
        return String::new();
    }

    let seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as usize;
    videos[seed % videos.len()].clone()
}

#[tauri::command]
fn get_video_count(source_path: String) -> usize {
    get_video_list(source_path).len()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    apply_windows_app_identity();

    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            apply_windows_app_identity();
            if let Some(window) = app.get_webview_window("main") {
                apply_app_icon_to_window(app, &window);
                let _ = window.show();
                let _ = window.unminimize();
                let _ = window.set_focus();
            }
        }))
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            None,
        ))
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .manage(TimerState {
            time_remaining: Mutex::new(DEFAULT_WORK_MINUTES * 60),
            is_paused: Mutex::new(false),
            work_duration: Mutex::new(DEFAULT_WORK_MINUTES * 60),
            break_duration: Mutex::new(DEFAULT_BREAK_MINUTES * 60),
        })
        .manage(AppConfigState {
            current: Mutex::new(AppConfig::default()),
        })
        .manage(ShortcutRegistrationState {
            reset_cycle_registered: Mutex::new(false),
        })
        .manage(BreakFlowState {
            current: Mutex::new(BreakFlowSession::default()),
        })
        .manage(BreakMediaStateStore {
            current: Mutex::new(BreakMediaState::default()),
        })
        .setup(|app| {
            let app_handle = app.handle().clone();
            let initial_config = load_or_init_app_config(&app_handle)?;

            if let Some(main_window) = app_handle.get_webview_window("main") {
                apply_app_icon_to_window(&app_handle, &main_window);
            }

            #[cfg(desktop)]
            {
                let shortcut_plugin = tauri_plugin_global_shortcut::Builder::new()
                    .with_shortcuts(["ctrl+alt+q"])?
                    .with_handler(|app, shortcut, event| {
                        if event.state == ShortcutState::Pressed
                            && shortcut.matches(Modifiers::CONTROL | Modifiers::ALT, Code::KeyQ)
                        {
                            reset_focus_cycle(app);
                        }
                    })
                    .build();

                let registered = match app_handle.plugin(shortcut_plugin) {
                    Ok(_) => true,
                    Err(error) => {
                        eprintln!(
                            "全局热键 Ctrl+Alt+Q 注册失败，可能被其他应用占用或注册流程异常: {error}"
                        );
                        false
                    }
                };
                let shortcut_state = app_handle.state::<ShortcutRegistrationState>();
                *shortcut_state.reset_cycle_registered.lock().unwrap() = registered;
            }

            {
                let config_state = app_handle.state::<AppConfigState>();
                *config_state.current.lock().unwrap() = initial_config.clone();
            }
            sync_timer_state(&app_handle, &initial_config, true);
            let _ = sync_autostart_state(&app_handle, initial_config.autostart_enabled);

            use tauri::menu::PredefinedMenuItem;

            let quit_i = MenuItem::with_id(
                app,
                "quit",
                tray_quit_label(&initial_config),
                true,
                None::<&str>,
            )?;
            let show_i = MenuItem::with_id(
                app,
                "show",
                tray_show_label(&initial_config),
                true,
                None::<&str>,
            )?;
            let timer_i = MenuItem::with_id(
                app,
                "timer_info",
                tray_timer_label(&initial_config, initial_config.work_minutes, 0),
                false,
                None::<&str>,
            )?;
            let start_break_i = MenuItem::with_id(
                app,
                "start_break",
                tray_start_break_label(&initial_config),
                true,
                None::<&str>,
            )?;
            let skip_break_i = MenuItem::with_id(
                app,
                "skip_break",
                tray_skip_break_label(&initial_config),
                true,
                None::<&str>,
            )?;
            let postpone_3_i = MenuItem::with_id(
                app,
                "postpone_3",
                tray_postpone_option_label(&initial_config, 3),
                true,
                None::<&str>,
            )?;
            let postpone_5_i = MenuItem::with_id(
                app,
                "postpone_5",
                tray_postpone_option_label(&initial_config, 5),
                true,
                None::<&str>,
            )?;
            let postpone_10_i = MenuItem::with_id(
                app,
                "postpone_10",
                tray_postpone_option_label(&initial_config, 10),
                true,
                None::<&str>,
            )?;
            let postpone_menu = SubmenuBuilder::with_id(
                app,
                "postpone_menu",
                tray_postpone_menu_label(&initial_config, true, MAX_POSTPONES_PER_FLOW),
            )
            .items(&[&postpone_3_i, &postpone_5_i, &postpone_10_i])
            .enabled(true)
            .build()?;
            let sep1 = PredefinedMenuItem::separator(app)?;
            let sep2 = PredefinedMenuItem::separator(app)?;
            let sep3 = PredefinedMenuItem::separator(app)?;

            let menu = Menu::with_items(
                app,
                &[
                    &timer_i,
                    &sep1,
                    &show_i,
                    &sep2,
                    &start_break_i,
                    &skip_break_i,
                    &postpone_menu,
                    &sep3,
                    &quit_i,
                ],
            )?;

            let _tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .tooltip(app_display_name(&initial_config))
                .show_menu_on_left_click(false)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "quit" => {
                        std::process::exit(0);
                    }
                    "show" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                    "start_break" => {
                        start_break_now(app);
                    }
                    "skip_break" => {
                        finalize_break_flow(app);
                    }
                    "postpone_3" => {
                        let _ = postpone_current_break(app, 3 * 60);
                    }
                    "postpone_5" => {
                        let _ = postpone_current_break(app, 5 * 60);
                    }
                    "postpone_10" => {
                        let _ = postpone_current_break(app, 10 * 60);
                    }
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| match event {
                    TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } => {
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                    _ => {}
                })
                .build(app)?;

            let handle = app.handle().clone();
            let menu_handle = menu.clone();
            tauri::async_runtime::spawn(async move {
                loop {
                    tokio::time::sleep(Duration::from_secs(1)).await;

                    let state = handle.state::<TimerState>();
                    let mut time = state.time_remaining.lock().unwrap();
                    let paused = state.is_paused.lock().unwrap();
                    let work_duration = *state.work_duration.lock().unwrap();
                    let config = handle.state::<AppConfigState>().current.lock().unwrap().clone();
                    let break_window_showing = is_break_window_showing(&handle);
                    let break_flow = get_break_flow_session(&handle);
                    let remaining_postpones =
                        MAX_POSTPONES_PER_FLOW.saturating_sub(break_flow.postpone_count);
                    let can_manage_current_break = break_flow.active;
                    let can_postpone_current_break =
                        can_manage_current_break && remaining_postpones > 0;

                    let mins = *time / 60;
                    let secs = *time % 60;
                    {
                        use tauri::menu::MenuItemKind;
                        let set_menu_text = |id: &str, text: String| {
                            if let Some(item) = menu_handle.get(id) {
                                if let MenuItemKind::MenuItem(menu_item) = item {
                                    let _ = menu_item.set_text(text);
                                }
                            }
                        };
                        let set_menu_enabled = |id: &str, enabled: bool| {
                            if let Some(item) = menu_handle.get(id) {
                                if let MenuItemKind::MenuItem(menu_item) = item {
                                    let _ = menu_item.set_enabled(enabled);
                                }
                            }
                        };
                        let update_postpone_menu = |text: String, enabled: bool| {
                            if let Some(item) = menu_handle.get("postpone_menu") {
                                if let MenuItemKind::Submenu(submenu) = item {
                                    let _ = submenu.set_text(text);
                                    let _ = submenu.set_enabled(enabled);
                                }
                            }
                        };

                        set_menu_text("timer_info", tray_timer_label(&config, mins, secs));
                        set_menu_text("show", tray_show_label(&config).to_string());
                        set_menu_text("start_break", tray_start_break_label(&config).to_string());
                        set_menu_text("skip_break", tray_skip_break_label(&config).to_string());
                        set_menu_text("postpone_3", tray_postpone_option_label(&config, 3));
                        set_menu_text("postpone_5", tray_postpone_option_label(&config, 5));
                        set_menu_text("postpone_10", tray_postpone_option_label(&config, 10));
                        set_menu_text("quit", tray_quit_label(&config).to_string());
                        set_menu_enabled("skip_break", can_manage_current_break);
                        set_menu_enabled("postpone_3", can_postpone_current_break);
                        set_menu_enabled("postpone_5", can_postpone_current_break);
                        set_menu_enabled("postpone_10", can_postpone_current_break);
                        update_postpone_menu(
                            tray_postpone_menu_label(
                                &config,
                                can_manage_current_break,
                                remaining_postpones,
                            ),
                            can_postpone_current_break,
                        );
                    }

                    if !*paused && *time > 0 {
                        if !break_window_showing
                            && config.natural_breaks_enabled
                            && get_idle_time() > 600
                        {
                            *time = work_duration;
                            reset_break_flow_state(&handle);
                            clear_break_media_state(&handle);
                            close_break_windows(&handle);
                        } else {
                            *time -= 1;
                        }
                        let _ = handle.emit("timer-tick", *time);
                    } else if *time == 0 && !*paused {
                        ensure_break_flow_active(&handle);
                        *time = work_duration;
                        let break_media_state = create_break_media_state(&config);
                        let _ = replace_break_media_state(&handle, break_media_state);

                        if let Ok(monitors) = handle.available_monitors() {
                            for (index, monitor) in monitors.iter().enumerate() {
                                let label = format!("break_window_{}", index);
                                if handle.get_webview_window(&label).is_none() {
                                    let position = monitor.position();
                                    let size = monitor.size();

                                    let window = WebviewWindowBuilder::new(
                                        &handle,
                                        label,
                                        WebviewUrl::App("break.html".into()),
                                    )
                                    .decorations(false)
                                    .transparent(true)
                                    .shadow(false)
                                    .always_on_top(true)
                                    .skip_taskbar(true)
                                    .resizable(false)
                                    .visible(false)
                                    .focused(true)
                                    .build()
                                    .unwrap();

                                    apply_app_icon_to_window(&handle, &window);
                                    let _ = window.set_position(PhysicalPosition::new(position.x, position.y));
                                    let _ = window.set_size(PhysicalSize::new(size.width, size.height));
                                    let _ = window.set_shadow(false);
                                    let _ = window.show();
                                    let _ = window.set_focus();
                                }
                            }
                        }
                    }
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_time_remaining,
            get_app_config,
            update_app_config,
            get_autostart_enabled,
            get_runtime_info,
            check_for_updates,
            install_update,
            get_break_media_state,
            update_break_media_state,
            hide_main_window,
            minimize_main_window,
            toggle_main_fullscreen,
            control_window,
            close_breaks,
            postpone_break,
            reset_break_cycle,
            update_timer_settings,
            get_random_video,
            get_video_count,
            get_video_list
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
