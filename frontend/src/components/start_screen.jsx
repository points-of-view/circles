import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { save, open } from "@tauri-apps/api/dialog";
import translate, { translateError } from "../locales";

const previousHostname = localStorage.getItem("circles.last_hostname");

const STATES = {
  idle: "IDLE",
  working: "WORKING",
  error: "ERROR",
  done: "DONE",
};

export function StartScreen({ setDarkMode, toggleFullScreen, projects }) {
  const [error, setError] = useState(null);

  async function importProject() {
    const filepath = await open({
      multiple: false,
      directory: false,
    });

    try {
      await invoke("import_project", { filepath });
    } catch (error) {
      setError(error);
    }
  }

  return (
    <div className="start-screen">
      <div className="start-screen__title">Circles</div>
      <button
        className="start-screen__button"
        onClick={importProject}
      >
        {translate("import_project")}
      </button>
      <button
        className="start-screen__button"
        onClick={toggleFullScreen}
      >
        {translate("start_fullscreen_button")}
      </button>
      <ProjectView projects={projects} setDarkMode={setDarkMode} />
    </div>
  );
}

function ProjectItem({ projectKey }) {
  const [state, setState] = useState(STATES.idle);
  const [error, setError] = useState(null);

  async function handleSubmit(e) {
    e.preventDefault();
    setState(STATES.working);
    setError(null);

    const data = new FormData(e.target);
    const projectKey = data.get("projectKey");
    const hostname = data.get("hostname");
    const darkMode = data.get("darkMode");
    try {
      await invoke("select_project", { projectKey, hostname });
      localStorage.setItem("circles.last_hostname", hostname);
      setDarkMode(darkMode);
      setProjectKey(projectKey);
    } catch (error) {
      setState(STATES.error);
      setError(error);
      // If an unknown error occurs, we want to log the details so we can see what went wrong
      // eslint-disable-next-line no-console
      if (error.kind === "Unknown") console.error(error);
    }
  }

  return (
    <li className="start-screen__project-item">
      <span className="project-item__title">{projectKey}</span>
      <form className="project-item__controls">
        <button
          type="submit"
          className="start-screen__button start-screen__button--start"
        >
          {translate("start_project_button")}
        </button>
        <span className="project-item__spacer"></span>
        <button
          type="submit"
          className="start-screen__button start-screen__button--link"
        >
          {translate("start_export_title")}
        </button>
        <button
          type="submit"
          className="start-screen__button start-screen__button--link"
        >
          {translate("start_delete_title")}
        </button>
      </form>
    </li>
  );
}

function ProjectView({ projects, setDarkMode }) {

  return (
    <div className="start-screen__card">
      <div className="start-screen__project-title">{translate("start_project_key")}</div>
      <ul className="start-screen__project-list">
        {projects.map((i, e) => (
          <ProjectItem key={e} projectKey={i.key} />
        )
        )}
      </ul>
    </div>
  );
}

// function ExportCard() {
//   const [state, setState] = useState(STATES.error);
//   const [error, setError] = useState(null);

//   async function handleSubmit(e) {
//     e.preventDefault();
//     setState(STATES.working);

//     const data = new FormData(e.target);
//     const projectKey = data.get("projectKey");

//     const filepath = await save({
//       filters: [
//         {
//           name: "export",
//           extensions: ["xlsx"],
//         },
//       ],
//     });
//     if (filepath === null) {
//       setError(translate("start_export_no_filepath_error"));
//       return;
//     }

//     try {
//       await invoke("save_export", { filepath, projectKey });
//       setState(STATES.done);
//     } catch (e) {
//       setError(e);
//       setState(STATES.error);
//     }
//   }

//   return (
//     <form
//       className="start-screen__card"
//       onSubmit={handleSubmit}
//       disabled={[STATES.working, STATES.done].includes(state)}
//     >
//       <h2 className="start-screen__title">{translate("start_export_title")}</h2>
//       <div className="start-screen__input">
//         <label className="start-screen__label" htmlFor="projectKey">
//           {translate("start_project_key")}
//         </label>
//         <input
//           className="start-screen__field"
//           type="text"
//           name="projectKey"
//           id="projectKey"
//           autoCapitalize="false"
//           required
//         />
//       </div>
//       <button className="start-screen__button" type="submit">
//         {translate("start_export_button")}
//       </button>
//       {state === STATES.working && (
//         <span className="start-screen__message start-screen__message--spinner">
//           {translate("start_export_working")}
//         </span>
//       )}
//       {state === STATES.done && (
//         <span className="start-screen__message start-screen__message--success">
//           {translate("start_export_done")}
//         </span>
//       )}
//       {state === STATES.error && (
//         <span className="start-screen__message start-screen__message--error">
//           {error}
//         </span>
//       )}
//     </form>
//   );
// }
