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
    questionContentAnswerValue: "QUESTION_CONTENT_ANSWER_VALUE",
    questionContentAnswerDescription: "QUESTION_CONTENT_ANSWER_DESCRIPTION",
    questionInstructionOpinion: "QUESTION_INSTRUCTION_OPINION",
  };

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

  const handleStepChange = (event) => {
    const selectedStep = event.target.value;
    setStep(selectedStep);
  };

  return (
    <>
      <div id="control-panel">
        <select value={step} onChange={handleStepChange}>
          <option value={STEPS.questionInstructionSplash}>
            Question Instruction Splash
          </option>
          <option value={STEPS.questionContentSplash}>
            Question Content Splash
          </option>
          <option value={STEPS.questionContentInteract}>
            Question Content Interact
          </option>
          <option value={STEPS.questionContentAnswerValue}>
            Question Content Answer Value
          </option>
          <option value={STEPS.questionContentAnswerDescription}>
            Question Content Answer Description
          </option>
          <option value={STEPS.questionInstructionOpinion}>
            Question Instruction Opinion
          </option>
        </select>
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
          tags={tags}
        />
      </div>
    </>
  );
}
