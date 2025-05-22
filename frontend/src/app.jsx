import { useEffect, useMemo, useState } from "react";
import { StartScreen } from "./components/start_screen";
import { invoke } from "@tauri-apps/api/tauri";
import Session from "./components/session";
import { appWindow } from "@tauri-apps/api/window";

export default function App() {
  const [projects, setProjects] = useState([]);
  const [projectKey, setProjectKey] = useState(null);
  const [darkMode, setDarkMode] = useState(true);
  const [fullscreen, setFullscreen] = useState(true);
  const project = useMemo(
    () => projects.find((p) => p.key === projectKey),
    [projects, projectKey],
  );
  const language = project?.availableLanguages[0];

  async function fetchProjects() {
    const projects = await invoke("get_projects");
    setProjects(projects);
  }

  function handleKeydown(event) {
    if (event.key === "f" && event.altKey) toggleFullScreen();
  }

  function toggleFullScreen() {
    setFullscreen((current) => {
      const new_value = !current;
      appWindow.setFullscreen(new_value);
      return new_value;
    });
  }

  useEffect(() => {
    fetchProjects();
  }, []);

  useEffect(() => {
    document.addEventListener("keydown", handleKeydown);
    return () => document.removeEventListener("keydown", handleKeydown);
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
      resetProject={() => setProjectKey(null)}
    />
  ) : (
    <StartScreen
      setProjectKey={setProjectKey}
      setDarkMode={setDarkMode}
      language={language}
      toggleFullScreen={toggleFullScreen}
      projects={projects}
    />
  );
}
