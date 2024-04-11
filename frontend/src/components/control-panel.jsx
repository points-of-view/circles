import { PHASES } from "../app";

export default function ControlPanel({
  phase,
  setPhase,
  project,
  startNewSession,
  error,
  language,
  sessionID,
  options,
  setChosenTheme,
}) {
  const handlePhaseChange = (event) => {
    const selectedPhase = event.target.value;
    setPhase(selectedPhase);
  };

  const handleOptionChange = (event) => {
    const selectedOption = event.target.value;
    if (phase === PHASES.pickTheme) {
      startNewSession(selectedOption);
      setChosenTheme(selectedOption);
      setPhase(PHASES.showQuestion);
    } else if (phase === PHASES.showQuestion) {
      setPhase(PHASES.showOpinionQuestion);
    } else if (phase === PHASES.showOpinionQuestion) {
      setChosenTheme(null);
      setPhase(PHASES.pickTheme);
    }
  };

  return (
    <div className="control-panel">
      <p>Project name: {project.name[language]}</p>
      <label htmlFor="select-phases">Choose a phase:</label>
      <select id="select-phases" value={phase} onChange={handlePhaseChange}>
        {Object.entries(PHASES).map(([key, value]) => (
          <option key={key} value={value}>
            {value}
          </option>
        ))}
      </select>
      <select
        id="select-option"
        value={"default"}
        onChange={handleOptionChange}
      >
        <option value="default" disabled>
          {" "}
          -- select an option --{" "}
        </option>
        {Object.entries(options).map(([key, value]) => (
          <option
            key={key}
            value={
              phase === PHASES.pickTheme ? project.themes[value].key : value
            }
          >
            {phase === PHASES.pickTheme
              ? project.themes[value].name[language]
              : value}
          </option>
        ))}
      </select>
      {error && <span>{error}</span>}
      {sessionID && <div>Currently in session {sessionID}</div>}
    </div>
  );
}
