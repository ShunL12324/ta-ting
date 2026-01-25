import { create } from 'zustand';

export type AppState = 'idle' | 'recording' | 'transcribing' | 'inputting';

interface AppStore {
  state: AppState;
  isRecording: boolean;
  transcriptionText: string;
  error: string | null;

  setState: (state: AppState) => void;
  setRecording: (recording: boolean) => void;
  setTranscriptionText: (text: string) => void;
  setError: (error: string | null) => void;
  resetState: () => void;
}

export const useAppStore = create<AppStore>((set) => ({
  state: 'idle',
  isRecording: false,
  transcriptionText: '',
  error: null,

  setState: (state) => set({ state }),
  setRecording: (recording) => set({ isRecording: recording }),
  setTranscriptionText: (text) => set({ transcriptionText: text }),
  setError: (error) => set({ error }),
  resetState: () =>
    set({
      state: 'idle',
      isRecording: false,
      transcriptionText: '',
      error: null,
    }),
}));
