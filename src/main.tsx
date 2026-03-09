import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import "./styles/global.css";
import "./i18n";
import App from "./App.tsx";
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";
import { useAppStore } from "./stores/appStore";

// 设置事件监听器
async function setupEventListeners() {
  console.log("设置后端事件监听器...");

  // Load persisted settings
  await useAppStore.getState().loadSettings();

  // 监听状态变化事件
  await listen<string>("state_changed", (event) => {
    const state = event.payload as "idle" | "recording" | "transcribing" | "inputting";
    console.log("状态变化:", state);

    const store = useAppStore.getState();
    store.setState(state);
    store.setRecording(state === "recording");
  });

  // 监听转录结果
  await listen<string>("transcription_result", (event) => {
    const text = event.payload;
    console.log("转录结果:", text);

    useAppStore.getState().setTranscriptionText(text);
  });

  // 监听错误
  await listen<string>("error", (event) => {
    const errorMsg = event.payload;
    console.error("后端错误:", errorMsg);

    useAppStore.getState().setError(errorMsg);

    // 可选：显示错误提示
    // alert(`错误: ${errorMsg}`);
  });

  // Listen for tray menu "start_dictation" event
  await listen('hotkey_pressed', async () => {
    await invoke('trigger_hotkey');
  });

  console.log("✅ 事件监听器设置完成");
}

// 启动应用
setupEventListeners()
  .then(() => {
    console.log("启动 React 应用...");
    createRoot(document.getElementById("root")!).render(
      <StrictMode>
        <App />
      </StrictMode>
    );
  })
  .catch((error) => {
    console.error("设置事件监听器失败:", error);
    // 即使监听器设置失败也渲染应用
    createRoot(document.getElementById("root")!).render(
      <StrictMode>
        <App />
      </StrictMode>
    );
  });
