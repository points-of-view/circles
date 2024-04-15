import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { InteractionScreen } from "./interaction-screen";
import ControlPanel from "./control-panel";
import { listen } from "@tauri-apps/api/event";
import shuffle from "../utils/shuffle";

export default function Session({ project, resetProject, language }) {
  const [, setTagsMap] = useState({});
  const [, setReaderError] = useState(null);
  const [error, setError] = useState(null);
  const [sessionID, setSessionID] = useState(null);
  const [phase, setPhase] = useState(0);
  const [themes, setThemes] = useState([]);
  const [chosenThemeKey, setChosenThemeKey] = useState(null);

  const chosenTheme = project.themes.find(
    (item) => item.key === chosenThemeKey,
  );

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
    document.addEventListener("keydown", handleKeyDown);
    return () => document.removeEventListener("keydown", handleKeyDown);
  }, []);

  useEffect(() => {
    if (phase === 0) {
      setThemes(shuffle(project.themes));
    }
  }, [phase]);

  function goToNextPhase() {
    if (chosenTheme === undefined && phase === 0) {
      throw new Error("A theme should be chosen when going to phase != 0");
    }
    setPhase(chosenTheme.questions[phase] === undefined ? 0 : phase + 1);
  }

  function handleKeyDown(e) {
    if (e.code === "ArrowRight") {
      if (phase === 0) {
        setChosenThemeKey(themes[0].key);
      }
      goToNextPhase();
    }
  }

  return (
    <>
      {import.meta.env.VITE_DEV_MODE && (
        <ControlPanel
          phase={phase}
          project={project}
          language={language}
          startNewSession={startNewSession}
          error={error}
          sessionID={sessionID}
          setChosenThemeKey={setChosenThemeKey}
          goToNextPhase={goToNextPhase}
          options={
            (phase === 0 && themes.slice(0, 3)) ||
            chosenTheme.questions[phase - 1].options
          }
        />
      )}
      {phase === 0 ? (
        <InteractionScreen
          title={"Choose a theme of your choice"}
          description={"Stand in the circle of your answer"}
          options={themes.slice(0, 3).map((a) => a.name[language])}
        />
      ) : (
        <InteractionScreen
          title={chosenTheme.questions[phase - 1].title[language]}
          description={"Stand in the circle of your answer"}
          options={chosenTheme.questions[phase - 1].options.map(
            (a) => a.value[language],
          )}
          themeName={chosenTheme.name[language]}
          phase={phase}
        />
      )}
    </>
  );
}
