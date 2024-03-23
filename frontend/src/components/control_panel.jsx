export default function ControlPanel({
  step,
  setStep,
  project,
  startNewSession,
  error,
  language,
  STEPS,
  sessionID,
}) {
  const handleStepChange = (event) => {
    const selectedStep = event.target.value;
    setStep(selectedStep);
  };

  return (
    <div id="control-panel">
      <p>Project name: {project.name[language]}</p>
      <label htmlFor="steps">Choose a step:</label>
      <select name="steps" value={step} onChange={handleStepChange}>
        {Object.entries(STEPS).map(([key, value]) => (
          <option key={key} value={value}>
            {value}
          </option>
        ))}
      </select>
      <form action="" onSubmit={startNewSession}>
        <input
          type="text"
          name="themeKey"
          id="themeKey"
          placeholder="themekey"
          required
        />
        <button type="submit">Start new session</button>
        {error && <span>{error}</span>}
      </form>

      {sessionID && <div>Currently in session {sessionID}</div>}
    </div>
  );
}
