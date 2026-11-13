(async function () {
  'use strict';

  const state = {
    motorEnabled: true,
    bubbleVisible: true,
    sensoryCalm: false,
    contrastBoost: false,
    cognitiveGloss: true,
    tremorStrength: 'balanced',
    maxMagneticRadius: 65.0,
  };

  if (typeof chrome !== 'undefined' && chrome.storage && chrome.storage.local) {
    const saved = await chrome.storage.local.get(null);
    Object.assign(state, saved);
  }

  const engine = new window.CentennialEngine();
  try {
    const wasmUrl = chrome.runtime.getURL('wasm/engine_core.wasm');
    await engine.init(wasmUrl);
  } catch (err) {
    return;
  }

  let bubbleEl = document.getElementById('steadyclick-bubble-cursor');
  if (!bubbleEl) {
    bubbleEl = document.createElement('div');
    bubbleEl.id = 'steadyclick-bubble-cursor';
    document.documentElement.appendChild(bubbleEl);
  }

  let targetCache = [];
  let elementMap = new Map();
  let nextElementId = 1;
  let currentlyCapturedEl = null;

  function refreshInteractiveTargets() {
    if (!state.motorEnabled) return;

    const selector = [
      'a[href]',
      'button:not([disabled])',
      'input:not([disabled])',
      'select:not([disabled])',
      'textarea:not([disabled])',
      '[role="button"]',
      '[role="link"]',
      '[role="checkbox"]',
      '[tabindex]:not([tabindex="-1"])',
    ].join(', ');

    const elements = document.querySelectorAll(selector);
    const newCache = [];
    const newMap = new Map();

    const viewportWidth = window.innerWidth;
    const viewportHeight = window.innerHeight;

    elements.forEach((el) => {
      const rect = el.getBoundingClientRect();
      if (
        rect.width > 0 &&
        rect.height > 0 &&
        rect.bottom >= -20 &&
        rect.top <= viewportHeight + 20 &&
        rect.right >= -20 &&
        rect.left <= viewportWidth + 20
      ) {
        let id = el.__steadyclick_id;
        if (!id) {
          id = nextElementId++;
          el.__steadyclick_id = id;
        }

        newMap.set(id, el);
        newCache.push({
          id,
          x0: rect.left,
          y0: rect.top,
          x1: rect.right,
          y1: rect.bottom,
        });
      }
    });

    targetCache = newCache;
    elementMap = newMap;
  }

  window.addEventListener('scroll', refreshInteractiveTargets, { passive: true });
  window.addEventListener('resize', refreshInteractiveTargets, { passive: true });
  setInterval(refreshInteractiveTargets, 1000);
  refreshInteractiveTargets();

  window.addEventListener(
    'pointermove',
    (e) => {
      if (!state.motorEnabled) {
        if (bubbleEl) bubbleEl.style.display = 'none';
        return;
      }

      const nowSec = performance.now() / 1000.0;
      const filtered = engine.filterCursor(e.clientX, e.clientY, nowSec);

      const res = engine.queryVoronoi(
        filtered.x,
        filtered.y,
        targetCache,
        state.maxMagneticRadius
      );

      if (state.bubbleVisible) {
        bubbleEl.style.display = 'block';
        bubbleEl.style.left = `${filtered.x}px`;
        bubbleEl.style.top = `${filtered.y}px`;

        const radius = Math.min(
          Math.max(res.distPrimary + 8, 14),
          state.maxMagneticRadius
        );
        bubbleEl.style.width = `${radius * 2}px`;
        bubbleEl.style.height = `${radius * 2}px`;
      } else {
        bubbleEl.style.display = 'none';
      }

      if (res.capturedId !== null && res.distPrimary <= state.maxMagneticRadius) {
        const targetEl = elementMap.get(res.capturedId);
        if (targetEl && targetEl !== currentlyCapturedEl) {
          if (currentlyCapturedEl) {
            currentlyCapturedEl.classList.remove('steadyclick-target-active');
          }
          targetEl.classList.add('steadyclick-target-active');
          currentlyCapturedEl = targetEl;
        }
      } else if (currentlyCapturedEl) {
        currentlyCapturedEl.classList.remove('steadyclick-target-active');
        currentlyCapturedEl = null;
      }
    },
    { passive: true }
  );

  window.addEventListener(
    'click',
    (e) => {
      if (!state.motorEnabled || !currentlyCapturedEl) return;

      if (!currentlyCapturedEl.contains(e.target) && e.target !== currentlyCapturedEl) {
        e.preventDefault();
        e.stopPropagation();
        currentlyCapturedEl.click();
        currentlyCapturedEl.focus();
      }
    },
    true
  );

  function applySensoryCalm() {
    if (state.sensoryCalm) {
      document.body.classList.add('steadyclick-sensory-calm');
      // Do not interrupt the active video player if user is actively watching YouTube
      const isYouTubeWatch = window.location.hostname.includes('youtube.com') && window.location.pathname.includes('/watch');
      if (!isYouTubeWatch) {
        document.querySelectorAll('video[autoplay][muted], video.ytd-video-preview').forEach((media) => {
          try { media.pause(); } catch (_) {}
        });
      }
    } else {
      document.body.classList.remove('steadyclick-sensory-calm');
    }
  }

  function parseRgb(str) {
    if (!str || str === 'transparent') return null;
    const match = str.match(/rgba?\((\d+),\s*(\d+),\s*(\d+)(?:,\s*([\d.]+))?\)/);
    if (match) {
      return {
        r: parseInt(match[1], 10),
        g: parseInt(match[2], 10),
        b: parseInt(match[3], 10),
        a: match[4] !== undefined ? parseFloat(match[4]) : 1.0,
      };
    }
    return null;
  }

  function getEffectiveBackgroundColor(el) {
    let current = el;
    while (current && current !== document.documentElement) {
      const bg = window.getComputedStyle(current).backgroundColor;
      const parsed = parseRgb(bg);
      if (parsed && parsed.a > 0.1) {
        return parsed;
      }
      current = current.parentElement;
    }
    const bodyBg = window.getComputedStyle(document.body || document.documentElement).backgroundColor;
    const parsedBody = parseRgb(bodyBg);
    if (parsedBody && parsedBody.a > 0.1) return parsedBody;

    const isDark = window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches;
    return isDark ? { r: 18, g: 18, b: 18, a: 1 } : { r: 255, g: 255, b: 255, a: 1 };
  }

  function applyAPCAContrastBoost() {
    if (!state.contrastBoost) {
      document.querySelectorAll('.steadyclick-contrast-dark, .steadyclick-contrast-light').forEach((el) => {
        el.classList.remove('steadyclick-contrast-dark', 'steadyclick-contrast-light');
      });
      return;
    }

    const textNodes = document.querySelectorAll('p, h1, h2, h3, h4, h5, h6, span, a, yt-formatted-string');
    textNodes.forEach((node) => {
      if (node.children.length > 2) return;

      const style = window.getComputedStyle(node);
      const color = parseRgb(style.color);
      const bg = getEffectiveBackgroundColor(node);

      if (color && bg) {
        const lc = engine.evaluateAPCA(color.r, color.g, color.b, bg.r, bg.g, bg.b);
        if (Math.abs(lc) < 55) {
          const bgLum = 0.2126 * bg.r + 0.7152 * bg.g + 0.0722 * bg.b;
          if (bgLum > 128) {
            node.classList.remove('steadyclick-contrast-dark');
            node.classList.add('steadyclick-contrast-light');
          } else {
            node.classList.remove('steadyclick-contrast-light');
            node.classList.add('steadyclick-contrast-dark');
          }
        }
      }
    });
  }

  function applyCognitiveGloss() {
    if (!state.cognitiveGloss) return;

    const paragraphs = document.querySelectorAll('p');
    paragraphs.forEach((p) => {
      if (p.__steadyclick_glossed) return;
      p.__steadyclick_glossed = true;

      const text = p.innerText;
      if (text.length > 500) return;

      const words = text.split(/\s+/);
      let unfamiliarFound = false;

      words.forEach((w) => {
        const cleanWord = w.replace(/[^a-zA-Z]/g, '').toLowerCase();
        if (cleanWord.length > 6 && !engine.isFamiliarWord(cleanWord)) {
          unfamiliarFound = true;
        }
      });

      if (unfamiliarFound) {
        p.title = 'Contains specialized or technical language';
      }
    });
  }

  applySensoryCalm();
  if (state.contrastBoost) applyAPCAContrastBoost();
  if (state.cognitiveGloss) applyCognitiveGloss();

  if (typeof chrome !== 'undefined' && chrome.storage) {
    chrome.storage.onChanged.addListener((changes) => {
      for (const [key, change] of Object.entries(changes)) {
        state[key] = change.newValue;
      }
      applySensoryCalm();
      applyAPCAContrastBoost();
      if (!state.motorEnabled && bubbleEl) {
        bubbleEl.style.display = 'none';
        if (currentlyCapturedEl) {
          currentlyCapturedEl.classList.remove('steadyclick-target-active');
        }
      }
    });
  }
})();
