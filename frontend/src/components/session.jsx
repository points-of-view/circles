import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { InteractionScreen } from "./interaction-screen";
import ControlPanel from "./control-panel";
import { listen } from "@tauri-apps/api/event";
import shuffle from "../utils/shuffle";
import translate from "../locales";

const TITLE_DELAY = 5_000;

export const STEPS = {
  showAnimationStart: "showAnimationStart",
  showBigTitle: "showBigTitle",
  showBigQuestion: "showBigQuestion",
  showMainInteractionScreen: "showMainInteractionScreen",
  showBigOption: "showBigOption",
  showFact: "showFact",
  showAnimationEnd: "showAnimationEnd",
};

export default function Session({ project, resetProject, language }) {
  const [, setTagsMap] = useState({});
  const [, setReaderError] = useState(null);
  const [error, setError] = useState(null);
  const [sessionID, setSessionID] = useState(null);
  const [phase, setPhase] = useState(0);
  const [step, setStep] = useState(STEPS.showBigTitle);
  const [themes, setThemes] = useState(project.themes);
  const [chosenThemeKey, setChosenThemeKey] = useState(null);
  const [title, setTitle] = useState("");
  const [description, setDescription] = useState("");

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
  }, [chosenTheme, phase]);

  useEffect(() => {
    if (phase === 0) {
      setChosenThemeKey(null);
      setThemes(shuffle(project.themes));
    }
  }, [phase]);

  useEffect(() => {
    switch (step) {
      case STEPS.showBigTitle:
        if (phase === 0) {
          setTitle(translate("choose_a_theme", language));
          startTransitionTo(STEPS.showMainInteractionScreen);
        } else if (
          chosenTheme &&
          chosenTheme.questions[phase - 1].options.some(
            (el) => el.correct === true,
          )
        ) {
          setTitle(translate("give_an_answer", language));
          startTransitionTo(STEPS.showBigQuestion);
        } else {
          setTitle(translate("whats_your_opinion", language));
          startTransitionTo(STEPS.showBigQuestion);
        }
        break;
      case STEPS.showBigQuestion:
        setTitle(chosenTheme.questions[phase - 1].title[language]);
        startTransitionTo(STEPS.showMainInteractionScreen);
        break;
      case STEPS.showMainInteractionScreen:
        setDescription(translate("stand_in_circle", language));
        break;
      case STEPS.showBigOption:
        if (phase === 0) {
          startTransitionTo(STEPS.showAnimationEnd);
        } else {
          if (chosenTheme.questions[phase - 1].explanation) {
            setTitle(translate("correct_answer", language));
            startTransitionTo(STEPS.showFact);
          }
        }
        break;
      case STEPS.showFact:
        startTransitionTo(STEPS.showAnimationEnd);
        break;
      case STEPS.showAnimationEnd:
        goToNextPhase();
        setStep(STEPS.showBigTitle);
        break;
      default:
        throw new Error("Step has no correct value. Value is:", step);
    }
  }, [step]);

  function goToNextPhase() {
    if (chosenTheme === undefined && phase === 0) {
      throw new Error("A theme should be chosen when going to phase != 0");
    }
    setPhase(chosenTheme.questions[phase] === undefined ? 0 : phase + 1);
  }

  function startTransitionTo(stepID) {
    setTimeout(() => setStep(stepID), TITLE_DELAY);
  }

  function handleKeyDown(event) {
    if (event.code === "ArrowRight") {
      if (
        phase === 0 ||
        (chosenTheme !== undefined &&
          chosenTheme.questions[phase - 1].options.some(
            (el) => el.correct === true,
          ))
      ) {
        setChosenThemeKey(themes[0].key);
        setStep(STEPS.showBigOption);
      } else {
        setStep(STEPS.showAnimationEnd);
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
      <InteractionScreen
        title={title}
        description={description}
        themeOptions={themes.slice(0, 3).map((a) => a.name[language])}
        phase={phase}
        language={language}
        chosenTheme={chosenTheme}
        step={step}
      />
    </>
  );
}
