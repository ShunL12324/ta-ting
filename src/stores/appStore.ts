import { create } from 'zustand';
import { invoke } from '@tauri-apps/api/core';

export type AppState = 'idle' | 'recording' | 'transcribing' | 'inputting';

interface AppSettings {
  hotkey: string;
  auto_paste: boolean;
}

interface AppStore {
  state: AppState;
  isRecording: boolean;
  transcriptionText: string;
  error: string | null;
  hotkey: string;

  setState: (state: AppState) => void;
  setRecording: (recording: boolean) => void;
  setTranscriptionText: (text: string) => void;
  setError: (error: string | null) => void;
  setHotkey: (hotkey: string) => void;
  resetState: () => void;
  loadSettings: () => Promise<void>;
}

export const useAppStore = create<AppStore>((set) => ({
  state: 'idle',
  isRecording: false,
  transcriptionText: '',
  error: null,
  hotkey: 'Ctrl+Shift+KeyV',

  setState: (state) => set({ state }),
  setRecording: (recording) => set({ isRecording: recording }),
  setTranscriptionText: (text) => set({ transcriptionText: text }),
  setError: (error) => set({ error }),
  setHotkey: (hotkey) => set({ hotkey }),
  resetState: () =>
    set({ state: 'idle', isRecording: false, transcriptionText: '', error: null }),

  loadSettings: async () => {
    try {
      const settings = await invoke<AppSettings>('get_settings');
      set({ hotkey: settings.hotkey });
    } catch (e) {
      console.error('Failed to load settings:', e);
    }
  },
}));
