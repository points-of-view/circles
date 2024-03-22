import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";

export default function Question({
  project,
  resetProject,
  language,
  setSessionID,
  step,
  STEPS,
}) {
  if (step === STEPS.questionInstructionSplash) {
    return (
      <div className="questionscreen">
        <div className="questionscreen-instruction--fullscreen">{project.translations.questionMetaInstruction[language]}</div>
        <div className="questionscreen-overlay questionscreen-overlay--fullscreen">
          <div className="toast questionscreen-overlay__lefttoptoast--fullscreen">
            {project.themes[0].name[language]}
          </div>
        </div>
      </div>
    );
  }
  else if (step === STEPS.questionContentSplash) {
    return (
      <div className="questionscreen">
        <div className="questionscreen-instruction--fullscreen">{project.themes[0].questions[0].title[language]}</div>
        <div className="questionscreen-overlay questionscreen-overlay--fullscreen">
          <div className="toast questionscreen-overlay__lefttoptoast--fullscreen">
            {project.themes[0].name[language]}
          </div>
        </div>
      </div>
    );
  } else if (step === STEPS.questionContentInteract) {
    return (
      <div className="questionscreen">
        <div className="questionscreen-overlay questionscreen-overlay--maximized">
          <div className="toast toast__stretchtop">
            {project.themes[0].questions[0].title[language]}
          </div>
          <div className="toast toast__middleleft">
            {project.themes[0].name[language]}
          </div>
          <div className="toast toast__middlecenter">
            {project.translations.questionStandInstruction[language]}
          </div>
          <div className="toast toast__timer">
            10
          </div>
          <div className="toast toast__logo">
            POV Erasmus+
          </div>
        </div>
        <div className="questionscreen-content">
          <div className="answer answer__container">
            <div className="answer people-amount">13</div>
            <div className="answer question-content">{project.themes[0].questions[0].options[0].value[language]}</div>
          </div>
          <div className="answer answer__container">
          <div className="answer people-amount">3</div>
            <div className="answer question-content">{project.themes[0].questions[0].options[1].value[language]}</div>
          </div>
          <div className="answer answer__container">
          <div className="answer people-amount">5</div>
            <div className="answer question-content">{project.themes[0].questions[0].options[2].value[language]}</div>
          </div>
        </div>
      </div>
    )
  }
}
