import {
  Keyboard,
  DownloadSimple,
  Check,
  Translate,
  Devices,
  SlidersHorizontal,
  Info,
} from '@phosphor-icons/react';
import { useState, useEffect, useCallback } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { useTranslation } from 'react-i18next';
import { useAppStore } from '../stores/appStore';
import { Button } from '../components/ui/button';
import { Switch } from '../components/ui/switch';
import { Label } from '../components/ui/label';
import { Badge } from '../components/ui/badge';
import { Card } from '../components/ui/card';

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

const LANG_IDS: LangId[] = ['zh', 'en', 'auto'];
const LANG_BUNDLED: Record<LangId, boolean> = { zh: true, en: false, auto: true };

function TranscriptionLangSelector() {
  const { t } = useTranslation();
  const [selected, setSelected] = useState<LangId>('zh');

  return (
    <div className="space-y-1.5">
      {LANG_IDS.map((id) => {
        const isAvailable = LANG_BUNDLED[id];
        const isSelected = selected === id;

        return (
          <div
            key={id}
            onClick={() => isAvailable && setSelected(id)}
            className={`flex items-center justify-between px-3 py-2 rounded-lg border-2 transition-all duration-150 ${
              isAvailable ? 'cursor-pointer' : 'cursor-default opacity-60'
            } ${
              isSelected
                ? 'border-primary bg-primary/10'
                : 'border-border bg-background hover:bg-accent'
            }`}
          >
            <div className="flex items-center gap-2.5">
              <div className={`w-3.5 h-3.5 rounded-full border-2 flex items-center justify-center flex-shrink-0 ${
                isSelected ? 'border-primary' : 'border-muted-foreground'
              }`}>
                {isSelected && <div className="w-1.5 h-1.5 rounded-full bg-primary" />}
              </div>
              <div>
                <span className="text-sm font-medium text-foreground">
                  {t(`settings.transcriptionLang.languages.${id}.name`)}
                </span>
                <span className="ml-1.5 text-[10px] text-muted-foreground">
                  {t(`settings.transcriptionLang.languages.${id}.desc`)}
                </span>
              </div>
            </div>
            {isAvailable ? (
              <Badge className="gap-1 border-transparent bg-success/15 text-success hover:bg-success/20">
                <Check size={12} weight="bold" />
                {t('common.installed')}
              </Badge>
            ) : (
              <Button
                variant="ghost"
                size="sm"
                onClick={(e) => e.stopPropagation()}
                className="h-auto py-0.5 px-2 text-[10px]"
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
    <div className="flex gap-2">
      {(['zh', 'en'] as UILang[]).map((id) => (
        <button
          key={id}
          onClick={() => i18n.changeLanguage(id)}
          className={`flex-1 py-2 px-3 rounded-lg border-2 text-left transition-all duration-150 ${
            current === id
              ? 'border-primary bg-primary/10'
              : 'border-border bg-background hover:bg-accent'
          }`}
        >
          <p className="text-sm font-medium text-foreground">
            {t(`settings.uiLang.options.${id}.label`)}
          </p>
          <p className="text-[10px] text-muted-foreground">
            {t(`settings.uiLang.options.${id}.sub`)}
          </p>
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
    <div className="space-y-2">
      <div className="px-3 py-2.5 border border-input rounded-md bg-background min-h-[42px] flex items-center justify-between gap-2">
        {recording ? (
          <span className="text-xs text-primary font-medium animate-pulse flex items-center gap-1.5">
            <Keyboard size={14} />
            {pendingParts.length > 0 ? <HotkeyBadges parts={pendingParts} /> : t('settings.hotkey.pressCombo')}
          </span>
        ) : pendingHotkey ? (
          <HotkeyBadges parts={pendingParts} />
        ) : (
          <HotkeyBadges parts={currentParts} />
        )}
        {!recording && (
          <Button variant="ghost" size="sm" onClick={startRecording} className="h-auto py-0.5 px-2 text-xs">
            {t('common.change')}
          </Button>
        )}
        {recording && (
          <Button variant="ghost" size="sm" onClick={cancelRecording} className="h-auto py-0.5 px-2 text-xs">
            {t('common.cancel')}
          </Button>
        )}
      </div>

      {pendingHotkey && !recording && (
        <div className="flex items-center justify-between gap-2">
          <span className="text-[10px] text-muted-foreground">{t('settings.hotkey.saveHint')}</span>
          <div className="flex gap-1.5">
            <Button variant="outline" size="sm" onClick={cancelRecording} className="h-auto py-1 px-2.5 text-[10px]">
              {t('common.cancel')}
            </Button>
            <Button size="sm" onClick={applyHotkey} className="h-auto py-1 px-2.5 text-[10px]">
              {t('common.apply')}
            </Button>
          </div>
        </div>
      )}

      {saveError && <p className="text-xs text-destructive">{saveError}</p>}
      {saveSuccess && <p className="text-xs text-success">{t('settings.hotkey.saved')}</p>}

      <p className="text-[10px] text-muted-foreground flex items-center gap-1">
        <span className="w-1 h-1 rounded-full bg-primary inline-block" />
        {t('settings.hotkey.modifierHint')}
      </p>
    </div>
  );
}

function SettingCard({
  icon: Icon,
  title,
  children,
}: {
  icon: React.ElementType;
  title: string;
  children: React.ReactNode;
}) {
  return (
    <Card className="p-4 space-y-3">
      <div className="flex items-center gap-2">
        <Icon size={15} weight="duotone" className="text-muted-foreground" />
        <span className="text-xs font-semibold text-muted-foreground uppercase tracking-wider">
          {title}
        </span>
      </div>
      {children}
    </Card>
  );
}

export function SettingsPage() {
  const { t } = useTranslation();

  return (
    <div className="p-4 space-y-3 max-w-lg">
      <SettingCard icon={Translate} title={t('settings.transcriptionLang.title')}>
        <TranscriptionLangSelector />
      </SettingCard>

      <SettingCard icon={Devices} title={t('settings.uiLang.title')}>
        <UILanguageSelector />
      </SettingCard>

      <SettingCard icon={Keyboard} title={t('settings.hotkey.title')}>
        <HotkeyRecorder />
      </SettingCard>

      <SettingCard icon={SlidersHorizontal} title={t('settings.behavior.title')}>
        <div className="space-y-2">
          <div className="flex items-center justify-between py-1">
            <Label htmlFor="autostart" className="text-sm font-medium cursor-pointer">
              {t('settings.behavior.autostart')}
            </Label>
            <Switch id="autostart" />
          </div>
          <div className="flex items-center justify-between py-1">
            <Label htmlFor="show-transcription" className="text-sm font-medium cursor-pointer">
              {t('settings.behavior.showTranscription')}
            </Label>
            <Switch id="show-transcription" defaultChecked />
          </div>
        </div>
      </SettingCard>

      <SettingCard icon={Info} title={t('settings.about.title')}>
        <div className="flex items-center justify-between">
          <div>
            <p className="text-sm font-semibold text-foreground">TaTing</p>
            <p className="text-xs text-muted-foreground">{t('settings.about.subtitle')}</p>
          </div>
          <div className="flex gap-1.5">
            <Badge variant="outline">v0.2.0</Badge>
            <Badge className="border-transparent bg-success/15 text-success hover:bg-success/20">Offline</Badge>
          </div>
        </div>
      </SettingCard>
    </div>
  );
}
