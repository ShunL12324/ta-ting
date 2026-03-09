export default {
  common: {
    cancel: '取消',
    apply: '应用',
    change: '更改',
    installed: '已安装',
    download: '下载',
  },
  nav: {
    home: '主页',
    settings: '设置',
  },
  home: {
    subtitle: 'AI 离线听写',
    status: {
      idle: '按快捷键开始听写',
      recording: '正在录音',
      transcribing: '正在转录',
      inputting: '正在输入',
    },
  },
  recording: {
    label: '正在录音',
  },
  settings: {
    transcriptionLang: {
      title: '转录语言',
      languages: {
        zh:   { name: '中文',    desc: 'Chinese'           },
        en:   { name: 'English', desc: 'English'           },
        auto: { name: '自动',    desc: 'Chinese & English' },
      },
    },
    uiLang: {
      title: '界面语言',
      options: {
        zh: { label: '中文',    sub: 'Chinese' },
        en: { label: 'English', sub: '英文'   },
      },
    },
    hotkey: {
      title: '全局热键',
      pressCombo: '按下组合键...',
      saveHint: '按"应用"保存新热键',
      modifierHint: '至少需要一个修饰键（Ctrl / Shift / Alt）',
      saved: '✓ 热键已保存',
    },
    behavior: {
      title: '应用行为',
      autostart: '开机自动启动',
      showTranscription: '显示转录过程',
    },
    about: {
      title: '关于',
      subtitle: 'AI 离线听写输入法',
      offline: '离线',
    },
  },
} as const;
