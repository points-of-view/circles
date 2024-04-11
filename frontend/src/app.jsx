import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { listen } from "@tauri-apps/api/event";
import { InteractionScreen } from "./components/interaction-screen";
import { SelectProject } from "./components/select_project";
import ControlPanel from "./components/control-panel";

export const PHASES = {
  pickTheme: "pickTheme",
  showQuestion: "showQuestion", 
  showOpinionQuestion: "showOpinionQuestion" 
};

export default function App() {
  const [project, setProject] = useState(null);
  const language = project?.availableLanguages[0];

  return project ? (
    <Session project={project} language={language} resetProject={() => setProject(null)} />
  ) : (
    <SelectProject setProject={setProject} language={language} />
  );
}

function Session({ project, resetProject, language }) {
  const [, setTagsMap] = useState({});
  const [, setReaderError] = useState(null);
  const [error, setError] = useState(null);
  const [sessionID, setSessionID] = useState(null);
  const [step, setStep] = useState(PHASES.questionInstructionSplash);

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

  useEffect(() => {
    const unlisten = listen("updated-tags", ({ payload }) =>
      setTagsMap(payload),
    );

    return () => unlisten.then((fn) => fn());
  }, []);

  useEffect(() => {
    const unlisten = listen("reader-error", ({ payload }) =>
      setReaderError(payload),
    );

    return () => unlisten.then((fn) => fn());
  }, []);

  useEffect(() => {
    function handleKeyDown(e) {
      if (e.keyCode === 39) {
        console.log("proceed")
      } else if (e.keyCode === 37) {
        console.log("previous")
      }
    }

    document.addEventListener('keydown', handleKeyDown);

    // Don't forget to clean up
    return function cleanup() {
      document.removeEventListener('keydown', handleKeyDown);
    }
  }, []);

  return (
    <div>
      {import.meta.env.VITE_DEV_MODE &&
      <ControlPanel
        step={step}
        setStep={setStep}
        project={project}
        language={language}
        startNewSession={startNewSession}
        error={error}
        sessionID={sessionID}
        setSessionID={setSessionID}
      />}
      <InteractionScreen
        title={"title"}
        description={"description"}
        theme={"theme"}
      />
    </div>
  );
}
