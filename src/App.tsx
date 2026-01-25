import { RecordingIndicator } from "./components/RecordingIndicator";
import { SettingsPanel } from "./components/SettingsPanel";
import { useAppStore } from "./stores/appStore";

function App() {
  const { isRecording, state, error } = useAppStore();

  const getStatusConfig = () => {
    switch (state) {
      case 'idle':
        return {
          text: '按 Ctrl+Shift+D 开始听写',
          emoji: '🎙️',
          color: 'text-gray-700'
        };
      case 'recording':
        return {
          text: '正在录音...',
          emoji: '🎤',
          color: 'text-red-600'
        };
      case 'transcribing':
        return {
          text: '正在转录...',
          emoji: '✨',
          color: 'text-blue-600'
        };
      case 'inputting':
        return {
          text: '正在输入...',
          emoji: '⌨️',
          color: 'text-green-600'
        };
      default:
        return {
          text: '就绪',
          emoji: '🎙️',
          color: 'text-gray-700'
        };
    }
  };

  const status = getStatusConfig();

  return (
    <div className="min-h-screen bg-gradient-to-br from-blue-50 via-indigo-50 to-purple-50 flex items-center justify-center p-8">
      <RecordingIndicator isRecording={isRecording} />
      <SettingsPanel />

      <div className="text-center space-y-12 max-w-2xl w-full">
        {/* 标题区域 */}
        <div className="space-y-4 transition-all duration-500">
          <h1 className="text-7xl font-black text-transparent bg-clip-text bg-gradient-to-r from-blue-600 to-indigo-600 tracking-tight">
            TaTing
          </h1>
          <p className="text-2xl text-gray-600 font-light">AI 离线听写输入法</p>
        </div>

        {/* 状态提示区域 */}
        <div className="space-y-6">
          <div className={`
            text-3xl font-semibold transition-all duration-300
            ${status.color}
          `}>
            <span className="inline-block mr-3 transition-transform duration-300 hover:scale-110">
              {status.emoji}
            </span>
            <span className="transition-opacity duration-300">
              {status.text}
            </span>
          </div>

          {/* 快捷键提示 */}
          <div className="inline-flex items-center gap-2 px-4 py-2 bg-white/60 backdrop-blur-sm rounded-full border border-gray-200 shadow-sm">
            <kbd className="px-2 py-1 bg-gray-100 text-gray-700 rounded font-mono text-sm font-semibold">
              Ctrl
            </kbd>
            <span className="text-gray-400">+</span>
            <kbd className="px-2 py-1 bg-gray-100 text-gray-700 rounded font-mono text-sm font-semibold">
              Shift
            </kbd>
            <span className="text-gray-400">+</span>
            <kbd className="px-2 py-1 bg-gray-100 text-gray-700 rounded font-mono text-sm font-semibold">
              D
            </kbd>
          </div>

          {/* 错误提示 */}
          {error && (
            <div className="mt-6 p-4 bg-red-50 border border-red-200 rounded-xl shadow-sm animate-in fade-in slide-in-from-top-2 duration-300">
              <p className="text-sm text-red-700 font-medium">{error}</p>
            </div>
          )}
        </div>

        {/* 状态指示器 */}
        <div className="pt-12">
          <div className="inline-flex items-center gap-3 px-5 py-2.5 bg-white/70 backdrop-blur-sm rounded-full shadow-sm border border-gray-200">
            <span className={`
              relative inline-flex w-3 h-3 rounded-full transition-all duration-300
              ${state === 'idle' ? 'bg-green-500' : 'bg-blue-500'}
            `}>
              {state !== 'idle' && (
                <span className="absolute inline-flex h-full w-full rounded-full bg-blue-400 opacity-75 animate-ping"></span>
              )}
            </span>
            <span className="text-sm font-medium text-gray-700">
              {state === 'idle' ? '就绪' : '工作中'}
            </span>
          </div>
        </div>
      </div>
    </div>
  );
}

export default App;
