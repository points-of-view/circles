import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import projects from "../../projects";

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
  const [error, setError] = useState(null);
  const [sessionID, setSessionID] = useState(null);
  const [reading, setReading] = useState(false);

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

  async function toggleReading() {
    try {
      // Invoke with the updated reading state
      const status = await invoke("toggle_reading");
      setReading(status);
    } catch (e) {
      setError(e);
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

      {sessionID && (
        <div>
          <div>Currently in session {sessionID}</div>{" "}
            <div>
              <span><button onClick={() => toggleReading()}>Toggle reading</button>Reading: {reading.toString()}</span>
            </div>
        </div>
      )}
    </div>
  );
}
