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
    <div className="chooseTheme">
      <div
        className={
          step === STEPS.themeSelector
            ? "chooseTheme__instruction"
            : STEPS.chooseThemeSplash && "chooseTheme__instruction--fullscreen"
        }
      >
        {project.translations.chooseTheme["en"]}
      </div>
      <div className="chooseTheme__content">
        <div className="chooseTheme__overlay">
          <div className="chooseTheme__overlay--middletoptoast">
            {project.translations.themeInstruction["en"]}
          </div>
          <div className="chooseTheme__overlay--righttoptoast">10</div>
          <div className="chooseTheme__overlay--rightbottomtoast">
            Logo / Info / Projectnumber
          </div>
        </div>
        <div className="chooseTheme__themes">
          {project.name[language]}
          <div className="chooseTheme__themes__theme">
            {project.themes[0].name.en}
          </div>
          <div className="chooseTheme__themes__theme">
            {project.themes[1].name.en}
          </div>
          <div className="chooseTheme__themes__theme">
            {project.themes[2].name.en}
          </div>
        </div>
      </div>
    </div>
  );
}
