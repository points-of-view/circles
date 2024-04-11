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
  const [step, setStep] = useState(PHASES.pickTheme);
  const [themes, setThemes] = useState([]);

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

    if (step === PHASES.pickTheme) {
      shuffleThemes(3);
    }

    // Don't forget to clean up
    return function cleanup() {
      document.removeEventListener('keydown', handleKeyDown);
    }
  }, []);

  function shuffleThemes(amountOfThemes) {
    let myArray = [];
    while (myArray.length < amountOfThemes) {
      let newRandomInt = Math.floor(Math.random() * project.themes.length);
      if (!myArray.includes(newRandomInt)) {
        myArray.push(newRandomInt);
        setThemes(oldArray => [...oldArray, project.themes[newRandomInt].name[language]]);
      }
    }
  }

  return (
    <>
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
      {step === PHASES.pickTheme ? (<InteractionScreen
        title={"title"}
        description={"description"}
        options={themes}
      />) : (<InteractionScreen
        title={"title"}
        description={"description"}
        theme={"theme"}
        options={themes}
      />)}
    </>
  );
}
