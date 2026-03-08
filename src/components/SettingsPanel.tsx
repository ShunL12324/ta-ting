import { Settings, Keyboard, Download, Check } from 'lucide-react';
import { useState, useEffect, useCallback } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { useAppStore } from '../stores/appStore';
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from './ui/dialog';
import { Button } from './ui/button';
import { Switch } from './ui/switch';
import { Label } from './ui/label';
import { Badge } from './ui/badge';
import { Separator } from './ui/separator';

// Convert internal hotkey format to display parts.
// "Ctrl+Shift+KeyV" → ["Ctrl", "Shift", "V"]
function hotkeyToDisplayParts(hotkey: string): string[] {
  return hotkey.split('+').map((part) => {
    if (part.startsWith('Key')) return part.slice(3);
    if (part.startsWith('Digit')) return part.slice(5);
    return part;
  });
}

function HotkeyBadges({ parts }: { parts: string[] }) {
  return (
    <div className="flex items-center gap-1.5 flex-wrap">
      {parts.map((part, i) => (
        <span key={i} className="flex items-center gap-1.5">
          <kbd className="px-2 py-1 bg-background text-foreground rounded text-xs font-semibold shadow-sm border border-border">
            {part}
          </kbd>
          {i < parts.length - 1 && (
            <span className="text-muted-foreground text-xs font-bold">+</span>
          )}
        </span>
      ))}
    </div>
  );
}

type LangId = 'zh' | 'en' | 'auto';

const LANGUAGES: { id: LangId; name: string; desc: string; bundled: boolean }[] = [
  { id: 'zh',   name: '中文',   desc: 'Chinese',           bundled: true  },
  { id: 'en',   name: 'English', desc: 'English',          bundled: false },
  { id: 'auto', name: '自动',   desc: 'Chinese & English', bundled: true  },
];

function LanguageSelector() {
  const [selected, setSelected] = useState<LangId>('zh');

  return (
    <div>
      <Label className="text-xs font-semibold mb-1.5 block">语言</Label>
      <div className="space-y-1.5">
        {LANGUAGES.map((lang) => {
          const isAvailable = lang.bundled;
          const isSelected = selected === lang.id;

          return (
            <div
              key={lang.id}
              onClick={() => isAvailable && setSelected(lang.id)}
              className={`flex items-center justify-between px-3 py-2 rounded-lg border-2 transition-all duration-150 ${
                isAvailable ? 'cursor-pointer' : 'cursor-default opacity-60'
              } ${
                isSelected
                  ? 'border-primary bg-primary/10'
                  : 'border-border bg-muted hover:bg-accent'
              }`}
            >
              <div className="flex items-center gap-2.5">
                <div
                  className={`w-3.5 h-3.5 rounded-full border-2 flex items-center justify-center flex-shrink-0 ${
                    isSelected ? 'border-primary' : 'border-muted-foreground'
                  }`}
                >
                  {isSelected && <div className="w-1.5 h-1.5 rounded-full bg-primary" />}
                </div>
                <div>
                  <span className="text-sm font-medium text-foreground">{lang.name}</span>
                  <span className="ml-1.5 text-[10px] text-muted-foreground">{lang.desc}</span>
                </div>
              </div>

              {isAvailable ? (
                <Badge variant="success" className="gap-1">
                  <Check className="w-3 h-3" />
                  已安装
                </Badge>
              ) : (
                <Button
                  variant="ghost"
                  size="sm"
                  onClick={(e) => e.stopPropagation()}
                  className="h-auto py-0.5 px-2 text-[10px]"
                >
                  <Download className="w-3 h-3" />
                  下载
                </Button>
              )}
            </div>
          );
        })}
      </div>
    </div>
  );
}

export function SettingsPanel() {
  const [isOpen, setIsOpen] = useState(false);
  const { hotkey, setHotkey } = useAppStore();

  // Hotkey recorder state
  const [recording, setRecording] = useState(false);
  const [pendingHotkey, setPendingHotkey] = useState<string | null>(null);
  const [pendingParts, setPendingParts] = useState<string[]>([]);
  const [saveError, setSaveError] = useState<string | null>(null);
  const [saveSuccess, setSaveSuccess] = useState(false);

  const currentParts = hotkeyToDisplayParts(hotkey);

  const startRecording = () => {
    setRecording(true);
    setPendingHotkey(null);
    setPendingParts([]);
    setSaveError(null);
    setSaveSuccess(false);
  };

  const cancelRecording = useCallback(() => {
    setRecording(false);
    setPendingHotkey(null);
    setPendingParts([]);
  }, []);

  const handleKeyDown = useCallback(
    (e: KeyboardEvent) => {
      if (!recording) return;
      e.preventDefault();
      e.stopPropagation();

      if (e.key === 'Escape') {
        cancelRecording();
        return;
      }

      const mods: string[] = [];
      if (e.ctrlKey) mods.push('Ctrl');
      if (e.shiftKey) mods.push('Shift');
      if (e.altKey) mods.push('Alt');
      if (e.metaKey) mods.push('Win');

      const modifierKeys = ['Control', 'Shift', 'Alt', 'Meta'];
      if (modifierKeys.includes(e.key)) {
        setPendingParts(mods);
        return;
      }

      if (mods.length === 0) return;

      const keyCode = e.code;
      const displayKey = keyCode.startsWith('Key')
        ? keyCode.slice(3)
        : keyCode.startsWith('Digit')
          ? keyCode.slice(5)
          : keyCode;

      const internal = [...mods, keyCode].join('+');
      setPendingHotkey(internal);
      setPendingParts([...mods, displayKey]);
      setRecording(false);
    },
    [recording, cancelRecording],
  );

  useEffect(() => {
    if (recording) {
      window.addEventListener('keydown', handleKeyDown, true);
      return () => window.removeEventListener('keydown', handleKeyDown, true);
    }
  }, [recording, handleKeyDown]);

  // Cancel recording if panel closes
  useEffect(() => {
    if (!isOpen) cancelRecording();
  }, [isOpen, cancelRecording]);

  const applyHotkey = async () => {
    if (!pendingHotkey) return;
    setSaveError(null);
    try {
      await invoke('set_hotkey', { hotkey: pendingHotkey });
      setHotkey(pendingHotkey);
      setPendingHotkey(null);
      setPendingParts([]);
      setSaveSuccess(true);
      setTimeout(() => setSaveSuccess(false), 2000);
    } catch (e) {
      setSaveError(String(e));
    }
  };

  return (
    <Dialog open={isOpen} onOpenChange={setIsOpen}>
      <DialogTrigger asChild>
        <Button
          variant="outline"
          size="icon"
          className="fixed bottom-6 right-6 rounded-xl shadow-lg"
          title="设置"
        >
          <Settings className="w-5 h-5" />
        </Button>
      </DialogTrigger>
      <DialogContent className="max-w-md">
        <DialogHeader>
          <DialogTitle>设置</DialogTitle>
        </DialogHeader>

        <div className="space-y-4">
          {/* Language */}
          <LanguageSelector />

          {/* Hotkey */}
          <div>
            <Label className="text-xs font-semibold mb-1.5 block">全局热键</Label>

            <div className="space-y-2">
              {/* Current / pending display */}
              <div className="px-3 py-2.5 border border-input rounded-md bg-muted min-h-[42px] flex items-center justify-between gap-2">
                {recording ? (
                  <span className="text-xs text-primary font-medium animate-pulse flex items-center gap-1.5">
                    <Keyboard className="w-3.5 h-3.5" />
                    {pendingParts.length > 0 ? (
                      <HotkeyBadges parts={pendingParts} />
                    ) : (
                      '按下组合键...'
                    )}
                  </span>
                ) : pendingHotkey ? (
                  <HotkeyBadges parts={pendingParts} />
                ) : (
                  <HotkeyBadges parts={currentParts} />
                )}

                {!recording && (
                  <Button variant="ghost" size="sm" onClick={startRecording} className="h-auto py-0.5 px-2 text-xs">
                    更改
                  </Button>
                )}
                {recording && (
                  <Button variant="ghost" size="sm" onClick={cancelRecording} className="h-auto py-0.5 px-2 text-xs">
                    取消
                  </Button>
                )}
              </div>

              {/* Confirm / error row */}
              {pendingHotkey && !recording && (
                <div className="flex items-center justify-between gap-2">
                  <span className="text-[10px] text-muted-foreground">按"应用"保存新热键</span>
                  <div className="flex gap-1.5">
                    <Button variant="outline" size="sm" onClick={cancelRecording} className="h-auto py-1 px-2.5 text-[10px]">
                      取消
                    </Button>
                    <Button size="sm" onClick={applyHotkey} className="h-auto py-1 px-2.5 text-[10px]">
                      应用
                    </Button>
                  </div>
                </div>
              )}

              {saveError && (
                <p className="text-xs text-destructive">{saveError}</p>
              )}
              {saveSuccess && (
                <p className="text-xs text-green-600">✓ 热键已保存</p>
              )}
            </div>

            <p className="mt-1 text-[10px] text-muted-foreground flex items-center gap-1">
              <span className="w-1 h-1 rounded-full bg-primary"></span>
              按下热键开始/停止录音，至少需要一个修饰键
            </p>
          </div>

          {/* Toggles */}
          <div className="space-y-2 pt-1">
            <div className="flex items-center justify-between p-3 rounded-lg bg-muted">
              <Label htmlFor="autostart" className="text-sm font-medium cursor-pointer">开机自动启动</Label>
              <Switch id="autostart" />
            </div>
            <div className="flex items-center justify-between p-3 rounded-lg bg-muted">
              <Label htmlFor="show-transcription" className="text-sm font-medium cursor-pointer">显示转录过程</Label>
              <Switch id="show-transcription" defaultChecked />
            </div>
          </div>
        </div>

        {/* About */}
        <Separator className="my-2" />
        <div className="text-center space-y-1.5">
          <p className="text-sm font-bold text-foreground">
            TaTing <span className="text-xs font-normal text-muted-foreground">v0.1.0</span>
          </p>
          <p className="text-xs text-muted-foreground">AI 离线听写输入法</p>
          <div className="flex items-center justify-center gap-1.5 pt-1.5">
            <Badge variant="outline">Sherpa-ONNX</Badge>
            <Badge variant="success">完全离线</Badge>
            <Badge variant="outline">隐私优先</Badge>
          </div>
        </div>
      </DialogContent>
    </Dialog>
  );
}
