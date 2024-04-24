import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { InteractionScreen } from "./interaction-screen";
import ControlPanel from "./control-panel";
import { listen } from "@tauri-apps/api/event";
import shuffle from "../utils/shuffle";
import translate from "../locales";

export default function Session({ project, resetProject, language }) {
  const [tagsMap, setTagsMap] = useState({});
  const [, setReaderError] = useState(null);
  const [error, setError] = useState(null);
  const [sessionID, setSessionID] = useState(null);
  const [phase, setPhase] = useState(0);
  const [themes, setThemes] = useState(project.themes);
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
    const unlisten = listen("updated-tags", ({ payload }) => {
      console.log(payload)
      const counts = Object.values(payload).reduce(
        (acc, cur) => {
          acc[cur.antenna]++;
          return acc;
        },
        { 1: 0, 2: 0, 3: 0 },
      );
      console.log(counts)
      setTagsMap(counts);
    });

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
  }, [chosenTheme, phase]);

  useEffect(() => {
    if (phase === 0) {
      setChosenThemeKey(null);
      setThemes(shuffle(project.themes));
    }
  }, [phase]);

  useEffect(() => {
    if (chosenTheme) goToNextPhase();
  }, [chosenTheme]);

  function goToNextPhase() {
    if (chosenTheme === undefined && phase === 0) {
      throw new Error("A theme should be chosen when going to phase != 0");
    }
    setPhase(chosenTheme.questions[phase] === undefined ? 0 : phase + 1);
  }

  function handleKeyDown(event) {
    if (event.code === "ArrowRight") {
      if (phase === 0) {
        const newTheme = themes[0].key;
        setChosenThemeKey(newTheme);
      } else {
        goToNextPhase();
      }
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
          title={translate("choose_a_theme", language)}
          description={translate("stand_in_circle", language)}
          options={themes.slice(0, 3).map((a) => a.name[language])}
          tagsMap={tagsMap}
        />
      ) : (
        <InteractionScreen
          title={chosenTheme.questions[phase - 1].title[language]}
          description={translate("stand_in_circle", language)}
          options={chosenTheme.questions[phase - 1].options.map(
            (a) => a.value[language],
          )}
          themeName={chosenTheme.name[language]}
          phase={phase}
          tagsMap={tagsMap}
        />
      )}
    </>
  );
}
