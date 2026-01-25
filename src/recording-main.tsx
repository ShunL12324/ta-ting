import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import { RecordingWindow } from "./pages/RecordingWindow";
import "./styles/global.css";

createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <RecordingWindow />
  </StrictMode>,
);
