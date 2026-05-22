(function () {
  const ICONS = {
    'cat': '<circle cx="12" cy="12" r="3"/><path d="M7 8 5 4l4 2"/><path d="m17 8 2-4-4 2"/><path d="M7 14c1 4 9 4 10 0"/><path d="M5 12h3"/><path d="M16 12h3"/>',
    'chevron-down': '<path d="m6 9 6 6 6-6"/>',
    'clock': '<circle cx="12" cy="12" r="9"/><path d="M12 7v5l3 2"/>',
    'film': '<rect x="3" y="5" width="18" height="14" rx="2"/><path d="M7 5v14"/><path d="M17 5v14"/><path d="M3 9h4"/><path d="M17 9h4"/><path d="M3 15h4"/><path d="M17 15h4"/>',
    'layout-dashboard': '<rect x="3" y="3" width="7" height="9" rx="1"/><rect x="14" y="3" width="7" height="5" rx="1"/><rect x="14" y="12" width="7" height="9" rx="1"/><rect x="3" y="16" width="7" height="5" rx="1"/>',
    'maximize': '<path d="M8 3H5a2 2 0 0 0-2 2v3"/><path d="M16 3h3a2 2 0 0 1 2 2v3"/><path d="M21 16v3a2 2 0 0 1-2 2h-3"/><path d="M8 21H5a2 2 0 0 1-2-2v-3"/>',
    'minimize': '<path d="M8 3v3a2 2 0 0 1-2 2H3"/><path d="M16 3v3a2 2 0 0 0 2 2h3"/><path d="M21 16h-3a2 2 0 0 0-2 2v3"/><path d="M3 16h3a2 2 0 0 1 2 2v3"/>',
    'palette': '<circle cx="13.5" cy="6.5" r=".5"/><circle cx="17.5" cy="10.5" r=".5"/><circle cx="8.5" cy="7.5" r=".5"/><circle cx="6.5" cy="12.5" r=".5"/><path d="M12 3a9 9 0 0 0 0 18h1.5a2.5 2.5 0 0 0 0-5H12a2 2 0 0 1 0-4h2a7 7 0 0 0 0-9z"/>',
    'pause': '<rect x="6" y="4" width="4" height="16" rx="1"/><rect x="14" y="4" width="4" height="16" rx="1"/>',
    'play': '<path d="m8 5 12 7-12 7z"/>',
    'play-circle': '<circle cx="12" cy="12" r="10"/><path d="m10 8 6 4-6 4z"/>',
    'settings': '<path d="M12 15.5A3.5 3.5 0 1 0 12 8a3.5 3.5 0 0 0 0 7.5z"/><path d="M19.4 15a1.7 1.7 0 0 0 .3 1.9l.1.1a2 2 0 1 1-2.8 2.8l-.1-.1a1.7 1.7 0 0 0-1.9-.3 1.7 1.7 0 0 0-1 1.6V21a2 2 0 1 1-4 0v-.1a1.7 1.7 0 0 0-1-1.6 1.7 1.7 0 0 0-1.9.3l-.1.1a2 2 0 1 1-2.8-2.8l.1-.1a1.7 1.7 0 0 0 .3-1.9 1.7 1.7 0 0 0-1.6-1H3a2 2 0 1 1 0-4h.1a1.7 1.7 0 0 0 1.6-1 1.7 1.7 0 0 0-.3-1.9l-.1-.1A2 2 0 1 1 7.1 4l.1.1a1.7 1.7 0 0 0 1.9.3h.1a1.7 1.7 0 0 0 1-1.6V3a2 2 0 1 1 4 0v.1a1.7 1.7 0 0 0 1 1.6 1.7 1.7 0 0 0 1.9-.3l.1-.1A2 2 0 1 1 20 7.1l-.1.1a1.7 1.7 0 0 0-.3 1.9v.1a1.7 1.7 0 0 0 1.6 1H21a2 2 0 1 1 0 4h-.1a1.7 1.7 0 0 0-1.5.8z"/>',
    'shield': '<path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z"/>',
    'skip-forward': '<path d="m5 4 10 8-10 8z"/><path d="M19 5v14"/>',
    'volume-2': '<path d="M11 5 6 9H3v6h3l5 4z"/><path d="M16 8.5a5 5 0 0 1 0 7"/><path d="M19 5a9 9 0 0 1 0 14"/>',
    'volume-x': '<path d="M11 5 6 9H3v6h3l5 4z"/><path d="m16 9 5 5"/><path d="m21 9-5 5"/>'
  }

  function renderIconElement(element) {
    const name = element.getAttribute('data-lucide')
    const paths = ICONS[name]
    if (!paths) {
      return
    }

    const svg = document.createElementNS('http://www.w3.org/2000/svg', 'svg')
    svg.setAttribute('xmlns', 'http://www.w3.org/2000/svg')
    svg.setAttribute('viewBox', '0 0 24 24')
    svg.setAttribute('width', element.style.width || '24')
    svg.setAttribute('height', element.style.height || '24')
    svg.setAttribute('fill', 'none')
    svg.setAttribute('stroke', 'currentColor')
    svg.setAttribute('stroke-width', '2')
    svg.setAttribute('stroke-linecap', 'round')
    svg.setAttribute('stroke-linejoin', 'round')
    svg.setAttribute('aria-hidden', 'true')
    svg.innerHTML = paths
    element.replaceWith(svg)
  }

  function createIcons(options = {}) {
    const root = options.root || document
    root.querySelectorAll('i[data-lucide]').forEach(renderIconElement)
  }

  window.vitalityIcons = { createIcons }

  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', () => createIcons(), { once: true })
  } else {
    createIcons()
  }
})()
