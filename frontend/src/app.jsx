import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import projects from "../../projects";
import ThemeSelector from "./components/theme_selector";

export default function App() {
  const [project, setProject] = useState(null);
  const language = project?.availableLanguages[0];

  return project ? (
    <Session
      project={project}
      language={language}
      resetProject={() => setProject(null)}
    />
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
  const STEPS = {
    chooseThemeSplash: "CHOOSE_THEME_SPLASH",
    themeSelector: "CHOOSE_THEME_SESSION",
  };

  const [step, setStep] = useState(STEPS.chooseThemeSplash);
  const [sessionID, setSessionID] = useState(null);

  return (
    <>
      <div id="control-panel">
        <button onClick={() => setStep(STEPS.chooseThemeSplash)}>
          Choose Theme Splash
        </button>
        <button onClick={() => setStep(STEPS.themeSelector)}>
          Open Theme Session
        </button>
        {sessionID && <div>Currently in session {sessionID}</div>}
      </div>
      <div className="container">
        <ThemeSelector
          project={project}
          language={language}
          resetProject={resetProject}
          sessionID={sessionID}
          setSessionID={setSessionID}
          step={step}
          STEPS={STEPS}
        />
      </div>
    </>
  );
}
