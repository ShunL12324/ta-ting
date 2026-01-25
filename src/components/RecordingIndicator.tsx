import { useEffect, useState } from 'react';
import { Mic, MicOff } from 'lucide-react';

interface RecordingIndicatorProps {
  isRecording: boolean;
}

export function RecordingIndicator({ isRecording }: RecordingIndicatorProps) {
  const [duration, setDuration] = useState(0);

  useEffect(() => {
    if (!isRecording) {
      setDuration(0);
      return;
    }

    const interval = setInterval(() => {
      setDuration((prev) => prev + 1);
    }, 1000);

    return () => clearInterval(interval);
  }, [isRecording]);

  const formatDuration = (seconds: number) => {
    const mins = Math.floor(seconds / 60);
    const secs = seconds % 60;
    return `${mins}:${secs.toString().padStart(2, '0')}`;
  };

  if (!isRecording) {
    return null; // 不录音时隐藏指示器
  }

  return (
    <div className="fixed top-6 right-6 z-50">
      <div className="relative">
        {/* 脉动背景 */}
        <div className="absolute inset-0 bg-red-500 rounded-2xl animate-pulse opacity-20 blur-xl"></div>

        {/* 主容器 */}
        <div className="relative flex items-center gap-3 px-5 py-3 bg-red-500 text-white rounded-2xl shadow-xl backdrop-blur-sm">
          {/* 录音图标带脉动效果 */}
          <div className="relative">
            <Mic className="w-5 h-5 relative z-10" />
            <div className="absolute inset-0 bg-white rounded-full opacity-30 animate-ping"></div>
          </div>

          {/* 计时器 */}
          <span className="font-mono text-lg font-bold tracking-wider">
            {formatDuration(duration)}
          </span>

          {/* 分隔线 */}
          <div className="w-px h-4 bg-white/30"></div>

          {/* 文字提示 */}
          <span className="text-sm font-medium">正在录音</span>

          {/* 音波动画 */}
          <div className="flex items-center gap-0.5 ml-1">
            <div className="w-0.5 h-2 bg-white rounded-full animate-pulse" style={{ animationDelay: '0ms' }}></div>
            <div className="w-0.5 h-3 bg-white rounded-full animate-pulse" style={{ animationDelay: '150ms' }}></div>
            <div className="w-0.5 h-4 bg-white rounded-full animate-pulse" style={{ animationDelay: '300ms' }}></div>
            <div className="w-0.5 h-3 bg-white rounded-full animate-pulse" style={{ animationDelay: '450ms' }}></div>
          </div>
        </div>
      </div>
    </div>
  );
}
