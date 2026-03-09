import { Microphone, CircleNotch, Check, Keyboard } from '@phosphor-icons/react';
import { useTranslation } from 'react-i18next';
import { useAppStore } from '../stores/appStore';
import type { AppState } from '../stores/appStore';

function hotkeyToDisplayParts(hotkey: string): string[] {
  return hotkey.split('+').map((part) => {
    if (part.startsWith('Key')) return part.slice(3);
    if (part.startsWith('Digit')) return part.slice(5);
    return part;
  });
}

const STATE_STYLE: Record<AppState, {
  ringClass: string;
  bgClass: string;
  Icon: React.ElementType;
  iconWeight: 'regular' | 'fill' | 'duotone';
  iconClass: string;
  labelClass: string;
  ping: boolean;
}> = {
  idle: {
    ringClass: 'border-border',
    bgClass: '',
    Icon: Microphone,
    iconWeight: 'regular',
    iconClass: 'text-muted-foreground',
    labelClass: 'text-muted-foreground',
    ping: false,
  },
  recording: {
    ringClass: 'border-destructive border-2',
    bgClass: 'bg-destructive/5',
    Icon: Microphone,
    iconWeight: 'fill',
    iconClass: 'text-destructive',
    labelClass: 'text-destructive',
    ping: true,
  },
  transcribing: {
    ringClass: 'border-primary border-2',
    bgClass: 'bg-primary/5',
    Icon: CircleNotch,
    iconWeight: 'regular',
    iconClass: 'text-primary animate-spin',
    labelClass: 'text-primary',
    ping: false,
  },
  inputting: {
    ringClass: 'border-success border-2',
    bgClass: 'bg-success/5',
    Icon: Check,
    iconWeight: 'fill',
    iconClass: 'text-success',
    labelClass: 'text-success',
    ping: false,
  },
};

export function HomePage() {
  const { t } = useTranslation();
  const { state, error, hotkey } = useAppStore();
  const style = STATE_STYLE[state] ?? STATE_STYLE.idle;
  const { Icon, iconWeight } = style;
  const hotkeyParts = hotkeyToDisplayParts(hotkey);

  return (
    <div className="flex flex-col items-center justify-center h-full gap-7 select-none px-8">
      {/* Brand */}
      <div className="text-center space-y-0.5">
        <h1 className="text-lg font-bold tracking-tight text-foreground">TaTing</h1>
        <p className="text-xs text-muted-foreground">{t('home.subtitle')}</p>
      </div>

      {/* State ring */}
      <div className="relative flex items-center justify-center">
        {style.ping && (
          <>
            <span className="absolute w-40 h-40 rounded-full border border-destructive animate-ping opacity-30" />
            <span className="absolute w-48 h-48 rounded-full border border-destructive animate-ping opacity-15"
              style={{ animationDelay: '0.4s' }} />
          </>
        )}
        <div className={`
          w-36 h-36 rounded-full border-2 flex items-center justify-center
          transition-all duration-500
          ${style.ringClass} ${style.bgClass}
        `}>
          <Icon size={36} weight={iconWeight} className={`transition-all duration-300 ${style.iconClass}`} />
        </div>
      </div>

      {/* Status label */}
      <p className={`text-sm font-medium transition-colors duration-300 ${style.labelClass}`}>
        {t(`home.status.${state}`)}
      </p>

      {/* Hotkey hint */}
      <div className="flex items-center gap-1.5">
        <Keyboard size={14} weight="regular" className="text-muted-foreground/60 mr-0.5" />
        {hotkeyParts.map((part, i) => (
          <span key={i} className="flex items-center gap-1.5">
            <kbd className="px-2 py-1 bg-muted text-foreground rounded-md font-mono text-xs font-semibold border border-border shadow-sm">
              {part}
            </kbd>
            {i < hotkeyParts.length - 1 && (
              <span className="text-muted-foreground/60 text-xs font-bold">+</span>
            )}
          </span>
        ))}
      </div>

      {/* Error */}
      {error && (
        <div className="w-full max-w-xs p-3 bg-destructive/10 border border-destructive/30 rounded-xl">
          <p className="text-xs text-destructive text-center">{error}</p>
        </div>
      )}
    </div>
  );
}
