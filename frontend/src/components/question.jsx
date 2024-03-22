export default function Question({ project, language, step, STEPS }) {
  if (step === STEPS.questionInstructionSplash) {
    return (
      <div className="questionscreen">
        <div className="questionscreen-overlay questionscreen-overlay--fullscreen">
          <div className="toast questionscreen-overlay__lefttoptoast--fullscreen">
            {project.themes[0].name[language]}
          </div>
        </div>
        <div className="questionscreen-content">
          <div className="circle circle__container--fullscreen">
            <div className="circle circle__instruction--fullscreen">
              {project.translations.questionMetaInstruction[language]}
            </div>
          </div>
        </div>
      </div>
    );
  } else if (step === STEPS.questionContentSplash) {
    return (
      <div className="questionscreen-content">
        <div className="questionscreen-overlay questionscreen-overlay--fullscreen">
          <div className="toast questionscreen-overlay__lefttoptoast--fullscreen">
            {project.themes[0].name[language]}
          </div>
        </div>
        <div className="circle circle__container--fullscreen">
          <div className="circle circle__instruction--fullscreen">
            {project.themes[0].questions[0].title[language]}
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
          <div className="toast toast__timer">10</div>
          <div className="toast__logo">POV Erasmus+</div>
        </div>
        <div className="questionscreen-content">
          <div className="circle circle__container">
            <div className="circle people-amount">13</div>
            <div className="circle question-content">
              {project.themes[0].questions[0].options[0].value[language]}
            </div>
          </div>
          <div className="circle circle__container">
            <div className="circle people-amount">3</div>
            <div className="circle question-content">
              {project.themes[0].questions[0].options[1].value[language]}
            </div>
          </div>
          <div className="circle circle__container">
            <div className="circle people-amount">5</div>
            <div className="circle question-content">
              {project.themes[0].questions[0].options[2].value[language]}
            </div>
          </div>
        </div>
      </div>
    );
  } else if (step === STEPS.questionContentAnswerValue) {
    return (
      <div className="questionscreen">
        <div className="questionscreen-overlay questionscreen-overlay--maximized">
          <div className="toast toast__topcenter">
            {project.translations.questionCorrectAnswerTitle[language]}
          </div>
          <div className="toast__logo">POV Erasmus+</div>
        </div>
        <div className="questionscreen-content">
          <div className="circle circle__container--fullscreen">
            <div className="circle circle--fullscreen">
              {project.themes[0].questions[0].answer.value[language]}
            </div>
          </div>
        </div>
      </div>
    );
  } else if (step === STEPS.questionContentAnswerDescription) {
    return (
      <div className="questionscreen">
        <div className="questionscreen-overlay questionscreen-overlay--maximized">
          <div className="toast toast__topcenter">
            {project.translations.questionCorrectAnswerTitle[language]}
          </div>
          <div className="toast__logo">POV Erasmus+</div>
        </div>
        <div className="questionscreen-content">
          <div className="circle circle__container--fullscreen">
            <div className="circle circle--fullscreen">
              <p>
                {
                  project.translations.questionCorrectAnswerDescription[
                    language
                  ]
                }
              </p>
              <p>
                {project.themes[0].questions[0].answer.description[language]}
              </p>
            </div>
          </div>
        </div>
      </div>
    );
  } else if (step === STEPS.questionInstructionOpinion) {
    return (
      <div className="questionscreen">
        <div className="questionscreen-overlay questionscreen-overlay--fullscreen">
          <div className="toast questionscreen-overlay__lefttoptoast--fullscreen">
            {project.themes[0].name[language]}
          </div>
        </div>
        <div className="questionscreen-content">
          <div className="circle circle__container--fullscreen">
            <div className="circle circle__instruction--fullscreen">
              {project.translations.questionOpinion[language]}
            </div>
          </div>
        </div>
      </div>
    );
  }
}
