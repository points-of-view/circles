import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";

export default function ThemeSelector({
  project,
  resetProject,
  language,
  setSessionID,
}) {
  const [error, setError] = useState(null);

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
    <div>
      <div className="header">{project.translations.chooseTheme["en"]}</div>
      <div className="content">
        <div className="overlay">
          <div className="overlay__middletoptoast">
            {project.translations.themeInstruction["en"]}
          </div>
          <div className="overlay__righttoptoast">10</div>
          <div className="overlay__rightbottomtoast">
            Logo / Info / Projectnumber
          </div>
        </div>
        <div className="themes">
          {project.name[language]}
          <div className="themes__theme">{project.themes[0].name.en}</div>
          <div className="themes__theme">{project.themes[1].name.en}</div>
          <div className="themes__theme">{project.themes[2].name.en}</div>
        </div>
      </div>
    </div>
  );
}
