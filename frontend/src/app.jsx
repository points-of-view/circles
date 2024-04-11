import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { listen } from "@tauri-apps/api/event";
import { InteractionScreen } from "./components/interaction-screen";
import { SelectProject } from "./components/select_project";

export default function App() {
  const [project, setProject] = useState(null);
  const language = project?.availableLanguages[0];

  return project ? (
    <Session project={project} language={language} resetProject={() => setProject(null)} />
  ) : (
    <SelectProject setProject={setProject} language={language} />
  );
}

function Session({ project, resetProject, language }) {
  const [, setTagsMap] = useState({});
  const [, setReaderError] = useState(null);
  const [error, setError] = useState(null);
  const [sessionID, setSessionID] = useState(null);

  async function startNewSession(e) {
    e.preventDefault();
    const data = new FormData(e.target);

    const themeKey = data.get("themeKey");

    try {
      const response = await invoke("start_session", { themeKey });
      setSessionID(response);
    } catch (e) {
      if (e === "Please select a project first") {
        resetProject();
      }
      setError(e);
    }
  }

  useEffect(() => {
    const unlisten = listen("updated-tags", ({ payload }) =>
      setTagsMap(payload),
    );

    return () => unlisten.then((fn) => fn());
  }, []);

  useEffect(() => {
    const unlisten = listen("reader-error", ({ payload }) =>
      setReaderError(payload),
    );

    return () => unlisten.then((fn) => fn());
  }, []);

  return (
    <div>
      {project.name[language]}
      <form action="" onSubmit={startNewSession}>
        <input type="text" name="themeKey" id="themeKey" required />
        <button type="submit">Start new session</button>
        {error && <span>{error}</span>}
      </form>

      {sessionID && <div>Currently in session {sessionID}</div>}
      <InteractionScreen
        title={"title"}
        description={"description"}
        theme={"theme"}
      />
    </div>
  );
}
