export default {
  common: {
    cancel: 'Cancel',
    apply: 'Apply',
    change: 'Change',
    installed: 'Installed',
    download: 'Download',
  },
  nav: {
    home: 'Home',
    settings: 'Settings',
  },
  home: {
    subtitle: 'Offline AI Dictation',
    status: {
      idle: 'Press hotkey to start dictation',
      recording: 'Recording',
      transcribing: 'Transcribing',
      inputting: 'Typing',
    },
  },
  recording: {
    label: 'Recording',
  },
  settings: {
    transcriptionLang: {
      title: 'Transcription Language',
      languages: {
        zh:   { name: 'Chinese',  desc: '中文'             },
        en:   { name: 'English',  desc: 'English'          },
        auto: { name: 'Auto',     desc: 'Chinese & English'},
      },
    },
    uiLang: {
      title: 'Display Language',
      options: {
        zh: { label: '中文',    sub: 'Chinese' },
        en: { label: 'English', sub: '英文'   },
      },
    },
    hotkey: {
      title: 'Global Hotkey',
      pressCombo: 'Press a key combination...',
      saveHint: 'Click "Apply" to save the new hotkey',
      modifierHint: 'At least one modifier key required (Ctrl / Shift / Alt)',
      saved: '✓ Hotkey saved',
    },
    behavior: {
      title: 'App Behavior',
      autostart: 'Launch at startup',
      showTranscription: 'Show transcription progress',
    },
    about: {
      title: 'About',
      subtitle: 'Offline AI Dictation IME',
      offline: 'Offline',
    },
  },
} as const;
