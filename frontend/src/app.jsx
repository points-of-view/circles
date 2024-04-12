import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { listen } from "@tauri-apps/api/event";
import { InteractionScreen } from "./components/interaction-screen";
import { SelectProject } from "./components/select_project";
import ControlPanel from "./components/control-panel";

export const PHASES = {
  pickTheme: "pickTheme",
  showQuestion: "showQuestion",
  showOpinionQuestion: "showOpinionQuestion",
};

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

function Session({ project, resetProject, language }) {
  const [, setTagsMap] = useState({});
  const [, setReaderError] = useState(null);
  const [error, setError] = useState(null);
  const [sessionID, setSessionID] = useState(null);
  const [phase, setPhase] = useState(PHASES.pickTheme);
  const [themesIndexes, setThemesIndexes] = useState([]);
  const [chosenTheme, setChosenTheme] = useState(null);

  async function startNewSession(themeKey) {
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
        console.log("proceed"); // eslint-disable-line
      } else if (e.keyCode === 37) {
        console.log("previous"); // eslint-disable-line
      }
    }

    document.addEventListener("keydown", handleKeyDown);

    // Don't forget to clean up
    return function cleanup() {
      document.removeEventListener("keydown", handleKeyDown);
    };
  }, []);

  useEffect(() => {
    if (shuffleThemes && phase === PHASES.pickTheme) {
      shuffleThemes(3);
    }
  }, [phase]);

  function shuffleThemes(amountOfThemes) {
    let myArray = [];
    setThemesIndexes([]);
    while (myArray.length < amountOfThemes) {
      let newRandomInt = Math.floor(Math.random() * project.themes.length);
      if (!myArray.includes(newRandomInt)) {
        myArray.push(newRandomInt);
        setThemesIndexes((oldArray) => [...oldArray, newRandomInt]);
      }
    }
  }

  function importThemeCopy() {
    if (chosenTheme !== null) {
      return project.themes.find((item) => item.key === chosenTheme);
    } else {
      return project.themes.find((item) => item.key === "migration");
    }
  }

  return (
    <>
      {import.meta.env.VITE_DEV_MODE && (
        <ControlPanel
          phase={phase}
          setPhase={setPhase}
          project={project}
          language={language}
          startNewSession={startNewSession}
          error={error}
          sessionID={sessionID}
          setSessionID={setSessionID}
          setChosenTheme={setChosenTheme}
          options={
            phase === PHASES.pickTheme
              ? themesIndexes
              : phase === PHASES.showQuestion
                ? importThemeCopy().questions[0].options.map(
                  (a) => a.value[language],
                )
                : importThemeCopy().questions[1].options.map(
                  (a) => a.value[language],
                )
          }
        />
      )}
      {phase === PHASES.pickTheme && (
        <InteractionScreen
          title={"Choose a theme of your choice"}
          description={"Stand in the circle of your answer"}
          options={themesIndexes.map((a) => project.themes[a].name[language])}
        />
      )}
      {phase === PHASES.showQuestion && (
        <InteractionScreen
          title={importThemeCopy().questions[0].title[language]}
          description={"Stand in the circle of your answer"}
          options={importThemeCopy().questions[0].options.map(
            (a) => a.value[language],
          )}
          themeName={importThemeCopy().name[language]}
          phase={phase}
        />
      )}
      {phase === PHASES.showOpinionQuestion && (
        <InteractionScreen
          title={importThemeCopy().questions[1].title[language]}
          description={"Stand in the circle of your answer"}
          options={importThemeCopy().questions[1].options.map(
            (a) => a.value[language],
          )}
          themeName={importThemeCopy().name[language]}
          phase={phase}
        />
      )}
    </>
  );
}
