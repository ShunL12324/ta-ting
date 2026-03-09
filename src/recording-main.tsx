import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import "@fontsource-variable/inter";
import "@fontsource/noto-sans-sc/400.css";
import { RecordingWindow } from "./pages/RecordingWindow";
import "./styles/global.css";

createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <RecordingWindow />
  </StrictMode>,
);
