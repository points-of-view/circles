import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import projects from "../../projects";
import ThemeSelector from "./components/theme_selector";
import ChooseThemeSplash from "./components/choose_theme_splash";

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
  const [step, setStep] = useState("ChooseThemeSplash");
  const [sessionID, setSessionID] = useState(null);

  return (
    <div>
      <div id="control-panel">
        <button onClick={() => setStep("ChooseThemeSplash")}>
          Choose Theme Splash
        </button>
        <button onClick={() => setStep("OpenThemeSession")}>
          Open Theme Session
        </button>
        {sessionID && <div>Currently in session {sessionID}</div>}
      </div>
      <div className="container">
        {step === "OpenThemeSession" ? (
          <ThemeSelector
            project={project}
            language={language}
            resetProject={resetProject}
            sessionID={sessionID}
            setSessionID={setSessionID}
          />
        ) : (
          step === "ChooseThemeSplash" && (
            <ChooseThemeSplash project={project} language={language} />
          )
        )}
      </div>
    </div>
  );
}
