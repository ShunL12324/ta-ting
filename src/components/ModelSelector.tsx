import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { listen, type UnlistenFn } from '@tauri-apps/api/event';
import { Download, Check, Loader2, Trash2, AlertCircle, Globe } from 'lucide-react';
import { Button } from './ui/button';
import { Badge } from './ui/badge';
import { Label } from './ui/label';

export interface ModelInfo {
  id: string;
  name: string;
  description: string;
  language: string;
  size_mb: number;
  status: 'bundled' | 'installed' | 'not_installed' | 'downloading';
  is_bundled: boolean;
}

interface DownloadProgress {
  model_id: string;
  downloaded: number;
  total: number | null;
  percentage: number;
  status: string;
}

export function ModelSelector() {
  const [models, setModels] = useState<ModelInfo[]>([]);
  const [activeModelId, setActiveModelId] = useState<string>('');
  const [downloadProgress, setDownloadProgress] = useState<Record<string, DownloadProgress>>({});
  const [error, setError] = useState<string | null>(null);
  const [isLoading, setIsLoading] = useState(true);

  // Load models on mount
  useEffect(() => {
    loadModels();
    loadActiveModel();
  }, []);

  // Listen for download progress events
  useEffect(() => {
    let unlistenProgress: UnlistenFn | null = null;
    let unlistenComplete: UnlistenFn | null = null;

    const setupListeners = async () => {
      unlistenProgress = await listen<DownloadProgress>('model_download_progress', (event) => {
        setDownloadProgress((prev) => ({
          ...prev,
          [event.payload.model_id]: event.payload,
        }));
      });

      unlistenComplete = await listen<string>('model_download_complete', (event) => {
        setDownloadProgress((prev) => {
          const next = { ...prev };
          delete next[event.payload];
          return next;
        });
        loadModels();
      });
    };

    setupListeners();

    return () => {
      unlistenProgress?.();
      unlistenComplete?.();
    };
  }, []);

  const loadModels = async () => {
    try {
      setIsLoading(true);
      const result = await invoke<ModelInfo[]>('get_available_models');
      setModels(result);
      setError(null);
    } catch (e) {
      setError(`加载模型列表失败: ${e}`);
    } finally {
      setIsLoading(false);
    }
  };

  const loadActiveModel = async () => {
    try {
      const result = await invoke<string>('get_active_model');
      setActiveModelId(result);
    } catch (e) {
      console.error('Failed to load active model:', e);
    }
  };

  const handleSelectModel = async (modelId: string) => {
    try {
      await invoke('set_active_model', { modelId });
      setActiveModelId(modelId);
    } catch (e) {
      setError(`切换模型失败: ${e}`);
      setTimeout(() => setError(null), 3000);
    }
  };

  const handleDownload = async (modelId: string) => {
    try {
      setError(null);
      await invoke('download_model', { modelId });
    } catch (e) {
      setError(`下载失败: ${e}`);
      setTimeout(() => setError(null), 5000);
    }
  };

  const handleCancelDownload = async (modelId: string) => {
    try {
      await invoke('cancel_model_download', { modelId });
      setDownloadProgress((prev) => {
        const next = { ...prev };
        delete next[modelId];
        return next;
      });
    } catch (e) {
      console.error('Failed to cancel download:', e);
    }
  };

  const handleDelete = async (modelId: string) => {
    if (!confirm('确定要删除这个模型吗？')) return;

    try {
      await invoke('delete_model', { modelId });
      await loadModels();
    } catch (e) {
      setError(`删除失败: ${e}`);
      setTimeout(() => setError(null), 3000);
    }
  };

  const getStatusBadge = (model: ModelInfo) => {
    const progress = downloadProgress[model.id];

    if (progress) {
      return (
        <div className="flex items-center gap-1.5">
          <div className="relative w-16 h-1.5 bg-muted rounded-full overflow-hidden">
            <div
              className="absolute inset-y-0 left-0 bg-primary transition-all duration-300"
              style={{ width: `${progress.percentage}%` }}
            />
          </div>
          <span className="text-[10px] text-primary font-medium">
            {progress.percentage.toFixed(0)}%
          </span>
        </div>
      );
    }

    switch (model.status) {
      case 'bundled':
        return <Badge variant="success">内置</Badge>;
      case 'installed':
        return <Badge variant="secondary">已安装</Badge>;
      case 'downloading':
        return (
          <Badge variant="warning" className="flex items-center gap-1">
            <Loader2 className="w-3 h-3 animate-spin" />
            下载中
          </Badge>
        );
      default:
        return <Badge variant="outline">{model.size_mb} MB</Badge>;
    }
  };

  if (isLoading) {
    return (
      <div className="flex items-center justify-center py-4">
        <Loader2 className="w-5 h-5 text-muted-foreground animate-spin" />
      </div>
    );
  }

  return (
    <div className="space-y-2">
      <Label className="flex items-center gap-1 text-xs font-semibold mb-1.5">
        <Globe className="w-3.5 h-3.5" />
        语音识别模型
      </Label>

      {error && (
        <div className="flex items-center gap-1.5 p-2 bg-destructive/10 text-destructive rounded-lg text-xs mb-2">
          <AlertCircle className="w-3.5 h-3.5 flex-shrink-0" />
          <span>{error}</span>
        </div>
      )}

      <div className="space-y-1.5">
        {models.map((model) => {
          const isActive = model.id === activeModelId;
          const isInstalled = model.status === 'bundled' || model.status === 'installed';
          const isDownloading = !!downloadProgress[model.id] || model.status === 'downloading';

          return (
            <div
              key={model.id}
              className={`relative p-2.5 rounded-lg border-2 transition-all duration-200 ${
                isActive
                  ? 'border-primary bg-primary/10'
                  : 'border-border bg-muted hover:border-input'
              }`}
            >
              <div className="flex items-start justify-between gap-2">
                <div className="flex-1 min-w-0">
                  <div className="flex items-center gap-1.5">
                    <span className="text-sm font-medium text-foreground truncate">
                      {model.name}
                    </span>
                    {isActive && (
                      <Check className="w-3.5 h-3.5 text-primary flex-shrink-0" />
                    )}
                  </div>
                  <p className="text-[10px] text-muted-foreground mt-0.5">{model.description}</p>
                </div>

                <div className="flex items-center gap-1.5 flex-shrink-0">
                  {getStatusBadge(model)}

                  {isInstalled && !isActive && (
                    <Button
                      size="sm"
                      onClick={() => handleSelectModel(model.id)}
                      className="h-auto py-0.5 px-2 text-[10px]"
                    >
                      使用
                    </Button>
                  )}

                  {!isInstalled && !isDownloading && (
                    <Button
                      variant="ghost"
                      size="icon"
                      onClick={() => handleDownload(model.id)}
                      title="下载"
                      className="w-6 h-6"
                    >
                      <Download className="w-4 h-4" />
                    </Button>
                  )}

                  {isDownloading && (
                    <Button
                      variant="secondary"
                      size="sm"
                      onClick={() => handleCancelDownload(model.id)}
                      className="h-auto py-0.5 px-2 text-[10px]"
                    >
                      取消
                    </Button>
                  )}

                  {model.status === 'installed' && !model.is_bundled && (
                    <Button
                      variant="ghost"
                      size="icon"
                      onClick={() => handleDelete(model.id)}
                      title="删除"
                      className="w-6 h-6 text-destructive hover:text-destructive hover:bg-destructive/10"
                    >
                      <Trash2 className="w-4 h-4" />
                    </Button>
                  )}
                </div>
              </div>
            </div>
          );
        })}
      </div>

      <p className="text-[10px] text-muted-foreground flex items-center gap-1 pt-1">
        <span className="w-1 h-1 rounded-full bg-success" />
        内置模型随安装包提供，其他模型需下载
      </p>
    </div>
  );
}
