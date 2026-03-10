import { useEffect, useRef, useState } from 'react';
import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';
import { Microphone } from '@phosphor-icons/react';

const AMBER = { r: 245, g: 166, b: 35 };

export function RecordingWindow() {
  const [duration, setDuration] = useState(0);
  const [waveformData, setWaveformData] = useState<number[]>(new Array(32).fill(0));
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const prevDataRef = useRef<number[]>(new Array(32).fill(0));

  useEffect(() => {
    const win = getCurrentWebviewWindow();
    win.setShadow(false);

    // Listen for reset signal sent before each show() so state is fresh
    let unlistenReset: (() => void) | null = null;
    win.listen('recording_reset', () => {
      setDuration(0);
      const zeros = new Array(32).fill(0);
      setWaveformData(zeros);
      prevDataRef.current = zeros;
    }).then((fn) => { unlistenReset = fn; });

    return () => { unlistenReset?.(); };
  }, []);

  useEffect(() => {
    const timer = setInterval(() => setDuration((p) => p + 1), 1000);
    const win = getCurrentWebviewWindow();
    let unlisten: (() => void) | null = null;

    win.listen<number[]>('audio_data', (event) => {
      const samples = event.payload;
      const n = 32;
      let raw: number[];

      if (samples.length >= n) {
        raw = Array.from({ length: n }, (_, i) => {
          const s = Math.floor(i * samples.length / n);
          const e = Math.floor((i + 1) * samples.length / n);
          return Math.max(...samples.slice(s, e).map(Math.abs));
        });
      } else {
        raw = Array.from({ length: n }, (_, i) => {
          const pos = (i / (n - 1)) * (samples.length - 1);
          const idx = Math.floor(pos);
          const frac = pos - idx;
          if (idx >= samples.length - 1) return Math.abs(samples[samples.length - 1] || 0);
          return Math.abs(samples[idx]) * (1 - frac) + Math.abs(samples[idx + 1]) * frac;
        });
      }

      const smoothed = raw.map((v, i) => prevDataRef.current[i] * 0.8 + v * 0.2);
      prevDataRef.current = smoothed;
      setWaveformData(smoothed);
    }).then((fn) => { unlisten = fn; });

    return () => { clearInterval(timer); unlisten?.(); };
  }, []);

  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;
    const ctx = canvas.getContext('2d');
    if (!ctx) return;

    const { width, height } = canvas;
    ctx.clearRect(0, 0, width, height);

    const barWidth = width / waveformData.length;
    const centerY = height / 2;

    waveformData.forEach((value, i) => {
      const scaled = Math.min(value * 60, 1.0);
      const barH = Math.max(1.5, scaled * height * 0.44);
      const x = i * barWidth;
      const r = Math.min(1.5, barH * 0.3);
      const alpha = 0.5 + scaled * 0.5;

      ctx.fillStyle = `rgba(${AMBER.r}, ${AMBER.g}, ${AMBER.b}, ${alpha})`;
      ctx.beginPath();
      ctx.roundRect(x + 0.5, centerY - barH, barWidth - 1, barH * 2, r);
      ctx.fill();
    });
  }, [waveformData]);

  const fmt = (s: number) =>
    `${Math.floor(s / 60)}:${(s % 60).toString().padStart(2, '0')}`;

  return (
    <div className="w-full h-full flex items-center gap-2.5 px-3 bg-card border border-border rounded-full">
        {/* Mic */}
        <div className="relative flex-shrink-0">
          <Microphone size={14} weight="fill" className="text-primary relative z-10" />
          <span className="absolute inset-0 rounded-full bg-primary/30 animate-ping" />
        </div>

        {/* Waveform */}
        <canvas
          ref={canvasRef}
          width={168}
          height={28}
          className="flex-1 min-w-0"
        />

        {/* Divider */}
        <div className="w-px h-4 bg-border flex-shrink-0" />

        {/* Timer */}
        <span className="flex-shrink-0 font-mono text-xs font-medium text-primary tabular-nums">
          {fmt(duration)}
        </span>
    </div>
  );
}
