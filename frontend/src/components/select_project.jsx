import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { save } from "@tauri-apps/api/dialog";
import projects from "../../../projects";

const previousHostname = localStorage.getItem("circles.last_hostname");

const EXPORT_STATE = {
  idle: "IDLE",
  working: "WORKING",
  done: "DONE",
};

export function SelectProject({ setProject }) {
  const [exportState, setExportState] = useState(EXPORT_STATE.idle);
  const [error, setError] = useState(null);

  async function handleSubmit(e) {
    e.preventDefault();
    const data = new FormData(e.target);
    const projectKey = data.get("projectKey");
    const hostname = data.get("hostname");
    try {
      await invoke("select_project", { projectKey, hostname });
      setProject(projects[projectKey]);
      localStorage.setItem("circles.last_hostname", hostname);
    } catch (e) {
      setError(e);
    }
  }

  async function saveExport() {
    const filepath = await save({
      filters: [
        {
          name: "export",
          extensions: ["xlsx"],
        },
      ],
    });
    try {
      setExportState(EXPORT_STATE.working);
      await invoke("save_export", { filepath });
      setExportState(EXPORT_STATE.done);
    } catch (e) {
      setError(e);
      setExportState(EXPORT_STATE.idle);
    }
  }

  return (
    <>
      <form action="" onSubmit={handleSubmit} style={{ color: "black" }}>
        <div>
          <label htmlFor="projectKey">Project key</label>
          <input type="text" name="projectKey" id="projectKey" required />
        </div>
        <div>
          <label htmlFor="hostname">Hostname</label>
          <input
            type="text"
            name="hostname"
            id="hostname"
            placeholder="fx9600123456"
            minLength={12}
            maxLength={12}
            required
            defaultValue={previousHostname}
          />
        </div>
        <button type="submit">Open project</button>
        {error && <span>{error}</span>}
      </form>

      <button onClick={saveExport} disabled={exportState !== EXPORT_STATE.idle}>
        Export all data
      </button>
      {exportState === EXPORT_STATE.working && (
        <span className="" style={{ color: "black" }}>
          Export data. Please wait...
        </span>
      )}
      {exportState === EXPORT_STATE.done && (
        <span className="" style={{ color: "black" }}>
          Export is done
        </span>
      )}
    </>
  );
}
