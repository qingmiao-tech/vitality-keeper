(() => {
  const DEFAULT_CONFIG = Object.freeze({
    strictMode: false,
    fullscreen: true,
    fullscreenFitMode: 'cover',
    autostartEnabled: true,
    dndEnabled: false,
    naturalBreaksEnabled: true,
    workMinutes: 45,
    breakMinutes: 5,
    videoSource: '',
    breakStrategy: 'video',
    disabledVideos: [],
    customVideoOrder: [],
    theme: 'dao',
    language: 'zh-CN',
    copyStyle: 'balanced',
    autoCheckUpdatesEnabled: true
  })

  let cachedConfig = null

  function cloneConfig(value) {
    return JSON.parse(JSON.stringify(value))
  }

  function normalizeConfig(value = {}) {
    return {
      strictMode: value.strictMode === true,
      fullscreen: value.fullscreen !== false,
      fullscreenFitMode: value.fullscreenFitMode === 'contain' ? 'contain' : DEFAULT_CONFIG.fullscreenFitMode,
      autostartEnabled: value.autostartEnabled !== false,
      dndEnabled: value.dndEnabled === true,
      naturalBreaksEnabled: value.naturalBreaksEnabled !== false,
      workMinutes: Number.isFinite(Number(value.workMinutes)) ? Number(value.workMinutes) : DEFAULT_CONFIG.workMinutes,
      breakMinutes: Number.isFinite(Number(value.breakMinutes)) ? Number(value.breakMinutes) : DEFAULT_CONFIG.breakMinutes,
      videoSource: typeof value.videoSource === 'string' ? value.videoSource : DEFAULT_CONFIG.videoSource,
      breakStrategy: value.breakStrategy === 'fixed' ? 'fixed' : 'video',
      disabledVideos: Array.isArray(value.disabledVideos) ? [...value.disabledVideos] : [],
      customVideoOrder: Array.isArray(value.customVideoOrder) ? [...value.customVideoOrder] : [],
      theme: value.theme === 'light' ? 'light' : 'dao',
      language: value.language === 'en-US' ? 'en-US' : DEFAULT_CONFIG.language,
      copyStyle: value.copyStyle === 'tao' ? 'tao' : DEFAULT_CONFIG.copyStyle,
      autoCheckUpdatesEnabled: value.autoCheckUpdatesEnabled !== false
    }
  }

  async function loadConfig(forceReload = false) {
    if (!forceReload && cachedConfig) {
      return cloneConfig(cachedConfig)
    }

    const loaded = await window.__TAURI__.core.invoke('get_app_config')
    cachedConfig = normalizeConfig(loaded)
    return cloneConfig(cachedConfig)
  }

  async function saveConfig(patch = {}) {
    const current = cachedConfig ?? (await loadConfig())
    const nextConfig = normalizeConfig({ ...current, ...patch })
    const saved = await window.__TAURI__.core.invoke('update_app_config', {
      config: nextConfig
    })

    cachedConfig = normalizeConfig(saved)
    return cloneConfig(cachedConfig)
  }

  function getCachedConfig() {
    return cachedConfig ? cloneConfig(cachedConfig) : cloneConfig(DEFAULT_CONFIG)
  }

  window.vitalityConfig = {
    defaults: cloneConfig(DEFAULT_CONFIG),
    loadConfig,
    saveConfig,
    getCachedConfig
  }
})()
