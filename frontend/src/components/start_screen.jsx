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

export function StartScreen({ setProjectKey, setDarkMode, toggleFullScreen, projects }) {
  const [error, setError] = useState(null); //nog iets mee doen
  const [viewStartCard, setViewStartCard] = useState(false)
  const [selectedProject,setSelectedProject] = useState(null);

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
      {viewStartCard && <StartProject setProjectKey={setProjectKey} setDarkMode={setDarkMode} setViewStartCard={setViewStartCard} selectedProject={selectedProject} />}
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
      <ProjectView projects={projects} setViewStartCard={setViewStartCard} setSelectedProject={setSelectedProject} />
    </div>
  );
}

function ProjectView({ projects, setViewStartCard, setSelectedProject }) {

  return (
    <div className="start-screen__project-list">
      <div className="start-screen__project-title">{translate("start_project_key")}</div>
      <ul className="start-screen__project-list">
        {projects.map((i, e) => (
          <ProjectItem key={e} projectKey={i.key} setViewStartCard={setViewStartCard} setSelectedProject={setSelectedProject} />
        )
        )}
      </ul>
    </div>
  );
}

function ProjectItem({ projectKey, setViewStartCard, setSelectedProject }) {
  return (
    <li className="start-screen__project-item">
      <span className="project-item__title">{projectKey}</span>
      <div
        className="project-item__controls">
        <button
          onClick={() => {setViewStartCard(true); setSelectedProject(projectKey);}}
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
      </div>
    </li>
  );
}

function StartProject ({setProjectKey, selectedProject , setDarkMode, setViewStartCard }) {
  const [state, setState] = useState(STATES.idle);
  const [error, setError] = useState(null);

  async function handleSubmit(e) {
    e.preventDefault();
    setState(STATES.working);
    setError(null);

    const data = new FormData(e.target);
    const projectKey = selectedProject;
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
    <form
      action=""
      onSubmit={handleSubmit}
      className="start-screen__card"
      disabled={[STATES.working, STATES.done].includes(state)}
    >
      <div
      className="start-screen__popup"
      >
      <h1 className="start-screen__title--dialog">
        {translate("start_project_title")}
      </h1>
      <div className="start-screen__input">
        <label className="start-screen__label" htmlFor="hostname">
          {translate("start_reader_hostname")}
        </label>
        <input
          className="start-screen__field"
          type="text"
          name="hostname"
          id="hostname"
          autoCapitalize="false"
          placeholder="fx9600123456"
          minLength={12}
          maxLength={12}
          required
          defaultValue={previousHostname}
        />
      </div>
      <div className="start-screen__input start-screen__input--checkbox">
        <input
          type="checkbox"
          name="darkMode"
          id="darkMode"
          className="start-screen__checkbox"
          defaultChecked={true}
        />
        <label htmlFor="darkMode" className="start-screen__label">
          {translate("start_dark_mode")}
        </label>
      </div>
      <div className="start-screen__button-container">
      <button
        type="submit"
        className="start-screen__button"
        disabled={[STATES.working, STATES.done].includes(state)}
      >
        {translate("start_project_button")}
      </button>
      <button
        type="button"
        className="start-screen__button"
        onClick={() => setViewStartCard(false)}
      >
        {translate("cancel_button")}
      </button>
      </div>
      {state === STATES.working && (
        <span className="start-screen__message start-screen__message--spinner">
          {translate("start_connecting")}
        </span>
      )}
      {state === STATES.error && (
        <span className="start-screen__message start-screen__message--error">
          {translateError(error)}
        </span>
      )}
      </div>
    </form>
  );
}