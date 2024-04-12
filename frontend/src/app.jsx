import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { listen } from "@tauri-apps/api/event";
import projects from "../../projects";
import { InteractionScreen } from "./components/interaction-screen";

export default function App() {
  const [project, setProject] = useState(null);
  const language = project?.availableLanguages[0];

  return project ? (
    <Session project={project} resetProject={() => setProject(null)} />
  ) : (
    <SelectProject setProject={setProject} language={language} />
  );
}

function SelectProject({ setProject }) {
  const [error, setError] = useState(null);

  async function handleSubmit(e) {
    e.preventDefault();
    const data = new FormData(e.target);
    const projectKey = data.get("projectKey");
    try {
      await invoke("select_project", { projectKey });
      setProject(projects[projectKey]);
    } catch (e) {
      setError(e);
    }
  }

  return (
    <form action="" onSubmit={handleSubmit}>
      <input type="text" name="projectKey" id="projectKey" required />
      <button type="submit">Open project</button>
      {error && <span>{error}</span>}
    </form>
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
    document.addEventListener("keydown", handleKeyDown);
    return () => document.removeEventListener("keydown", handleKeyDown);
  }, []);

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
  
  function handleKeyDown(e) {
    if (e.code === "ArrowRight") {
      console.log("proceed"); // eslint-disable-line
    } else if (e.code === "ArrowLeft") {
      console.log("previous"); // eslint-disable-line
    }
  }

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
