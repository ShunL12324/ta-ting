import { RecordingIndicator } from "./components/RecordingIndicator";
import { SettingsPanel } from "./components/SettingsPanel";
import { UpdateChecker } from "./components/UpdateChecker";
import { useAppStore } from "./stores/appStore";

function App() {
  const { isRecording, state, error } = useAppStore();

  const getStatusConfig = () => {
    switch (state) {
      case 'idle':
        return {
          text: '按 Ctrl+Shift+V 开始听写',
          emoji: '🎙️',
          color: 'text-muted-foreground'
        };
      case 'recording':
        return {
          text: '正在录音...',
          emoji: '🎤',
          color: 'text-destructive'
        };
      case 'transcribing':
        return {
          text: '正在转录...',
          emoji: '✨',
          color: 'text-primary'
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
          color: 'text-muted-foreground'
        };
    }
  };

  const status = getStatusConfig();

  return (
    <div className="min-h-screen bg-background flex items-center justify-center p-8">
      <RecordingIndicator isRecording={isRecording} />
      <SettingsPanel />
      <UpdateChecker />

      <div className="text-center space-y-12 max-w-2xl w-full">
        {/* 标题区域 */}
        <div className="space-y-2 transition-all duration-500">
          <h1 className="text-4xl font-bold tracking-tight text-foreground">
            TaTing
          </h1>
          <p className="text-sm text-muted-foreground mt-1">AI 离线听写输入法</p>
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
          <div className="inline-flex items-center gap-2 px-4 py-2 bg-background/60 backdrop-blur-sm rounded-full border border-border shadow-sm">
            <kbd className="px-2 py-1 bg-muted text-foreground rounded font-mono text-sm font-semibold border border-border">
              Ctrl
            </kbd>
            <span className="text-muted-foreground">+</span>
            <kbd className="px-2 py-1 bg-muted text-foreground rounded font-mono text-sm font-semibold border border-border">
              Shift
            </kbd>
            <span className="text-muted-foreground">+</span>
            <kbd className="px-2 py-1 bg-muted text-foreground rounded font-mono text-sm font-semibold border border-border">
              V
            </kbd>
          </div>

          {/* 错误提示 */}
          {error && (
            <div className="mt-6 p-4 bg-destructive/10 border border-destructive/30 rounded-xl shadow-sm animate-in fade-in slide-in-from-top-2 duration-300">
              <p className="text-sm text-destructive font-medium">{error}</p>
            </div>
          )}
        </div>

        {/* 状态指示器 */}
        <div className="pt-12">
          <div className="inline-flex items-center gap-3 px-5 py-2.5 bg-background/70 backdrop-blur-sm rounded-full shadow-sm border border-border">
            <span className={`
              relative inline-flex w-3 h-3 rounded-full transition-all duration-300
              ${state === 'idle' ? 'bg-green-500' : 'bg-primary'}
            `}>
              {state !== 'idle' && (
                <span className="absolute inline-flex h-full w-full rounded-full bg-primary opacity-75 animate-ping"></span>
              )}
            </span>
            <span className="text-sm font-medium text-muted-foreground">
              {state === 'idle' ? '就绪' : '工作中'}
            </span>
          </div>
        </div>
      </div>
    </div>
  );
}

export default App;
