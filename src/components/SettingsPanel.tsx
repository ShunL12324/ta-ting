import { Settings, X, Keyboard, Download, Check } from 'lucide-react';
import { useState, useEffect, useCallback } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { useAppStore } from '../stores/appStore';

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
          <kbd className="px-2 py-1 bg-white text-gray-700 rounded text-xs font-semibold shadow-sm border border-gray-200">
            {part}
          </kbd>
          {i < parts.length - 1 && (
            <span className="text-gray-400 text-xs font-bold">+</span>
          )}
        </span>
      ))}
    </div>
  );
}

type LangId = 'zh' | 'en' | 'auto';

const LANGUAGES: { id: LangId; name: string; desc: string; bundled: boolean }[] = [
  { id: 'zh',   name: '中文',   desc: 'Chinese',              bundled: true  },
  { id: 'en',   name: 'English', desc: 'English',             bundled: false },
  { id: 'auto', name: '自动',   desc: 'Chinese & English',    bundled: true  },
];

function LanguageSelector() {
  const [selected, setSelected] = useState<LangId>('zh');

  return (
    <div>
      <label className="block text-xs font-semibold text-gray-700 mb-1.5">语言</label>
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
                  ? 'border-blue-500 bg-blue-50'
                  : 'border-gray-200 bg-gray-50 hover:bg-gray-100'
              }`}
            >
              <div className="flex items-center gap-2.5">
                <div
                  className={`w-3.5 h-3.5 rounded-full border-2 flex items-center justify-center flex-shrink-0 ${
                    isSelected ? 'border-blue-500' : 'border-gray-300'
                  }`}
                >
                  {isSelected && <div className="w-1.5 h-1.5 rounded-full bg-blue-500" />}
                </div>
                <div>
                  <span className="text-sm font-medium text-gray-800">{lang.name}</span>
                  <span className="ml-1.5 text-[10px] text-gray-400">{lang.desc}</span>
                </div>
              </div>

              {isAvailable ? (
                <span className="flex items-center gap-1 text-[10px] text-green-600 font-medium">
                  <Check className="w-3 h-3" />
                  已安装
                </span>
              ) : (
                <button
                  onClick={(e) => e.stopPropagation()}
                  className="flex items-center gap-1 text-[10px] text-blue-500 hover:text-blue-700 font-medium"
                >
                  <Download className="w-3 h-3" />
                  下载
                </button>
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
        // Only modifiers pressed so far — show partial state
        setPendingParts(mods);
        return;
      }

      // Complete combination
      if (mods.length === 0) return; // must have at least one modifier

      const keyCode = e.code; // "KeyV", "Digit1", "F1", "Space", etc.
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
    <>
      <button
        onClick={() => setIsOpen(true)}
        className="fixed bottom-6 right-6 p-3 bg-gradient-to-br from-gray-800 to-gray-900 hover:from-gray-700 hover:to-gray-800 text-white rounded-xl shadow-lg transition-all duration-300 hover:scale-105 group"
        title="设置"
      >
        <Settings className="w-5 h-5 transition-transform duration-300 group-hover:rotate-90" />
      </button>

      {isOpen && (
        <div
          className="fixed inset-0 bg-black/50 backdrop-blur-sm flex items-center justify-center z-50 p-4 animate-in fade-in duration-200"
          onClick={() => setIsOpen(false)}
        >
          <div
            className="bg-white rounded-xl shadow-2xl p-5 max-w-md w-full max-h-[90vh] overflow-y-auto animate-in slide-in-from-bottom-4 duration-300"
            onClick={(e) => e.stopPropagation()}
          >
            {/* Header */}
            <div className="flex items-center justify-between mb-5">
              <h2 className="text-xl font-bold text-gray-800">设置</h2>
              <button
                onClick={() => setIsOpen(false)}
                className="p-1.5 hover:bg-gray-100 rounded-lg transition-all duration-200 hover:rotate-90"
              >
                <X className="w-5 h-5 text-gray-600" />
              </button>
            </div>

            <div className="space-y-4">
              {/* Language */}
              <LanguageSelector />

              {/* Hotkey */}
              <div>
                <label className="block text-xs font-semibold text-gray-700 mb-1.5">
                  全局热键
                </label>

                <div className="space-y-2">
                  {/* Current / pending display */}
                  <div className="px-3 py-2.5 border-2 border-gray-200 rounded-lg bg-gradient-to-br from-gray-50 to-gray-100 min-h-[42px] flex items-center justify-between gap-2">
                    {recording ? (
                      <span className="text-xs text-blue-500 font-medium animate-pulse flex items-center gap-1.5">
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
                      <button
                        onClick={startRecording}
                        className="text-[10px] text-blue-500 hover:text-blue-700 font-medium whitespace-nowrap"
                      >
                        更改
                      </button>
                    )}
                    {recording && (
                      <button
                        onClick={cancelRecording}
                        className="text-[10px] text-gray-400 hover:text-gray-600 font-medium whitespace-nowrap"
                      >
                        取消
                      </button>
                    )}
                  </div>

                  {/* Confirm / error row */}
                  {pendingHotkey && !recording && (
                    <div className="flex items-center justify-between gap-2">
                      <span className="text-[10px] text-gray-500">按"应用"保存新热键</span>
                      <div className="flex gap-1.5">
                        <button
                          onClick={cancelRecording}
                          className="px-2.5 py-1 text-[10px] text-gray-500 hover:text-gray-700 border border-gray-200 rounded-md"
                        >
                          取消
                        </button>
                        <button
                          onClick={applyHotkey}
                          className="px-2.5 py-1 text-[10px] bg-blue-500 hover:bg-blue-600 text-white rounded-md font-medium"
                        >
                          应用
                        </button>
                      </div>
                    </div>
                  )}

                  {saveError && (
                    <p className="text-[10px] text-red-500">{saveError}</p>
                  )}
                  {saveSuccess && (
                    <p className="text-[10px] text-green-600">✓ 热键已保存</p>
                  )}
                </div>

                <p className="mt-1 text-[10px] text-gray-500 flex items-center gap-1">
                  <span className="w-1 h-1 rounded-full bg-blue-500"></span>
                  按下热键开始/停止录音，至少需要一个修饰键
                </p>
              </div>

              {/* Toggles */}
              <div className="space-y-2 pt-1">
                <label className="flex items-center justify-between p-3 bg-gray-50 rounded-lg hover:bg-gray-100 transition-colors cursor-pointer group">
                  <span className="text-xs font-medium text-gray-700 group-hover:text-gray-900">
                    开机自动启动
                  </span>
                  <div className="relative">
                    <input type="checkbox" className="peer sr-only" defaultChecked={false} />
                    <div className="w-9 h-5 bg-gray-300 rounded-full peer-checked:bg-blue-500 transition-all duration-200 peer-focus:ring-2 peer-focus:ring-blue-300"></div>
                    <div className="absolute left-0.5 top-0.5 w-4 h-4 bg-white rounded-full transition-all duration-200 peer-checked:translate-x-4 shadow-sm"></div>
                  </div>
                </label>

                <label className="flex items-center justify-between p-3 bg-gray-50 rounded-lg hover:bg-gray-100 transition-colors cursor-pointer group">
                  <span className="text-xs font-medium text-gray-700 group-hover:text-gray-900">
                    显示转录过程
                  </span>
                  <div className="relative">
                    <input type="checkbox" className="peer sr-only" defaultChecked={true} />
                    <div className="w-9 h-5 bg-gray-300 rounded-full peer-checked:bg-blue-500 transition-all duration-200 peer-focus:ring-2 peer-focus:ring-blue-300"></div>
                    <div className="absolute left-0.5 top-0.5 w-4 h-4 bg-white rounded-full transition-all duration-200 peer-checked:translate-x-4 shadow-sm"></div>
                  </div>
                </label>
              </div>
            </div>

            {/* About */}
            <div className="mt-5 pt-4 border-t-2 border-gray-100">
              <div className="text-center space-y-1.5">
                <p className="text-sm font-bold text-gray-800">
                  TaTing <span className="text-xs font-normal text-gray-500">v0.1.0</span>
                </p>
                <p className="text-xs text-gray-600">AI 离线听写输入法</p>
                <div className="flex items-center justify-center gap-1.5 text-[10px] pt-1.5">
                  <span className="px-1.5 py-0.5 bg-blue-50 text-blue-600 rounded font-medium">
                    Sherpa-ONNX
                  </span>
                  <span className="px-1.5 py-0.5 bg-green-50 text-green-600 rounded font-medium">
                    完全离线
                  </span>
                  <span className="px-1.5 py-0.5 bg-purple-50 text-purple-600 rounded font-medium">
                    隐私优先
                  </span>
                </div>
              </div>
            </div>
          </div>
        </div>
      )}
    </>
  );
}
