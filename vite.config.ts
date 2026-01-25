import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import path from "path";

// https://vite.dev/config/
export default defineConfig({
  plugins: [react()],

  // Tauri expects a fixed port for development
  server: {
    port: 5173,
    strictPort: true,
    host: '127.0.0.1', // 明确绑定到 IPv4，避免 Tauri 连接问题
  },

  // 路径别名
  resolve: {
    alias: {
      "@": path.resolve(__dirname, "./src"),
    },
  },

  // Build optimization
  build: {
    target: "esnext",
    minify: "esbuild",
    rollupOptions: {
      input: {
        main: path.resolve(__dirname, 'index.html'),
        recording: path.resolve(__dirname, 'recording.html'),
      },
    },
  },
});
