import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";

function App() {
  const [sessionID, setSessionID] = useState(null);

  async function startNewSession(e) {
    e.preventDefault();
    const data = new FormData(e.target);

    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    const response = await invoke("start_session", {
      projectKey: data.get("projectKey"),
      themeKey: data.get("themeKey"),
    });
    setSessionID(response);
  }

  return (
    <div>
      <form action="" onSubmit={startNewSession}>
        <input type="text" name="projectKey" id="projectKey" required />
        <input type="text" name="themeKey" id="themeKey" required />
        <button type="submit">Start new session</button>
      </form>

      {sessionID && <div>Currently in session {sessionID}</div>}
    </div>
  );
}

export default App;
