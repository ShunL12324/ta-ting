import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { listen, type UnlistenFn } from '@tauri-apps/api/event';
import {
  DownloadSimple,
  Check,
  CircleNotch,
  Trash,
  Warning,
  X,
} from '@phosphor-icons/react';

export interface ModelInfo {
  id: string;
  name: string;
  description: string;
  language: string;
  size_mb: number;
  status: 'bundled' | 'installed' | 'not_installed';
  is_bundled: boolean;
}

interface DownloadProgress {
  model_id: string;
  downloaded: number;
  total: number | null;
  percentage: number;
  status: string;
}

const LANG_LABEL: Record<string, string> = { zh: 'ZH', en: 'EN', auto: 'AUTO' };

export function ModelSelector() {
  const [models, setModels] = useState<ModelInfo[]>([]);
  const [activeModelId, setActiveModelId] = useState<string>('');
  const [progress, setProgress] = useState<Record<string, DownloadProgress>>({});
  const [switchingId, setSwitchingId] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    loadModels();
    loadActiveModel();
  }, []);

  useEffect(() => {
    const listeners: UnlistenFn[] = [];
    const setup = async () => {
      listeners.push(
        await listen<DownloadProgress>('model_download_progress', (e) => {
          setProgress((prev) => ({ ...prev, [e.payload.model_id]: e.payload }));
        }),
        await listen<string>('model_download_complete', (e) => {
          setProgress((prev) => { const next = { ...prev }; delete next[e.payload]; return next; });
          loadModels();
        }),
        await listen<string>('model_download_error', (e) => {
          const colonIdx = e.payload.indexOf(':');
          const modelId = colonIdx > 0 ? e.payload.slice(0, colonIdx).trim() : '';
          setProgress((prev) => { const next = { ...prev }; if (modelId) delete next[modelId]; return next; });
          setError(e.payload);
          setTimeout(() => setError(null), 6000);
        }),
      );
    };
    setup();
    return () => { listeners.forEach((fn) => fn()); };
  }, []);

  const loadModels = async () => {
    try { setModels(await invoke<ModelInfo[]>('get_available_models')); }
    catch (e) { setError(`Failed to load models: ${e}`); }
  };

  const loadActiveModel = async () => {
    try { setActiveModelId(await invoke<string>('get_active_model')); }
    catch {}
  };

  const handleSelect = async (modelId: string) => {
    if (modelId === activeModelId || switchingId) return;
    setSwitchingId(modelId);
    try {
      await invoke('set_active_model', { modelId });
      setActiveModelId(modelId);
    } catch (e) {
      setError(`Failed to switch model: ${e}`);
    } finally {
      setSwitchingId(null);
    }
  };

  const handleDownload = async (modelId: string) => {
    setError(null);
    setProgress((prev) => ({
      ...prev,
      [modelId]: { model_id: modelId, downloaded: 0, total: null, percentage: 0, status: 'downloading' },
    }));
    try { await invoke('download_model', { modelId }); }
    catch (e) {
      setProgress((prev) => { const next = { ...prev }; delete next[modelId]; return next; });
      setError(`Download failed: ${e}`);
    }
  };

  const handleCancel = async (modelId: string) => {
    try { await invoke('cancel_model_download', { modelId }); } catch {}
    setProgress((prev) => { const next = { ...prev }; delete next[modelId]; return next; });
  };

  const handleDelete = async (modelId: string) => {
    try { await invoke('delete_model', { modelId }); loadModels(); }
    catch (e) { setError(`Delete failed: ${e}`); }
  };

  return (
    <div className="space-y-0.5">
      {error && (
        <div className="flex items-center gap-1.5 px-2 py-1.5 mb-2 bg-destructive/10 text-destructive rounded-md text-xs">
          <Warning size={12} weight="fill" className="flex-shrink-0" />
          <span className="flex-1 min-w-0 truncate">{error}</span>
        </div>
      )}

      {models.map((model) => {
        const isActive = model.id === activeModelId;
        const isSwitching = switchingId === model.id;
        const dl = progress[model.id];
        const isDownloading = !!dl && dl.status === 'downloading';
        const isExtracting = !!dl && dl.status === 'extracting';
        const isBusy = isDownloading || isExtracting || isSwitching;
        const isInstalled = model.status === 'bundled' || model.status === 'installed';
        const isClickable = isInstalled && !isActive && !isBusy;

        return (
          <div
            key={model.id}
            onClick={() => isClickable && handleSelect(model.id)}
            className={`relative rounded-md transition-colors ${
              isActive
                ? 'bg-primary/8 pl-[13px]'
                : isClickable
                  ? 'cursor-pointer hover:bg-muted/60 pl-3'
                  : 'pl-3'
            }`}
          >
            {/* Amber left accent for active */}
            {isActive && (
              <span className="absolute left-0 top-1.5 bottom-1.5 w-[3px] rounded-full bg-primary" />
            )}

            <div className="flex items-center justify-between py-2 pr-2">
              {/* Left: name + chips */}
              <div className="flex items-center gap-2 min-w-0">
                <span className="text-sm font-medium text-foreground leading-none">{model.name}</span>
                <span className="text-[10px] font-semibold px-1.5 py-0.5 rounded bg-muted text-muted-foreground leading-none">
                  {LANG_LABEL[model.language] ?? model.language.toUpperCase()}
                </span>
                <span className="text-xs text-muted-foreground truncate hidden sm:block">
                  {model.description}
                </span>
              </div>

              {/* Right: actions */}
              <div className="flex items-center gap-1.5 flex-shrink-0 ml-2">
                {/* Active check */}
                {isActive && !isSwitching && (
                  <Check size={14} weight="bold" className="text-primary" />
                )}

                {/* Switching spinner */}
                {isSwitching && (
                  <CircleNotch size={13} weight="bold" className="text-primary animate-spin" />
                )}

                {/* Download button */}
                {!isInstalled && !isBusy && (
                  <button
                    onClick={(e) => { e.stopPropagation(); handleDownload(model.id); }}
                    className="flex items-center gap-1 px-2 py-0.5 rounded border border-dashed border-primary/40 text-primary/70 hover:border-primary hover:text-primary hover:bg-primary/5 transition-colors text-xs font-medium"
                  >
                    <DownloadSimple size={11} weight="bold" />
                    {model.size_mb} MB
                  </button>
                )}

                {/* Extracting spinner */}
                {isExtracting && (
                  <CircleNotch size={13} weight="bold" className="text-primary animate-spin" />
                )}

                {/* Cancel (downloading only) */}
                {isDownloading && (
                  <button
                    onClick={(e) => { e.stopPropagation(); handleCancel(model.id); }}
                    className="flex items-center gap-0.5 px-1.5 py-0.5 rounded text-muted-foreground hover:text-foreground hover:bg-muted transition-colors text-xs"
                  >
                    <X size={11} weight="bold" />
                    Cancel
                  </button>
                )}

                {/* Delete (installed, not active, not bundled) */}
                {model.status === 'installed' && !isActive && !isBusy && (
                  <button
                    onClick={(e) => { e.stopPropagation(); handleDelete(model.id); }}
                    className="p-1 rounded text-muted-foreground/40 hover:text-destructive hover:bg-destructive/10 transition-colors"
                  >
                    <Trash size={12} />
                  </button>
                )}
              </div>
            </div>

            {/* Progress row */}
            {(isDownloading || isExtracting) && (
              <div className="pb-2 pr-2 flex items-center gap-2">
                <div className="flex-1 h-[3px] bg-muted rounded-full overflow-hidden">
                  <div
                    className="h-full bg-primary rounded-full transition-all duration-300"
                    style={{ width: `${dl.percentage}%` }}
                  />
                </div>
                <span className="text-[10px] text-muted-foreground font-mono tabular-nums w-24 text-right">
                  {isExtracting
                    ? 'Extracting…'
                    : dl.total
                      ? `${(dl.downloaded / 1024 / 1024).toFixed(0)} / ${(dl.total / 1024 / 1024).toFixed(0)} MB`
                      : `${(dl.downloaded / 1024 / 1024).toFixed(0)} MB`}
                </span>
              </div>
            )}
          </div>
        );
      })}
    </div>
  );
}
