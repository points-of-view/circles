import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import projects from "../../projects";
import ProjectSelector from "./components/project_selector";
import Theme from "./components/theme";

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
  const [step, setStep] = useState("theme");

  return (
    <div>
      <div className="container">
        {step === "open project" ? (
          <ProjectSelector
            project={project}
            language={language}
            resetProject={resetProject}
          />
        ) : (
          step === "theme" && <Theme />
        )}
      </div>
    </div>
  );
}
