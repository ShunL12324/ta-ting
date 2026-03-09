import {
  Keyboard,
  DownloadSimple,
  Check,
} from '@phosphor-icons/react';
import { useState, useEffect, useCallback } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { useTranslation } from 'react-i18next';
import { useAppStore } from '../stores/appStore';
import { Button } from '../components/ui/button';
import { Switch } from '../components/ui/switch';
import { Label } from '../components/ui/label';
import { Separator } from '../components/ui/separator';

function hotkeyToDisplayParts(hotkey: string): string[] {
  return hotkey.split('+').map((part) => {
    if (part.startsWith('Key')) return part.slice(3);
    if (part.startsWith('Digit')) return part.slice(5);
    return part;
  });
}

function HotkeyBadges({ parts }: { parts: string[] }) {
  return (
    <div className="flex items-center gap-1 flex-wrap">
      {parts.map((part, i) => (
        <span key={i} className="flex items-center gap-1">
          <kbd className="px-1.5 py-0.5 bg-muted text-foreground rounded text-xs font-semibold border border-border">
            {part}
          </kbd>
          {i < parts.length - 1 && (
            <span className="text-muted-foreground text-xs">+</span>
          )}
        </span>
      ))}
    </div>
  );
}

function SectionLabel({ children }: { children: React.ReactNode }) {
  return (
    <p className="px-1 pb-2 text-[10px] font-semibold text-muted-foreground uppercase tracking-widest">
      {children}
    </p>
  );
}

type LangId = 'zh' | 'en' | 'auto';

const LANG_IDS: LangId[] = ['zh', 'en', 'auto'];
const LANG_BUNDLED: Record<LangId, boolean> = { zh: true, en: false, auto: true };

function TranscriptionLangSelector() {
  const { t } = useTranslation();
  const [selected, setSelected] = useState<LangId>('zh');

  return (
    <div>
      {LANG_IDS.map((id) => {
        const isAvailable = LANG_BUNDLED[id];
        const isSelected = selected === id;

        return (
          <div
            key={id}
            onClick={() => isAvailable && setSelected(id)}
            className={`flex items-center justify-between px-2 py-2 rounded-md transition-colors ${
              isAvailable ? 'cursor-pointer' : 'cursor-default opacity-50'
            } ${isSelected ? 'bg-accent' : 'hover:bg-muted/60'}`}
          >
            <div>
              <span className="text-sm font-medium text-foreground">
                {t(`settings.transcriptionLang.languages.${id}.name`)}
              </span>
              <span className="ml-2 text-xs text-muted-foreground">
                {t(`settings.transcriptionLang.languages.${id}.desc`)}
              </span>
            </div>
            {isAvailable ? (
              isSelected
                ? <Check size={15} weight="bold" className="text-primary flex-shrink-0" />
                : null
            ) : (
              <Button
                variant="ghost"
                size="sm"
                onClick={(e) => e.stopPropagation()}
                className="h-auto py-0.5 px-2 text-xs text-muted-foreground"
              >
                <DownloadSimple size={12} />
                {t('common.download')}
              </Button>
            )}
          </div>
        );
      })}
    </div>
  );
}

type UILang = 'zh' | 'en';

function UILanguageSelector() {
  const { i18n, t } = useTranslation();
  const current = (i18n.language === 'en' ? 'en' : 'zh') as UILang;

  return (
    <div className="flex bg-muted rounded-lg p-1">
      {(['zh', 'en'] as UILang[]).map((id) => (
        <button
          key={id}
          onClick={() => i18n.changeLanguage(id)}
          className={`flex-1 py-1.5 rounded-md text-sm font-medium transition-all ${
            current === id
              ? 'bg-background text-foreground shadow-sm'
              : 'text-muted-foreground hover:text-foreground'
          }`}
        >
          {t(`settings.uiLang.options.${id}.label`)}
        </button>
      ))}
    </div>
  );
}

function HotkeyRecorder() {
  const { t } = useTranslation();
  const { hotkey, setHotkey } = useAppStore();
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

      if (e.key === 'Escape') { cancelRecording(); return; }

      const mods: string[] = [];
      if (e.ctrlKey) mods.push('Ctrl');
      if (e.shiftKey) mods.push('Shift');
      if (e.altKey) mods.push('Alt');
      if (e.metaKey) mods.push('Win');

      if (['Control', 'Shift', 'Alt', 'Meta'].includes(e.key)) {
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

      setPendingHotkey([...mods, keyCode].join('+'));
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
    <div className="space-y-1">
      <div className="flex items-center justify-between px-2 py-2 rounded-md hover:bg-muted/60 transition-colors">
        <div className="flex items-center gap-2 flex-1 min-w-0">
          {recording ? (
            <span className="text-xs text-primary font-medium animate-pulse flex items-center gap-1.5">
              <Keyboard size={14} />
              {pendingParts.length > 0
                ? <HotkeyBadges parts={pendingParts} />
                : t('settings.hotkey.pressCombo')}
            </span>
          ) : (
            <HotkeyBadges parts={pendingParts.length > 0 ? pendingParts : currentParts} />
          )}
        </div>
        <div className="flex gap-1 flex-shrink-0">
          {!recording && !pendingHotkey && (
            <Button variant="ghost" size="sm" onClick={startRecording} className="h-7 px-2 text-xs">
              {t('common.change')}
            </Button>
          )}
          {(recording || pendingHotkey) && (
            <Button variant="ghost" size="sm" onClick={cancelRecording} className="h-7 px-2 text-xs">
              {t('common.cancel')}
            </Button>
          )}
          {pendingHotkey && !recording && (
            <Button size="sm" onClick={applyHotkey} className="h-7 px-2 text-xs">
              {t('common.apply')}
            </Button>
          )}
        </div>
      </div>
      {saveError && <p className="px-2 text-xs text-destructive">{saveError}</p>}
      {saveSuccess && <p className="px-2 text-xs text-primary">{t('settings.hotkey.saved')}</p>}
      <p className="px-2 text-[10px] text-muted-foreground">{t('settings.hotkey.modifierHint')}</p>
    </div>
  );
}

export function SettingsPage() {
  const { t } = useTranslation();

  return (
    <div className="px-4 py-4 space-y-4 h-full overflow-y-auto">

      <section>
        <SectionLabel>{t('settings.transcriptionLang.title')}</SectionLabel>
        <TranscriptionLangSelector />
      </section>

      <Separator />

      <section>
        <SectionLabel>{t('settings.uiLang.title')}</SectionLabel>
        <UILanguageSelector />
      </section>

      <Separator />

      <section>
        <SectionLabel>{t('settings.hotkey.title')}</SectionLabel>
        <HotkeyRecorder />
      </section>

      <Separator />

      <section>
        <SectionLabel>{t('settings.behavior.title')}</SectionLabel>
        <div>
          <div className="flex items-center justify-between px-2 py-2 rounded-md hover:bg-muted/60 transition-colors">
            <Label htmlFor="autostart" className="text-sm font-normal cursor-pointer">
              {t('settings.behavior.autostart')}
            </Label>
            <Switch id="autostart" />
          </div>
          <div className="flex items-center justify-between px-2 py-2 rounded-md hover:bg-muted/60 transition-colors">
            <Label htmlFor="show-transcription" className="text-sm font-normal cursor-pointer">
              {t('settings.behavior.showTranscription')}
            </Label>
            <Switch id="show-transcription" defaultChecked />
          </div>
        </div>
      </section>

      <Separator />

      <section>
        <SectionLabel>{t('settings.about.title')}</SectionLabel>
        <div className="flex items-center justify-between px-2 py-1.5">
          <span className="text-sm text-foreground">TaTing</span>
          <div className="flex items-center gap-2 text-xs text-muted-foreground">
            <span>v0.2.0</span>
            <span>·</span>
            <span className="text-success">{t('settings.about.offline')}</span>
          </div>
        </div>
        <p className="px-2 text-xs text-muted-foreground">{t('settings.about.subtitle')}</p>
      </section>

    </div>
  );
}
