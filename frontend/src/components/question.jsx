import Shape from "./shape";

export default function Question({ project, language, step, STEPS, tags }) {
  if (step === STEPS.questionInstructionSplash) {
    return (
      <div className="questionscreen">
        <div className="overlay overlay--fullscreen">
          <div className="overlay__item">
            {project.themes[0].name[language]}
          </div>
        </div>
        <div className="questionscreen-content">
          <div className="circle circle__container--fullscreen">
            <div className="circle circle__instruction--fullscreen">
              <div>?</div>
              <div>
                {project.translations.questionMetaInstruction[language]}
              </div>
            </div>
          </div>
        </div>
      </div>
    );
  } else if (step === STEPS.questionContentSplash) {
    return (
      <div className="questionscreen">
        <div className="overlay overlay--fullscreen">
          <div className="overlay__item">
            {project.themes[0].name[language]}
          </div>
        </div>
        <div className="questionscreen-content">
          <div className="circle circle__container--fullscreen">
            <div className="circle circle__instruction--fullscreen">
              {project.themes[0].questions[0].title[language]}
            </div>
          </div>
        </div>
      </div>
    );
  } else if (step === STEPS.questionContentInteract) {
    return (
      <div className="questionscreen">
        <div className="overlay overlay--maximized">
          <div className="overlay__item overlay__item--question-stretched">
            {project.themes[0].questions[0].title[language]}
          </div>
          <div className="overlay__item overlay__item--theme">
            {project.themes[0].name[language]}
          </div>
          <div className="overlay__item overlay__item--instruction">
            {project.translations.questionStandInstruction[language]}
          </div>
          <div className="overlay__item overlay__item--timer">10</div>
          <div className="overlay__item--logo">POV Erasmus+</div>
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
                  .map((index, label) => (
                    <Shape
                      key={label}
                      shape={"figure"}
                      className="people-figure"
                    />
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
                  .map((index, label) => (
                    <Shape
                      key={label}
                      shape={"figure"}
                      className="people-figure"
                    />
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
                  .map((index, label) => (
                    <Shape
                      key={label}
                      shape={"figure"}
                      className="people-figure"
                    />
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
        <div className="overlay overlay--maximized">
          <div className="overlay__item overlay__item--correctanswer">
            {project.translations.questionCorrectAnswerTitle[language]}
          </div>
          <div className="overlay__item--logo">POV Erasmus+</div>
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
        <div className="overlay overlay--maximized">
          <div className="overlay__item overlay__item--correctanswer">
            {project.translations.questionCorrectAnswerTitle[language]}
          </div>
          <div className="overlay__item--logo">POV Erasmus+</div>
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
        <div className="overlay overlay--fullscreen">
          <div className="overlay__item">
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
