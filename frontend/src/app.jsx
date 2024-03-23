import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import projects from "../../projects";
import Question from "./components/question";
import ControlPanel from "./components/control_panel";

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
    questionContentAnswerValue: "QUESTION_CONTENT_ANSWER_VALUE",
    questionContentAnswerDescription: "QUESTION_CONTENT_ANSWER_DESCRIPTION",
    questionInstructionOpinion: "QUESTION_INSTRUCTION_OPINION",
  };

  // NOTE: This is a dummy tag object, to be replaced by actual read tags.
  const tags = [
    { label: "ABD", option: 3 },
    { label: "ABC", option: 1 },
    { label: "ABB", option: 1 },
    { label: "ABA", option: 3 },
    { label: "ABD", option: 3 },
    { label: "ABC", option: 2 },
    { label: "ABB", option: 1 },
    { label: "ABA", option: 3 },
    { label: "ABC", option: 2 },
    { label: "ABC", option: 3 },
    { label: "ABC", option: 1 },
  ];

  const [step, setStep] = useState(STEPS.questionInstructionSplash);
  const [sessionID, setSessionID] = useState(null);
  const [error, setError] = useState(null);

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
    <>
      <ControlPanel
        step={step}
        project={project}
        language={language}
        setStep={setStep}
        startNewSession={startNewSession}
        error={error}
        STEPS={STEPS}
        sessionID={sessionID}
        setSessionID={setSessionID}
      />
      <Question
        project={project}
        language={language}
        step={step}
        STEPS={STEPS}
        tags={tags}
      />
    </>
  );
}
