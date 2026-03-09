import { useState, useEffect, useRef } from 'react';
import { Microphone, GearSix, Minus, X } from '@phosphor-icons/react';
import { useTranslation } from 'react-i18next';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { RecordingIndicator } from './components/RecordingIndicator';
import { UpdateChecker } from './components/UpdateChecker';
import { useAppStore } from './stores/appStore';
import { HomePage } from './pages/HomePage';
import { SettingsPage } from './pages/SettingsPage';

type Page = 'home' | 'settings';

function TitleBar() {
  const ref = useRef<HTMLDivElement>(null);

  useEffect(() => {
    const el = ref.current;
    if (!el) return;
    const appWindow = getCurrentWindow();
    const onMouseDown = (e: MouseEvent) => {
      if (e.buttons === 1) appWindow.startDragging();
    };
    el.addEventListener('mousedown', onMouseDown);
    return () => el.removeEventListener('mousedown', onMouseDown);
  }, []);

  return (
    <div
      ref={ref}
      className="h-9 flex items-center justify-between px-3 bg-card border-b border-border flex-shrink-0 select-none z-10 shadow-sm cursor-grab active:cursor-grabbing"
    >
      <div className="flex items-center gap-2 pointer-events-none">
        <div className="w-4 h-4 rounded bg-primary flex items-center justify-center">
          <span className="text-primary-foreground text-[9px] font-black leading-none">T</span>
        </div>
        <span className="text-xs font-medium text-muted-foreground">TaTing</span>
      </div>

      <div className="flex items-center gap-0.5">
        <button
          onMouseDown={(e) => e.stopPropagation()}
          onClick={() => getCurrentWindow().minimize()}
          className="w-7 h-7 flex items-center justify-center rounded text-muted-foreground hover:bg-muted hover:text-foreground transition-colors cursor-default"
        >
          <Minus size={14} />
        </button>
        <button
          onMouseDown={(e) => e.stopPropagation()}
          onClick={() => getCurrentWindow().close()}
          className="w-7 h-7 flex items-center justify-center rounded text-muted-foreground hover:bg-destructive hover:text-destructive-foreground transition-colors cursor-default"
        >
          <X size={14} />
        </button>
      </div>
    </div>
  );
}

interface NavItem {
  page: Page;
  labelKey: string;
  Icon: React.ElementType;
}

const NAV_ITEMS: NavItem[] = [
  { page: 'home',     labelKey: 'nav.home',     Icon: Microphone },
  { page: 'settings', labelKey: 'nav.settings', Icon: GearSix    },
];

interface SidebarProps {
  currentPage: Page;
  onNavigate: (page: Page) => void;
}

function Sidebar({ currentPage, onNavigate }: SidebarProps) {
  const { t } = useTranslation();

  return (
    <div className="w-40 flex flex-col py-2 px-2 bg-muted border-r border-border flex-shrink-0">
      {/* Nav */}
      <div className="flex flex-col gap-0.5 flex-1">
        {NAV_ITEMS.map(({ page, labelKey, Icon }) => {
          const isActive = currentPage === page;

          return (
            <button
              key={page}
              onClick={() => onNavigate(page)}
              className={`flex items-center gap-2.5 w-full px-3 py-2 rounded-lg text-sm font-medium transition-colors ${
                isActive
                  ? 'bg-background text-foreground shadow-sm'
                  : 'text-muted-foreground hover:bg-accent hover:text-foreground'
              }`}
            >
              <Icon size={16} weight={isActive ? 'fill' : 'regular'} />
              <span>{t(labelKey)}</span>
            </button>
          );
        })}
      </div>

      {/* Version */}
      <span className="px-3 text-[10px] text-muted-foreground/40 font-mono">
        v0.2.0
      </span>
    </div>
  );
}

function App() {
  const [page, setPage] = useState<Page>('home');
  const { isRecording } = useAppStore();

  return (
    <div className="flex flex-col h-screen bg-background overflow-hidden">
      <TitleBar />
      <div className="flex flex-1 overflow-hidden">
        <Sidebar currentPage={page} onNavigate={setPage} />
        <main className="flex-1 overflow-y-auto relative">
          <RecordingIndicator isRecording={isRecording} />
          <UpdateChecker />
          {page === 'home' && <HomePage />}
          {page === 'settings' && <SettingsPage />}
        </main>
      </div>
    </div>
  );
}

export default App;
