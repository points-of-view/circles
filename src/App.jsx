import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";

function App() {
  const [sessionID, setSessionID] = useState(null);

  async function startNewSession() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    const response = await invoke("start_session");
    setSessionID(response);
  }

  return (
    <div>
      <button onClick={startNewSession}>Start new session</button>

      {sessionID && <div>Currently in session {sessionID}</div>}
    </div>
  );
}

export default App;
