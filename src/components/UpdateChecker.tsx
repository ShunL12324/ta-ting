import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { check } from '@tauri-apps/plugin-updater';
import { relaunch } from '@tauri-apps/plugin-process';
import { Button } from './ui/button';
import { toast } from 'sonner';

export function UpdateChecker() {
  const [isChecking, setIsChecking] = useState(false);
  const [updateAvailable, setUpdateAvailable] = useState(false);
  const [updateVersion, setUpdateVersion] = useState('');
  const [isDownloading, setIsDownloading] = useState(false);
  const [downloadProgress, setDownloadProgress] = useState(0);

  useEffect(() => {
    // 监听托盘菜单的"检查更新"事件
    const unlisten = listen('check_update_requested', async () => {
      console.log('收到检查更新请求');
      await handleCheckUpdate();
    });

    return () => {
      unlisten.then((fn) => fn());
    };
  }, []);

  const handleCheckUpdate = async () => {
    setIsChecking(true);
    try {
      console.log('开始检查更新...');
      const update = await check();

      if (update) {
        console.log('发现新版本:', update.version);
        setUpdateAvailable(true);
        setUpdateVersion(update.version);
        toast.success(`发现新版本 ${update.version}`, {
          description: '点击"立即更新"按钮下载安装',
          duration: 10000,
        });
      } else {
        console.log('已是最新版本');
        toast.info('已是最新版本');
      }
    } catch (error) {
      console.error('检查更新失败:', error);
      toast.error('检查更新失败', {
        description: String(error),
      });
    } finally {
      setIsChecking(false);
    }
  };

  const handleInstallUpdate = async () => {
    setIsDownloading(true);
    try {
      const update = await check();
      if (!update) {
        toast.info('没有可用更新');
        return;
      }

      console.log('开始下载更新...');
      toast.info('开始下载更新...', {
        description: '请稍候，下载完成后会自动安装',
      });

      // 下载并安装更新
      await update.downloadAndInstall((event) => {
        switch (event.event) {
          case 'Started':
            console.log('下载开始:', event.data);
            setDownloadProgress(0);
            break;
          case 'Progress':
            const progress = Math.round(
              (event.data.downloaded / event.data.contentLength) * 100
            );
            console.log(`下载进度: ${progress}%`);
            setDownloadProgress(progress);
            break;
          case 'Finished':
            console.log('下载完成');
            setDownloadProgress(100);
            toast.success('更新下载完成，正在安装...', {
              description: '应用将在 3 秒后重启',
            });
            break;
        }
      });

      // 重启应用以应用更新
      setTimeout(async () => {
        console.log('重启应用以应用更新');
        await relaunch();
      }, 3000);
    } catch (error) {
      console.error('更新失败:', error);
      toast.error('更新失败', {
        description: String(error),
      });
    } finally {
      setIsDownloading(false);
    }
  };

  // 这个组件主要在后台工作，不需要渲染 UI
  // 如果需要显示更新按钮，可以放在设置面板中
  return null;
}
