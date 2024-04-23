import { useEffect, useState } from "react";
import { SelectProject } from "./components/select_project";
import { invoke } from "@tauri-apps/api/tauri";
import Session from "./components/session";

export default function App() {
  const [project, setProject] = useState(null);
  const language = project?.availableLanguages[0];

  // If this component gets destroyed (on refresh, or on window exit), we stop our reader
  useEffect(() => {
    return () => invoke("close_connection");
  }, []);

  return project ? (
    <Session
      project={project}
      language={language}
      resetProject={() => setProject(null)}
    />
  ) : (
    <SelectProject setProject={setProject} language={language} />
  );
}
