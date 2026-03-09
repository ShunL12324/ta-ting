import { useEffect, useRef, useState } from 'react';
import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';
import { Microphone } from '@phosphor-icons/react';

export function RecordingWindow() {
  const [duration, setDuration] = useState(0);
  const [waveformData, setWaveformData] = useState<number[]>(new Array(30).fill(0));
  const [debugLog, setDebugLog] = useState<string[]>([]);
  const canvasRef = useRef<HTMLCanvasElement>(null);

  // 用于平滑过渡的前一帧数据
  const prevDataRef = useRef<number[]>(new Array(30).fill(0));

  const addLog = (msg: string) => {
    console.log(msg);
    setDebugLog(prev => [...prev.slice(-5), `${new Date().toLocaleTimeString()}: ${msg}`]);
  };

  // 立即输出，确认组件加载
  useEffect(() => {
    addLog('🔴 组件挂载');
  }, []);

  useEffect(() => {
    addLog('🟢 useEffect 触发');

    // 计时器
    const timer = setInterval(() => {
      setDuration((prev) => prev + 1);
    }, 1000);

    // 获取当前窗口并监听音频数据
    const currentWindow = getCurrentWebviewWindow();
    addLog(`🔵 窗口: ${currentWindow.label}`);

    const setupListener = async () => {
      try {
        const unlisten = await currentWindow.listen<number[]>('audio_data', (event) => {
          const samples = event.payload;
          const barCount = 30;

          let newData: number[];

          if (samples.length >= barCount) {
            // 如果采样点足够多，直接分段取最大值
            newData = Array.from({ length: barCount }, (_, i) => {
              const startIdx = Math.floor(i * samples.length / barCount);
              const endIdx = Math.floor((i + 1) * samples.length / barCount);
              const segment = samples.slice(startIdx, endIdx);
              return segment.length > 0 ? Math.max(...segment.map(Math.abs)) : 0;
            });
          } else {
            // 如果采样点不够，用线性插值扩展
            newData = Array.from({ length: barCount }, (_, i) => {
              const pos = (i / (barCount - 1)) * (samples.length - 1);
              const idx = Math.floor(pos);
              const frac = pos - idx;

              if (idx >= samples.length - 1) {
                return Math.abs(samples[samples.length - 1] || 0);
              }

              // 线性插值
              const val = Math.abs(samples[idx]) * (1 - frac) + Math.abs(samples[idx + 1]) * frac;
              return val;
            });
          }

          // 平滑处理：让数值缓慢过渡而不是突变
          const smoothing = 0.85; // 提高到 0.85，更柔和（保留85%旧值 + 15%新值）
          const smoothedData = newData.map((newVal, i) => {
            const oldVal = prevDataRef.current[i];
            return oldVal * smoothing + newVal * (1 - smoothing);
          });

          prevDataRef.current = smoothedData;
          setWaveformData(smoothedData);
        });

        addLog('✅ 监听器已设置');
        return unlisten;
      } catch (error) {
        addLog(`❌ 错误: ${error}`);
        return () => {};
      }
    };

    let unlisten: (() => void) | null = null;
    setupListener().then((fn) => {
      unlisten = fn;
      addLog('🎯 监听器就绪');
    });

    return () => {
      addLog('🔴 组件卸载');
      clearInterval(timer);
      if (unlisten) {
        unlisten();
      }
    };
  }, []);

  useEffect(() => {
    // 绘制波形 - 优雅的对称波形动画
    const canvas = canvasRef.current;
    if (!canvas) return;

    const ctx = canvas.getContext('2d');
    if (!ctx) return;

    const width = canvas.width;
    const height = canvas.height;

    // 清空画布
    ctx.clearRect(0, 0, width, height);

    const barCount = waveformData.length;
    const barWidth = width / barCount;
    const centerY = height / 2;

    // 添加发光效果
    ctx.shadowBlur = 4;
    ctx.shadowColor = 'rgba(255, 255, 255, 0.5)';

    waveformData.forEach((value, index) => {
      // 增加振幅缩放，限制最大高度为画布高度的90%（留出空间给圆角）
      const scaledValue = Math.min(value * 50, 1.0);
      const maxBarHeight = height * 0.45; // 单侧最大高度（总共90%）
      const barHeight = Math.max(4, scaledValue * maxBarHeight);

      const x = index * barWidth;

      // 动态调整圆角半径：小柱子用小圆角，大柱子用大圆角
      const radius = Math.min(3, barHeight * 0.3);

      // 创建上下对称的渐变色
      const gradient = ctx.createLinearGradient(x, centerY - barHeight, x, centerY + barHeight);
      gradient.addColorStop(0, 'rgba(255, 255, 255, 0.9)');
      gradient.addColorStop(0.5, 'rgba(255, 255, 255, 1)');
      gradient.addColorStop(1, 'rgba(255, 255, 255, 0.9)');
      ctx.fillStyle = gradient;

      // 绘制完整的圆角矩形（从上到下）
      ctx.beginPath();
      ctx.roundRect(
        x + 1,
        centerY - barHeight,
        barWidth - 2,
        barHeight * 2,
        radius
      );
      ctx.fill();
    });

    // 重置阴影
    ctx.shadowBlur = 0;
  }, [waveformData]);

  const formatDuration = (seconds: number) => {
    const mins = Math.floor(seconds / 60);
    const secs = seconds % 60;
    return `${mins}:${secs.toString().padStart(2, '0')}`;
  };

  return (
    <div className="w-full h-full p-2.5 flex flex-col gap-2">
      {/* 调试日志 */}
      <div className="bg-black/80 text-white text-xs p-2 rounded max-h-20 overflow-y-auto">
        {debugLog.map((log, i) => <div key={i}>{log}</div>)}
      </div>

      {/* 录音条 */}
      <div className="flex-1 flex items-center justify-center">
        <div className="
          w-full h-full
          flex items-center gap-3 px-4 py-2
          bg-destructive
          rounded-full
          shadow-2xl
          animate-in fade-in slide-in-from-bottom-2 duration-300
        ">
          {/* 左侧：录音图标 */}
          <div className="relative flex-shrink-0">
            <Microphone className="w-5 h-5 text-white" />
            <div className="absolute inset-0 bg-white rounded-full opacity-40 animate-ping"></div>
          </div>

          {/* 调试信息 */}
          <div className="text-white text-xs">
            数据: {waveformData.slice(-3).map(v => v.toFixed(2)).join(', ')}
          </div>

          {/* 中间：波形 */}
          <div className="flex-1">
            <canvas
              ref={canvasRef}
              width={180}
              height={36}
              className="w-full h-full"
            />
          </div>

          {/* 右侧：计时器 */}
          <div className="flex-shrink-0 text-white font-mono text-base font-bold tracking-wider">
            {formatDuration(duration)}
          </div>

          {/* 音波动画点 */}
          <div className="flex items-center gap-0.5 flex-shrink-0">
            <div className="w-0.5 h-2 bg-white/80 rounded-full animate-pulse" style={{ animationDelay: '0ms' }}></div>
            <div className="w-0.5 h-3 bg-white/80 rounded-full animate-pulse" style={{ animationDelay: '150ms' }}></div>
            <div className="w-0.5 h-2.5 bg-white/80 rounded-full animate-pulse" style={{ animationDelay: '300ms' }}></div>
          </div>
        </div>
      </div>
    </div>
  );
}
