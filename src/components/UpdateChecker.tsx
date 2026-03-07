import { useEffect } from 'react';
import { listen } from '@tauri-apps/api/event';
import { check } from '@tauri-apps/plugin-updater';
import { toast } from 'sonner';

export function UpdateChecker() {
  useEffect(() => {
    const unlisten = listen('check_update_requested', async () => {
      console.log('收到检查更新请求');
      try {
        const update = await check();
        if (update) {
          console.log('发现新版本:', update.version);
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
        toast.error('检查更新失败', { description: String(error) });
      }
    });

    return () => {
      unlisten.then((fn) => fn());
    };
  }, []);

  return null;
}
