(() => {
  const DEFAULT_LANGUAGE = 'zh-CN'
  const SUPPORTED_LANGUAGES = ['zh-CN', 'en-US']
  const DEFAULT_COPY_STYLE = 'balanced'
  const SUPPORTED_COPY_STYLES = ['balanced', 'tao']

  const MESSAGES = {
    'zh-CN': {
      'app.name': '元气守恒·筑基令',
      'dashboard.documentTitle': '元气守恒·筑基令 (Vitality Keeper)',
      'break.documentTitle': '休息中 - 元气守恒',
      'window.enterFullscreen': '全屏',
      'window.minimize': '最小化',
      'window.hide': '隐藏',
      'nav.home': '总览',
      'nav.schedule': '节律',
      'nav.media': '视频',
      'nav.theme': '外观',
      'nav.system': '系统',
      'cat.sleeping': '守护中，节律稳定',
      'home.heroTitle': '开始今天的专注节律。',
      'home.heroSubtitle': '用一次完整的筑基休息，把久坐和疲惫挡在节律之外。',
      'home.nextBreak': '距下次筑基休息',
      'home.currentMode': '当前模式',
      'home.focusMode': '专注守恒',
      'home.videoCount': '已加载视频',
      'home.guardTitle': '休息守护',
      'home.guardDesc': '设置筑基令触发时的介入强度。',
      'home.strictTitle': '严格模式 (不可跳过)',
      'home.strictDesc': '筑基休息开始后隐藏跳过入口，帮助自己认真完成本轮恢复。',
      'home.fullscreenTitle': '休息时默认全屏',
      'home.fullscreenDesc': '触发筑基令时自动全屏播放，让视频引导更沉浸。',
      'home.dndTitle': '沉浸免打扰探测',
      'home.dndDesc': '当系统正在全屏游戏或播放视频时，暂缓触发休息。',
      'home.naturalTitle': '自然归元 (空闲重置)',
      'home.naturalDesc': '检测到离开电脑或锁屏超过 10 分钟，自动重置当前周期。',
      'schedule.title': '休息节律',
      'schedule.description': '管理专注间隔与单次筑基休息时长。',
      'schedule.workTitle': '专注间隔',
      'schedule.workDesc': '两次筑基休息之间的工作时间 (分钟)',
      'schedule.breakTitle': '单次休息时长',
      'schedule.breakDesc': '固定时长模式下的休息持续时间',
      'media.title': '本地视频引导',
      'media.description': '选择休息时播放的本地拉伸、健身或养生视频。',
      'media.sourceTitle': '视频来源',
      'media.noSource': '尚未选择视频源',
      'media.selectFile': '选择文件',
      'media.selectDirectory': '选择文件夹',
      'media.strategyTitle': '休息结束策略',
      'media.strategyDesc': '选择跟随视频自然结束，或循环播放直到固定休息时长结束。',
      'media.strategyVideo': '视频播完结束',
      'media.strategyFixed': '固定休息时长结束',
      'media.fullscreenModeTitle': '全屏呈现方式',
      'media.fullscreenModeDesc': '选择铺满屏幕或完整显示视频内容。完整显示会保留黑边，更适合超宽屏与竖屏。',
      'media.fullscreenModeCover': '铺满屏幕（可能裁剪）',
      'media.fullscreenModeContain': '完整显示（保留黑边）',
      'media.playlistTitle': '视频列表与时长预览：',
      'media.loading': '正在加载...',
      'media.loadingMetadata': '正在加载视频元数据...',
      'media.noSourcePlaylist': '尚未选择视频源。',
      'media.noneMatched': '未匹配到任何视频文件。',
      'media.readError': '读取视频列表失败，请检查目录权限或资源路径。',
      'media.count': '{count} 部',
      'media.videoDuration': '{filename} [时长: {duration}]',
      'media.videoDurationReadError': '{filename} [无法读取时长]',
      'media.selectDirectoryFailed': '无法打开文件夹选择器',
      'media.selectFileFailed': '无法打开文件选择器',
      'theme.title': '外观主题',
      'theme.description': '切换控制中心的视觉风格。',
      'theme.dao': '数字禅意',
      'theme.light': '白昼极简',
      'system.title': '系统集成',
      'system.description': '管理开机自启、快捷键与语言等系统能力。',
      'system.languageTitle': '界面语言',
      'system.languageDesc': '切换控制台和休息页的显示语言，默认中文。',
      'system.languageZh': '简体中文',
      'system.languageEn': 'English',
      'system.copyStyleTitle': '文案风格',
      'system.copyStyleDesc': '默认保留当前更通用的表达，也可以切换为更接近早期版本、带一点道家气息的归元叙述。',
      'system.copyStyleBalanced': '平衡',
      'system.copyStyleTao': '归元',
      'system.autostartTitle': '开机自启',
      'system.autostartDesc': '系统启动后自动在后台运行',
      'system.shortcutTitle': '全局热键',
      'system.shortcutDesc': '按下 Ctrl + Alt + Q 快速重置休息周期',
      'system.shortcutRegistered': '已启用',
      'system.shortcutUnavailable': '注册失败',
      'system.resetCycleAction': '立即重置',
      'system.versionTitle': '版本与更新',
      'system.versionDesc': '在控制中心查看当前版本，并通过 GitHub Releases 检查和安装更新。',
      'system.currentVersion': '当前版本',
      'system.updateSource': '更新源',
      'system.updateSourceGithub': 'GitHub Releases',
      'system.autoCheckUpdatesTitle': '启动时自动检查更新',
      'system.autoCheckUpdatesDesc': '应用启动后在后台检查 GitHub 上的新版本。',
      'system.checkUpdates': '检查更新',
      'system.installUpdate': '下载并安装',
      'system.updateStatusTitle': '更新状态',
      'system.updateStatusIdle': '尚未检查更新',
      'system.updateStatusChecking': '正在检查 GitHub 上的可用更新...',
      'system.updateStatusAutoChecking': '启动后正在自动检查更新...',
      'system.updateStatusUpToDate': '当前已经是最新版本',
      'system.updateStatusAvailable': '检测到新版本 {version}',
      'system.updateStatusDownloading': '正在下载并准备安装更新...',
      'system.updateStatusInstalling': '更新包已准备就绪，安装程序即将启动。',
      'system.updateStatusError': '更新检查或安装失败',
      'system.updateMetaCurrent': '当前版本：v{version}',
      'system.updateMetaLatest': '最新版本：v{version}',
      'system.updateMetaPublishedAt': '发布时间：{value}',
      'system.updateMetaSource': '更新源：GitHub Releases',
      'system.updateNoNotes': '本次版本未附带发布说明。',
      'system.updateDownloadProgress': '已下载 {percent}%',
      'common.minuteShort': '分',
      'break.headerTitle': '筑基休息开始',
      'break.headerDesc': '现在进入恢复时间。请跟随视频放松身体，给注意力一次完整回充。',
      'break.recoverAudio': '点击屏幕任意位置可恢复声音',
      'break.mute': '静音',
      'break.playPause': '暂停 / 播放',
      'break.toggleFullscreen': '切换全屏',
      'break.postpone': '推迟 5 分钟',
      'break.skip': '跳过本次',
      'break.status': '休息进行中，请专注当下',
      'break.countdownLabel': '剩余时间',
      'break.completion': '本轮休息已完成，状态正在回到最佳节律。',
      'break.finish': '返回工作'
    },
    'en-US': {
      'app.name': 'Vitality Keeper',
      'dashboard.documentTitle': 'Vitality Keeper',
      'break.documentTitle': 'Break Time - Vitality Keeper',
      'window.enterFullscreen': 'Fullscreen',
      'window.minimize': 'Minimize',
      'window.hide': 'Hide',
      'nav.home': 'Home',
      'nav.schedule': 'Schedule',
      'nav.media': 'Media',
      'nav.theme': 'Appearance',
      'nav.system': 'System',
      'cat.sleeping': 'The cat is napping Zzz...',
      'home.heroTitle': 'Start today\'s focus cadence.',
      'home.heroSubtitle': 'Your current rhythm is stable. Keep breathing and stay focused.',
      'home.nextBreak': 'Next Long Break In',
      'home.currentMode': 'Current Mode',
      'home.focusMode': 'Focus Rhythm',
      'home.videoCount': 'Loaded Videos',
      'home.guardTitle': 'Core Guard Rules',
      'home.guardDesc': 'Adjust how strongly the app intervenes during your break cycle.',
      'home.strictTitle': 'Strict Mode (No Skip)',
      'home.strictDesc': 'Once a break starts, block system UI input and remove interruption paths.',
      'home.fullscreenTitle': 'Fullscreen by Default',
      'home.fullscreenDesc': 'Open the break page fullscreen automatically without window borders.',
      'home.dndTitle': 'Immersive DND Detection',
      'home.dndDesc': 'Delay the break when a fullscreen game or video is running.',
      'home.naturalTitle': 'Natural Reset (Idle Reset)',
      'home.naturalDesc': 'Reset the current cycle after 10 minutes of idle time or lock screen.',
      'schedule.title': 'Schedule',
      'schedule.description': 'Manage your focus and recovery cycle.',
      'schedule.workTitle': 'Work Interval',
      'schedule.workDesc': 'Time between long breaks (minutes)',
      'schedule.breakTitle': 'Break Duration',
      'schedule.breakDesc': 'How long the enforced recovery lasts',
      'media.title': 'Media (Local Video Source)',
      'media.description': 'Choose the local exercise or wellness videos to play during breaks.',
      'media.sourceTitle': 'Video Source',
      'media.noSource': 'No video source selected',
      'media.selectFile': 'Choose File',
      'media.selectDirectory': 'Choose Folder',
      'media.strategyTitle': 'Break Finish Strategy',
      'media.strategyDesc': 'End when the video finishes, or loop until the configured break duration ends.',
      'media.strategyVideo': 'End with video',
      'media.strategyFixed': 'End with break duration',
      'media.fullscreenModeTitle': 'Fullscreen Presentation',
      'media.fullscreenModeDesc': 'Choose whether fullscreen playback fills the screen or preserves the full video frame with letterboxing.',
      'media.fullscreenModeCover': 'Fill screen (may crop)',
      'media.fullscreenModeContain': 'Show full frame (letterbox)',
      'media.playlistTitle': 'Playlist Preview & Duration Scan:',
      'media.loading': 'Loading...',
      'media.loadingMetadata': 'Loading video metadata...',
      'media.noSourcePlaylist': 'No video source selected yet.',
      'media.noneMatched': 'No video files were found.',
      'media.readError': 'Failed to read the video list. Check folder access or the media path.',
      'media.count': '{count} videos',
      'media.videoDuration': '{filename} [Duration: {duration}]',
      'media.videoDurationReadError': '{filename} [Duration unavailable]',
      'media.selectDirectoryFailed': 'Unable to open the folder picker',
      'media.selectFileFailed': 'Unable to open the file picker',
      'theme.title': 'Appearance',
      'theme.description': 'Switch the workspace tone with one click.',
      'theme.dao': 'Digital Zen',
      'theme.light': 'Minimal Daylight',
      'system.title': 'System',
      'system.description': 'Controls for low-level system integration.',
      'system.languageTitle': 'Interface Language',
      'system.languageDesc': 'Switch the dashboard and break page language. Defaults to Chinese.',
      'system.languageZh': 'Simplified Chinese',
      'system.languageEn': 'English',
      'system.copyStyleTitle': 'Copy Style',
      'system.copyStyleDesc': 'Keep the current neutral tone by default, or switch to a more ritual, Dao-inspired narrative voice.',
      'system.copyStyleBalanced': 'Balanced',
      'system.copyStyleTao': 'Ritual',
      'system.autostartTitle': 'Launch at Startup',
      'system.autostartDesc': 'Wake automatically in the background when the system starts',
      'system.shortcutTitle': 'Global Shortcut',
      'system.shortcutDesc': 'Press Ctrl + Alt + Q to reset the break cycle quickly',
      'system.shortcutRegistered': 'Enabled',
      'system.shortcutUnavailable': 'Registration failed',
      'system.resetCycleAction': 'Reset Now',
      'system.versionTitle': 'Version & Updates',
      'system.versionDesc': 'See the current version and check or install updates from GitHub Releases.',
      'system.currentVersion': 'Current Version',
      'system.updateSource': 'Update Source',
      'system.updateSourceGithub': 'GitHub Releases',
      'system.autoCheckUpdatesTitle': 'Auto-check on launch',
      'system.autoCheckUpdatesDesc': 'Check GitHub for new releases in the background when the app starts.',
      'system.checkUpdates': 'Check for Updates',
      'system.installUpdate': 'Download & Install',
      'system.updateStatusTitle': 'Update Status',
      'system.updateStatusIdle': 'No update check has been run yet.',
      'system.updateStatusChecking': 'Checking GitHub for available updates...',
      'system.updateStatusAutoChecking': 'Running an automatic update check on launch...',
      'system.updateStatusUpToDate': 'This build is already up to date',
      'system.updateStatusAvailable': 'New version {version} is available',
      'system.updateStatusDownloading': 'Downloading and preparing the update...',
      'system.updateStatusInstalling': 'The update package is ready. The installer is about to start.',
      'system.updateStatusError': 'Update check or installation failed',
      'system.updateMetaCurrent': 'Current version: v{version}',
      'system.updateMetaLatest': 'Latest version: v{version}',
      'system.updateMetaPublishedAt': 'Published: {value}',
      'system.updateMetaSource': 'Source: GitHub Releases',
      'system.updateNoNotes': 'No release notes were provided for this version.',
      'system.updateDownloadProgress': '{percent}% downloaded',
      'common.minuteShort': 'min',
      'break.headerTitle': 'Break Triggered',
      'break.headerDesc': 'High workload detected. Recovery mode is now active. Follow the guidance on screen.',
      'break.recoverAudio': 'Click anywhere to restore audio',
      'break.mute': 'Mute',
      'break.playPause': 'Pause / Play',
      'break.toggleFullscreen': 'Toggle fullscreen',
      'break.postpone': 'Postpone 5 min',
      'break.skip': 'Skip',
      'break.status': 'Break in progress. Stay with the moment.',
      'break.countdownLabel': 'Time Left',
      'break.completion': 'Recovery complete. The system is back at its best state.',
      'break.finish': 'Back to work'
    }
  }

  const COPY_STYLE_OVERRIDES = {
    'zh-CN': {
      tao: {
        'cat.sleeping': '玄猫镇守，气息归元',
        'home.heroTitle': '开启今日的行功节律。',
        'home.heroSubtitle': '以一轮筑基调息，化久坐浊气，养神清意定。',
        'home.currentMode': '行持状态',
        'home.focusMode': '守一凝神',
        'home.guardTitle': '筑基护持',
        'home.guardDesc': '设定筑基令临场时的护持力度。',
        'home.strictTitle': '严守戒律（不可跳过）',
        'home.strictDesc': '筑基一起便收束杂念，暂隐跳过入口，务求本轮调息做满。',
        'home.fullscreenTitle': '休息时入定全屏',
        'home.fullscreenDesc': '筑基令触发后自动铺展全屏，让引功影卷更具沉浸感。',
        'home.dndTitle': '避扰观机',
        'home.dndDesc': '若系统正处于全屏行事，暂缓本轮休息，待机缘平顺再启。',
        'home.naturalTitle': '离席归元（空闲重置）',
        'home.naturalDesc': '若离席或锁屏超过 10 分钟，则视作自行调息，当前周期回归初始。',
        'schedule.title': '行功节律',
        'schedule.description': '掌管专注行功的间隔与每轮筑基时长。',
        'schedule.workTitle': '行功间隔',
        'schedule.workDesc': '两轮筑基之间可专注行功的时长（分钟）',
        'schedule.breakTitle': '筑基时长',
        'schedule.breakDesc': '固定时长模式下，本轮调息持续多久',
        'media.title': '本地引功影卷',
        'media.description': '选择休息时播放的拉伸、健身或养生影卷，为调息入门。',
        'media.strategyTitle': '收功策略',
        'media.strategyDesc': '可随影卷自然收功，也可循环行卷直至本轮筑基时长圆满。',
        'media.strategyVideo': '影卷播毕收功',
        'media.strategyFixed': '满时长后收功',
        'system.copyStyleDesc': '默认保留当前更通用的表达，也可以切换为更接近早期版本、带一点道家气息的归元叙述。',
        'system.shortcutDesc': '按下 Ctrl + Alt + Q，可令本轮节律归元重启。',
        'system.resetCycleAction': '归元重置',
        'break.headerTitle': '筑基调息已启',
        'break.headerDesc': '此刻当收心敛息。请随引功影卷舒展筋骨，让神意缓缓归定。',
        'break.postpone': '推迟五分钟',
        'break.skip': '略过此轮',
        'break.status': '调息行进中，请安心守神',
        'break.countdownLabel': '尚余时刻',
        'break.completion': '本轮筑基已圆满，神气正缓缓回归清明。',
        'break.finish': '回到行功'
      }
    },
    'en-US': {
      tao: {
        'home.heroTitle': 'Enter today’s ritual cadence.',
        'home.heroSubtitle': 'Use one complete recovery ritual to clear the weight of long sitting and restore attention.',
        'home.focusMode': 'Stillness Practice',
        'home.guardTitle': 'Ritual Guard',
        'break.headerTitle': 'Recovery Ritual Begun',
        'break.status': 'The ritual is underway. Stay with the breath.',
        'system.shortcutDesc': 'Press Ctrl + Alt + Q to return the current cadence to its starting point.'
      }
    }
  }

  function normalizeLanguage(value) {
    return SUPPORTED_LANGUAGES.includes(value) ? value : DEFAULT_LANGUAGE
  }

  function normalizeCopyStyle(value) {
    return SUPPORTED_COPY_STYLES.includes(value) ? value : DEFAULT_COPY_STYLE
  }

  function interpolate(template, params = {}) {
    return String(template).replace(/\{(\w+)\}/g, (_, key) => {
      return params[key] === undefined ? '' : String(params[key])
    })
  }

  function t(key, language = DEFAULT_LANGUAGE, params = {}, copyStyle = DEFAULT_COPY_STYLE) {
    const normalizedLanguage = normalizeLanguage(language)
    const normalizedCopyStyle = normalizeCopyStyle(copyStyle)
    const messages = MESSAGES[normalizedLanguage] || MESSAGES[DEFAULT_LANGUAGE]
    const overrides = COPY_STYLE_OVERRIDES[normalizedLanguage]?.[normalizedCopyStyle] || {}
    const fallback = MESSAGES[DEFAULT_LANGUAGE][key] || key
    return interpolate(overrides[key] || messages[key] || fallback, params)
  }

  function applyI18n(language, root = document, copyStyle = DEFAULT_COPY_STYLE) {
    const normalizedLanguage = normalizeLanguage(language)
    const normalizedCopyStyle = normalizeCopyStyle(copyStyle)
    document.documentElement.lang = normalizedLanguage

    root.querySelectorAll('[data-i18n]').forEach((element) => {
      element.textContent = t(element.getAttribute('data-i18n'), normalizedLanguage, {}, normalizedCopyStyle)
    })

    root.querySelectorAll('[data-i18n-title]').forEach((element) => {
      element.setAttribute('title', t(element.getAttribute('data-i18n-title'), normalizedLanguage, {}, normalizedCopyStyle))
    })

    root.querySelectorAll('[data-i18n-placeholder]').forEach((element) => {
      element.setAttribute('placeholder', t(element.getAttribute('data-i18n-placeholder'), normalizedLanguage, {}, normalizedCopyStyle))
    })

    root.querySelectorAll('[data-i18n-aria-label]').forEach((element) => {
      element.setAttribute('aria-label', t(element.getAttribute('data-i18n-aria-label'), normalizedLanguage, {}, normalizedCopyStyle))
    })

    return normalizedLanguage
  }

  window.vitalityI18n = {
    defaultLanguage: DEFAULT_LANGUAGE,
    defaultCopyStyle: DEFAULT_COPY_STYLE,
    normalizeLanguage,
    normalizeCopyStyle,
    applyI18n,
    t
  }
})()
