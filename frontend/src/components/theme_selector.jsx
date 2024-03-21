import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";

export default function ThemeSelector({
  project,
  resetProject,
  language,
  setSessionID,
  step,
  STEPS,
}) {
  const [error, setError] = useState(null); // eslint-disable-line

  async function startNewSession(e) { // eslint-disable-line
    e.preventDefault();
    const data = new FormData(e.target);

    const themeKey = data.get("themeKey");

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

  return (
    <div className="themeselector">
      <div
        className={
          step === STEPS.themeSelector
            ? "themeselector-instruction"
            : STEPS.chooseThemeSplash && "themeselector-instruction--fullscreen"
        }
      >
        {project.translations.chooseTheme["en"]}
      </div>
      <div className="themeselector-content">
        <div className="themeselector-overlay">
          <div className="themeselector-overlay__middletoptoast">
            {project.translations.themeInstruction["en"]}
          </div>
          <div className="themeselector-overlay__righttoptoast">10</div>
          <div className="themeselector-overlay__rightbottomtoast">
            Logo / Info / Projectnumber
          </div>
        </div>
        <div className="themeselector-themes">
          <div className="themeselector-themes__theme">
            {project.themes[0].name[language]}
          </div>
          <div className="themeselector-themes__theme">
            {project.themes[1].name[language]}
          </div>
          <div className="themeselector-themes__theme">
            {project.themes[2].name[language]}
          </div>
        </div>
      </div>
    </div>
  );
}
