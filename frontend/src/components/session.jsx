import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { InteractionScreen } from "./interaction-screen";
import ControlPanel from "./control-panel";
import { listen } from "@tauri-apps/api/event";
import shuffle from "../utils/shuffle";
import translate from "../locales";

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
  const [tagsMap, setTagsMap] = useState({});
  const [, setReaderError] = useState(null);
  const [error, setError] = useState(null);
  const [sessionID, setSessionID] = useState(null);
  const [phase, setPhase] = useState(0);
  const [step, setStep] = useState(STEPS.showBigTitle);
  const [themes, setThemes] = useState(project.themes);
  const [chosenTheme, setChosenTheme] = useState(null);
  const [registeredAnswersInBackend, setRegisteredAnswersInBackend] =
    useState(false);

  const title = {
    ...((phase === 0 &&
      step !== STEPS.showBigOption && {
        value: translate("choose_a_theme", language),
      }) ||
      (step === STEPS.showBigTitle &&
        chosenTheme.questions[phase - 1].type === "quiz" && {
          value: translate("give_an_answer", language),
        }) ||
      (step === STEPS.showBigTitle &&
        chosenTheme.questions[phase - 1].type === "opinion" && {
          value: translate("whats_your_opinion", language),
        }) ||
      ((step === STEPS.showBigQuestion ||
        step === STEPS.showMainInteractionScreen) && {
        value: chosenTheme.questions[phase - 1].title[language],
      }) ||
      (phase !== 0 &&
        step === STEPS.showBigOption &&
        chosenTheme.questions[phase - 1].explanation && {
          value: translate("correct_answer", language),
        })),

    ...((step === STEPS.showBigTitle && { showBigTitle: true }) ||
      (step === STEPS.showBigQuestion && { showBigTitle: true })),
  };

  const description = {
    ...(step === STEPS.showMainInteractionScreen && {
      value: translate("stand_in_circle", language),
    }),
  };

  const options = {
    ...((step === STEPS.showMainInteractionScreen &&
      phase === 0 && {
        list: themes.slice(0, 3).map((a) => a.name[language]),
      }) ||
      (step === STEPS.showBigOption &&
        phase === 0 && { list: [chosenTheme.name[language]] }) ||
      (step === STEPS.showBigOption &&
        phase !== 0 && {
          list: [
            chosenTheme.questions[phase - 1].options.find(
              (el) => el.correct === true,
            )?.value[language],
          ],
        }) ||
      (step === STEPS.showMainInteractionScreen &&
        phase !== 0 && {
          list: chosenTheme.questions[phase - 1].options.map(
            (a) => a.value[language],
          ),
        }) ||
      (step === STEPS.showFact && {
        list: [
          translate("did_you_know", language) +
            "<br><br>" +
            chosenTheme.questions[phase - 1].explanation[language],
        ],
        showDescriptionLayout: true,
      })),
  };

  const themeName = {
    ...(phase !== 0 && { value: chosenTheme.name[language] }),
  };

  const logo = {
    ...(step === STEPS.showBigTitle && { show: true }),
  };

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

  async function saveAnswers() {
    try {
      await invoke("save_step_results", {
        currentStep: chosenTheme.questions[phase - 1].key,
        tagsMap,
      });
      setRegisteredAnswersInBackend(true);
    } catch (e) {
      throw new Error(
        "The answers couldn't be saved to the backend. Error:",
        e,
      );
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
  }, [step, phase]);

  useEffect(() => {
    if (phase === 0) {
      setChosenTheme(null);
      setThemes(shuffle(project.themes));
    }
  }, [phase]);

  function goToNextPhase() {
    if (chosenTheme === undefined && phase === 0) {
      throw new Error("A theme should be chosen when going to phase != 0");
    }
    setPhase(chosenTheme.questions[phase] === undefined ? 0 : phase + 1);
    setStep(STEPS.showBigTitle);
  }

  function goToNextStep() {
    switch (step) {
      case STEPS.showBigTitle:
        if (phase === 0) {
          setStep(STEPS.showMainInteractionScreen);
        } else {
          setStep(STEPS.showBigQuestion);
        }
        break;
      case STEPS.showBigQuestion:
        setStep(STEPS.showMainInteractionScreen);
        break;
      case STEPS.showMainInteractionScreen:
        if (phase === 0) {
          setChosenTheme(themes[0]);
          setStep(STEPS.showBigOption);
        } else if (chosenTheme.questions[phase - 1].type === "quiz") {
          setStep(STEPS.showBigOption);
        } else if (chosenTheme.questions[phase - 1].type === "opinion") {
          if (registeredAnswersInBackend === false) {
            saveAnswers();
          } else {
            goToNextPhase();
          }
        }
        break;
      case STEPS.showBigOption:
        if (phase === 0) {
          goToNextPhase();
        } else {
          if (chosenTheme.questions[phase - 1].explanation) {
            setStep(STEPS.showFact);
          }
        }
        break;
      case STEPS.showFact:
        goToNextPhase();
        break;
      case STEPS.showAnimationEnd:
        goToNextPhase();
        break;
    }
  }

  function goToPreviousStep() {
    switch (step) {
      case STEPS.showBigTitle:
        if (phase === 1) {
          setPhase(0);
          setStep(STEPS.showMainInteractionScreen);
        } else if (phase > 1) {
          setPhase(phase - 1);
          setStep(STEPS.showMainInteractionScreen);
        }
        break;
      case STEPS.showBigQuestion:
        setStep(STEPS.showBigTitle);
        break;
      case STEPS.showMainInteractionScreen:
        if (phase === 0) {
          setStep(STEPS.showBigTitle);
        } else if (registeredAnswersInBackend) {
          setRegisteredAnswersInBackend(false);
        } else {
          setStep(STEPS.showBigQuestion);
        }
        break;
      case STEPS.showBigOption:
        setStep(STEPS.showMainInteractionScreen);
        break;
      case STEPS.showFact:
        setStep(STEPS.showBigOption);
        break;
    }
  }

  function handleKeyDown(event) {
    if (event.code === "ArrowRight") goToNextStep();
    else if (event.code === "ArrowLeft") goToPreviousStep();
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
          goToNextPhase={goToNextPhase}
          options={
            (phase === 0 && themes.slice(0, 3)) ||
            chosenTheme.questions[phase - 1].options
          }
        />
      )}
      <InteractionScreen
        content={{
          title,
          description,
          options,
          themeName,
          logo,
        }}
      />
    </>
  );
}
