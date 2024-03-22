import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import projects from "../../projects";
import Question from "./components/question";

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
    questionInstructionSplash: "QUESTION_INSTRUCTION_SPLASH",
    questionContentSplash: "QUESTION_CONTENT_SPLASH",
    questionContentInteract: "QUESTION_CONTENT_INTERACT",
  };

  const [step, setStep] = useState(STEPS.questionInstructionSplash);
  const [sessionID, setSessionID] = useState(null);

  return (
    <>
      <div id="control-panel">
        <button onClick={() => setStep(STEPS.questionInstructionSplash)}>
          Question Instruction Splash
        </button>
        <button onClick={() => setStep(STEPS.questionContentSplash)}>
        Question Content Splash
        </button>
        <button onClick={() => setStep(STEPS.questionContentInteract)}>
        Question Content Interact
        </button>
        {sessionID && <div>Currently in session {sessionID}</div>}
      </div>
      <div className="container">
        <Question
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
