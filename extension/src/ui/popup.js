document.addEventListener('DOMContentLoaded', async () => {
  const fields = [
    'motorEnabled',
    'bubbleVisible',
    'contrastBoost',
    'sensoryCalm',
    'cognitiveGloss',
  ];

  if (typeof chrome === 'undefined' || !chrome.storage || !chrome.storage.local) {
    return;
  }

  const data = await chrome.storage.local.get([...fields, 'tremorStrength']);

  fields.forEach((field) => {
    const el = document.getElementById(field);
    if (el && data[field] !== undefined) {
      el.checked = data[field];
    }
  });

  const currentStrength = data.tremorStrength || 'balanced';
  updateStrengthSelection(currentStrength);

  fields.forEach((field) => {
    const el = document.getElementById(field);
    if (el) {
      el.addEventListener('change', () => {
        chrome.storage.local.set({ [field]: el.checked });
      });
    }
  });

  const strengthButtons = document.querySelectorAll('#strengthGroup .sub-btn');
  strengthButtons.forEach((btn) => {
    btn.addEventListener('click', () => {
      const val = btn.getAttribute('data-strength');
      updateStrengthSelection(val);
      chrome.storage.local.set({ tremorStrength: val });
    });
  });

  document.getElementById('presetMotor')?.addEventListener('click', () => {
    applyPreset({
      motorEnabled: true,
      bubbleVisible: true,
      contrastBoost: false,
      sensoryCalm: false,
      cognitiveGloss: false,
    });
  });

  document.getElementById('presetReading')?.addEventListener('click', () => {
    applyPreset({
      motorEnabled: false,
      bubbleVisible: false,
      contrastBoost: true,
      sensoryCalm: true,
      cognitiveGloss: true,
    });
  });

  document.getElementById('presetAll')?.addEventListener('click', () => {
    applyPreset({
      motorEnabled: true,
      bubbleVisible: true,
      contrastBoost: true,
      sensoryCalm: true,
      cognitiveGloss: true,
    });
  });

  function updateStrengthSelection(val) {
    const buttons = document.querySelectorAll('#strengthGroup .sub-btn');
    buttons.forEach((btn) => {
      if (btn.getAttribute('data-strength') === val) {
        btn.classList.add('active');
      } else {
        btn.classList.remove('active');
      }
    });
  }

  function applyPreset(values) {
    for (const [k, v] of Object.entries(values)) {
      const el = document.getElementById(k);
      if (el) el.checked = v;
    }
    chrome.storage.local.set(values);
  }
});
