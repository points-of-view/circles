import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import projects from "../../../projects";

const previousHostname = localStorage.getItem("circles.last_hostname");

export function SelectProject({ setProject }) {
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

  return (
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
  );
}
