import { useEffect, useState, useMemo } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { InteractionScreen } from "./interaction-screen";
import ControlPanel from "./control-panel";
import { listen } from "@tauri-apps/api/event";
import shuffle from "../utils/shuffle";
import translate from "../locales";

export const STEPS = {
  showBigTitle: "showBigTitle",
  showBigQuestion: "showBigQuestion",
  showMainInteractionScreen: "showMainInteractionScreen",
  showBigOption: "showBigOption",
  showFact: "showFact",
};

const COLORS = ["green", "pink", "orange"];

function assignOptionColors(options) {
  return options.map((option, index) => ({
    value: option,
    color: COLORS[index],
  }));
}

export default function Session({ project, resetProject, language, darkMode }) {
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

  const currentQuestion =
    chosenTheme !== null && chosenTheme.questions[phase - 1];
  const description =
    step === STEPS.showMainInteractionScreen
      ? translate("stand_in_circle", language)
      : null;
  const themeName = phase !== 0 && chosenTheme.name[language];
  const accentColor = COLORS.toReversed()[phase];
  const showLogo = step === STEPS.showBigTitle;
  const iconID = step === STEPS.showBigTitle ? currentQuestion?.type : null;
  const showBackgroundElements = step !== STEPS.showMainInteractionScreen;
  const showBigTitle = [
    STEPS.showBigTitle,
    STEPS.showBigQuestion,
    STEPS.showFact,
  ].includes(step);
  const showFact = step === STEPS.showFact;
  const title = (() => {
    if (phase === 0 && step !== STEPS.showBigOption) {
      return [
        { text: translate("choose_a_theme_start", language) },
        { text: translate("choose_a_theme_accent", language), accent: true },
      ];
    } else if (step === STEPS.showBigTitle && currentQuestion.type === "quiz") {
      return [
        { text: translate("quiz_question_start", language) },
        { text: translate("quiz_question_accent", language), accent: true },
        { text: translate("quiz_question_end", language) },
      ];
    } else if (
      step === STEPS.showBigTitle &&
      currentQuestion.type === "opinion"
    ) {
      return [
        { text: translate("toughts_next_statement_start", language) },
        {
          text: translate("toughts_next_statement_accent", language),
          accent: true,
        },
        { text: translate("toughts_next_statement_end", language) },
      ];
    } else if (step === STEPS.showBigQuestion) {
      return `${translate(currentQuestion.type === "quiz" ? "question" : "statement", language)}: ${currentQuestion.title[language]}`;
    } else if (step === STEPS.showMainInteractionScreen) {
      return currentQuestion.title[language];
    } else if (step === STEPS.showBigOption && currentQuestion?.explanation) {
      return translate("correct_answer", language);
    } else if (step === STEPS.showFact) {
      return (
        translate("did_you_know", language) +
        "<br><br>" +
        currentQuestion.explanation[language]
      );
    }
  })();
  const options = (() => {
    if (phase === 0) {
      if (step === STEPS.showMainInteractionScreen) {
        return assignOptionColors(themes.map((a) => a.name[language]));
      } else if (step === STEPS.showBigOption) {
        return [
          {
            value: chosenTheme.name[language],
            color:
              COLORS[
                themes.findIndex((theme) => theme.key === chosenTheme.key)
              ],
          },
        ];
      }
    } else {
      if (step === STEPS.showMainInteractionScreen) {
        return assignOptionColors(
          currentQuestion.options.map((a) => a.value[language]),
        );
      } else if (step === STEPS.showBigOption) {
        const correctAnswerIndex = currentQuestion.options.findIndex(
          (a) => a.correct === true,
        );
        return [
          {
            value: currentQuestion.options[correctAnswerIndex]?.value[language],
            color: COLORS[correctAnswerIndex],
          },
        ];
      }
    }
  })();

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
        currentStep: currentQuestion.key,
        tagsMap,
      });
    } catch (error) {
      // The user cannot fix this error, so we just log it for debugging
      // eslint-disable-next-line no-console
      console.error("The answers couldn't be saved to the backend.", error);
    }
  }

  const tagCount = useMemo(
    () =>
      Object.values(tagsMap).reduce(
        (acc, cur) => {
          acc[cur.antenna]++;
          return acc;
        },
        { 1: 0, 2: 0, 3: 0 },
      ),
    [tagsMap],
  );

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
  }, [step, phase, tagsMap, tagCount, registeredAnswersInBackend]);

  useEffect(() => {
    if (phase === 0) {
      setThemes(
        shuffle(project.themes)
          .filter((t) => chosenTheme?.key !== t.key)
          .slice(0, 3),
      );
      setChosenTheme(null);
    }
  }, [phase]);

  function goToNextPhase() {
    if (chosenTheme === undefined && phase === 0) {
      throw new Error("A theme should be chosen when going to phase != 0");
    }
    setPhase(chosenTheme.questions[phase] === undefined ? 0 : phase + 1);
    setStep(STEPS.showBigTitle);
  }

  function chooseTheme() {
    const highestAmount = Math.max(...Object.values(tagCount));
    const popularAnswers = Object.keys(tagCount).filter(
      (key) => tagCount[key] === highestAmount,
    );
    const chooseRandomKey = Math.floor(Math.random() * popularAnswers.length);
    setChosenTheme(themes[popularAnswers[chooseRandomKey] - 1]);
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
          chooseTheme();
          setStep(STEPS.showBigOption);
        } else if (currentQuestion.type === "quiz") {
          saveAnswers();
          setStep(STEPS.showBigOption);
        } else if (currentQuestion.type === "opinion") {
          if (registeredAnswersInBackend === false) {
            saveAnswers();
            setRegisteredAnswersInBackend(true);
          } else if (registeredAnswersInBackend === true) {
            goToNextPhase();
            setRegisteredAnswersInBackend(false);
          }
        }
        break;
      case STEPS.showBigOption:
        if (phase === 0) {
          startNewSession(chosenTheme.key);
          goToNextPhase();
        } else {
          if (currentQuestion.explanation) {
            setStep(STEPS.showFact);
          }
        }
        break;
      case STEPS.showFact:
        goToNextPhase();
        break;
    }
  }

  function goToPreviousStep() {
    switch (step) {
      case STEPS.showBigTitle:
        if (phase > 0) {
          setPhase((currentPhase) => currentPhase - 1);
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
          options={phase === 0 ? themes : currentQuestion.options}
        />
      )}
      <InteractionScreen
        darkMode={darkMode}
        title={title}
        showBigTitle={showBigTitle}
        iconID={iconID}
        showBackgroundElements={showBackgroundElements}
        showFact={showFact}
        description={description}
        options={options}
        themeName={themeName}
        accentColor={accentColor}
        showLogo={showLogo}
        tagCount={tagCount}
      />
    </>
  );
}
