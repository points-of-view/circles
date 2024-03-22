export default function Question({ project, language, step, STEPS, tags }) {
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
            <div className="circle people-amount">
              {tags.filter((tag) => tag.option === 1).length}
            </div>
            <div className="circle question-content">
              <div>
                {project.themes[0].questions[0].options[0].value[language]}
              </div>
              <div className="people-figure">
                {tags
                  .filter((tag) => tag.option === 1)
                  .map((index) => (
                    <svg
                      key={index}
                      width="15"
                      height="66"
                      viewBox="0 0 15 66"
                      fill="none"
                      xmlns="http://www.w3.org/2000/svg"
                    >
                      <path
                        d="M13.2268 6.81944C13.2268 10.0801 10.583 12.7238 7.32237 12.7238C4.0613 12.7238 1.41797 10.0801 1.41797 6.81944C1.41797 3.55837 4.06139 0.915039 7.32237 0.915039C10.5831 0.915039 13.2268 3.55846 13.2268 6.81944Z"
                        fill="white"
                      />
                      <path
                        d="M10.5676 14.2434H4.08794C1.98237 14.2434 0.267578 15.9476 0.267578 18.0637V37.0036C0.267578 38.7944 1.50503 40.2922 3.16537 40.7046L4.29427 65.1035L10.3612 65.1039L11.2078 40.77C13.0096 40.4662 14.388 38.8923 14.388 37.0037V18.0529C14.3774 15.9474 12.6732 14.2432 10.5676 14.2432L10.5676 14.2434Z"
                        fill="white"
                      />
                    </svg>
                  ))}
              </div>
            </div>
          </div>
          <div className="circle circle__container">
            <div className="circle people-amount">
              {tags.filter((tag) => tag.option === 2).length}
            </div>
            <div className="circle question-content">
              <div>
                {project.themes[0].questions[0].options[0].value[language]}
              </div>
              <div className="people-figure">
                {tags
                  .filter((tag) => tag.option === 2)
                  .map((index) => (
                    <svg
                      key={index}
                      width="15"
                      height="66"
                      viewBox="0 0 15 66"
                      fill="none"
                      xmlns="http://www.w3.org/2000/svg"
                    >
                      <path
                        d="M13.2268 6.81944C13.2268 10.0801 10.583 12.7238 7.32237 12.7238C4.0613 12.7238 1.41797 10.0801 1.41797 6.81944C1.41797 3.55837 4.06139 0.915039 7.32237 0.915039C10.5831 0.915039 13.2268 3.55846 13.2268 6.81944Z"
                        fill="white"
                      />
                      <path
                        d="M10.5676 14.2434H4.08794C1.98237 14.2434 0.267578 15.9476 0.267578 18.0637V37.0036C0.267578 38.7944 1.50503 40.2922 3.16537 40.7046L4.29427 65.1035L10.3612 65.1039L11.2078 40.77C13.0096 40.4662 14.388 38.8923 14.388 37.0037V18.0529C14.3774 15.9474 12.6732 14.2432 10.5676 14.2432L10.5676 14.2434Z"
                        fill="white"
                      />
                    </svg>
                  ))}
              </div>
            </div>
          </div>
          <div className="circle circle__container">
            <div className="circle people-amount">
              {tags.filter((tag) => tag.option === 3).length}
            </div>
            <div className="circle question-content">
              <div>
                {project.themes[0].questions[0].options[0].value[language]}
              </div>
              <div className="people-figure">
                {tags
                  .filter((tag) => tag.option === 3)
                  .map((index) => (
                    <svg
                      key={index}
                      width="15"
                      height="66"
                      viewBox="0 0 15 66"
                      fill="none"
                      xmlns="http://www.w3.org/2000/svg"
                    >
                      <path
                        d="M13.2268 6.81944C13.2268 10.0801 10.583 12.7238 7.32237 12.7238C4.0613 12.7238 1.41797 10.0801 1.41797 6.81944C1.41797 3.55837 4.06139 0.915039 7.32237 0.915039C10.5831 0.915039 13.2268 3.55846 13.2268 6.81944Z"
                        fill="white"
                      />
                      <path
                        d="M10.5676 14.2434H4.08794C1.98237 14.2434 0.267578 15.9476 0.267578 18.0637V37.0036C0.267578 38.7944 1.50503 40.2922 3.16537 40.7046L4.29427 65.1035L10.3612 65.1039L11.2078 40.77C13.0096 40.4662 14.388 38.8923 14.388 37.0037V18.0529C14.3774 15.9474 12.6732 14.2432 10.5676 14.2432L10.5676 14.2434Z"
                        fill="white"
                      />
                    </svg>
                  ))}
              </div>
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
