import { useEffect, useState } from "react";
import { StartScreen } from "./components/start_screen";
import { invoke } from "@tauri-apps/api/tauri";
import Session from "./components/session";
import { appWindow } from "@tauri-apps/api/window";

export default function App() {
  const [project, setProject] = useState(null);
  const [darkMode, setDarkMode] = useState(true);
  const [fullscreen, setFullscreen] = useState(true);
  const language = project?.availableLanguages[0];

  function toggleFullScreen() {
    appWindow.setFullscreen(!fullscreen);
    setFullscreen(!fullscreen);
  };

  useEffect(() => {
    document.addEventListener("dblclick", toggleFullScreen);
    return () => document.removeEventListener("dblclick", toggleFullScreen);
  }, [fullscreen]);

  // If this component gets destroyed (on refresh, or on window exit), we stop our reader
  useEffect(() => {
    return () => invoke("close_connection");
  }, []);

  return project ? (
    <Session
      project={project}
      language={language}
      darkMode={darkMode}
      resetProject={() => setProject(null)}
    />
  ) : (
    <StartScreen
      setProject={setProject}
      setDarkMode={setDarkMode}
      language={language}
    />
  );
}
