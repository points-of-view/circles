import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";

export default function ProjectSelector({project, resetProject, language}) {
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

  return (
    <div>
      {project.name[language]}
      <form action="" onSubmit={startNewSession}>
        <input type="text" name="themeKey" id="themeKey" required />
        <button type="submit">Start new session</button>
        {error && <span>{error.toString()}</span>}
      </form>

      {sessionID && <div>Currently in session {sessionID}</div>}
    </div>
  );
}