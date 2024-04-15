export default function ControlPanel({
  phase,
  project,
  language,
  startNewSession,
  error,
  sessionID,
  setChosenThemeKey,
  goToNextPhase,
  options,
}) {
  const handleOptionChange = (event) => {
    const selectedOption = event.target.value;
    if (phase === 0) {
      startNewSession(selectedOption);
      setChosenThemeKey(selectedOption);
      goToNextPhase();
    } else {
      goToNextPhase();
    }
  };

  return (
    <div className="control-panel">
      <p>Project name: {project.name[language]}</p>
      <select
        id="select-option"
        value={"default"}
        onChange={handleOptionChange}
      >
        <option value="default" disabled>
          {" "}
          -- select an option --{" "}
        </option>
        {Object.entries(options).map(([key, content]) => (
          <option key={key} value={content.key}>
            {content.name ? content.name[language] : content.value[language]}
          </option>
        ))}
      </select>
      {error && <span>{error}</span>}
      {sessionID && <div>Currently in session {sessionID}</div>}
    </div>
  );
}
