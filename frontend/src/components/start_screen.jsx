import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { listen } from "@tauri-apps/api/event";
import { save, open } from "@tauri-apps/api/dialog";
import translate, { translateError } from "../locales";
import eastIcon from "../assets/visuals/east.svg";
import deleteIcon from "../assets/visuals/delete.svg";
import importIcon from "../assets/visuals/add.svg";
import fullscreenIcon from "../assets/visuals/fullscreen.svg";

const previousHostname = localStorage.getItem("circles.last_hostname");

const STATES = {
  idle: "IDLE",
  working: "WORKING",
  error: "ERROR",
  done: "DONE",
};

export function StartScreen({
  setProjectKey,
  setDarkMode,
  toggleFullScreen,
  projects,
}) {
  const [viewPopUp, setViewPopUp] = useState(null);
  const [selectedProject, setSelectedProject] = useState(null);

  return (
    <div className="start-screen">
      {viewPopUp === "start" && (
        <StartProject
          setProjectKey={setProjectKey}
          setDarkMode={setDarkMode}
          setViewPopUp={setViewPopUp}
          selectedProject={selectedProject}
        />
      )}
      {viewPopUp === "import" && <ImportCard setViewPopUp={setViewPopUp} />}
      {viewPopUp === "export" && (
        <ExportCard
          setViewPopUp={setViewPopUp}
          selectedProject={selectedProject}
        />
      )}
      {viewPopUp === "delete" && (
        <DeleteData
          setViewPopUp={setViewPopUp}
          selectedProject={selectedProject}
        />
      )}
      <div className="start-screen__title">Circles</div>
      <button
        className="start-screen__button"
        onClick={() => setViewPopUp("import")}
      >
        <img src={importIcon} alt="import_icon" />
        {translate("import_project")}
      </button>
      <button className="start-screen__button" onClick={toggleFullScreen}>
        <img src={fullscreenIcon} alt="fullscreen_icon" />
        {translate("start_fullscreen_button")}
      </button>
      <ProjectView
        projects={projects}
        setViewPopUp={setViewPopUp}
        setSelectedProject={setSelectedProject}
      />
    </div>
  );
}

function ProjectView({ projects, setViewPopUp, setSelectedProject }) {
  return (
    <div className="start-screen__project-list">
      <div className="start-screen__project-title">
        {translate("start_project_key")}
      </div>
      <ul className="start-screen__project-list">
        {projects.map((i, e) => (
          <ProjectItem
            key={e}
            projectKey={i.key}
            setViewPopUp={setViewPopUp}
            setSelectedProject={setSelectedProject}
          />
        ))}
      </ul>
    </div>
  );
}

function ProjectItem({ projectKey, setViewPopUp, setSelectedProject }) {
  return (
    <li className="start-screen__project-item">
      <span className="project-item__title">{projectKey}</span>
      <div className="project-item__controls">
        <button
          className="start-screen__button start-screen__button--start"
          onClick={() => {
            setViewPopUp("start");
            setSelectedProject(projectKey);
          }}
        >
          {translate("start_project_button")}
        </button>
        <span className="project-item__spacer"></span>
        <button
          className="start-screen__button start-screen__button--link"
          onClick={() => {
            setViewPopUp("export");
            setSelectedProject(projectKey);
          }}
        >
          {translate("start_export_title")}
        </button>
        <button
          className="start-screen__button start-screen__button--link"
          onClick={() => {
            setViewPopUp("delete");
            setSelectedProject(projectKey);
          }}
        >
          {translate("start_delete_title")}
        </button>
      </div>
    </li>
  );
}

function StartProject({
  setProjectKey,
  selectedProject,
  setDarkMode,
  setViewPopUp,
}) {
  const [state, setState] = useState(STATES.idle);
  const [error, setError] = useState(null);
  const [connectionStatus, setConnectionStatus] = useState(null);

  useEffect(() => {
    const unlisten = listen("connection-status", ({ payload }) =>
      setConnectionStatus(payload),
    );

    return () => unlisten.then((fn) => fn());
  }, []);

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
      setConnectionStatus(null);
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
      <div className="start-screen__popup">
        <h1 className="start-screen__title--dialog">
          {translate("start_project_title") + selectedProject}
        </h1>
        <span className="start-screen__label">
          {translate("start_reader_hostname_subtitle")}
        </span>
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
            {translate("start_button")}
            <img src={eastIcon} alt="arrow_east" />
          </button>
          <button
            type="button"
            className="start-screen__button start-screen__button--light"
            onClick={() => setViewPopUp(null)}
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
      {connectionStatus && (
        <span className="start-screen__detail">{connectionStatus}</span>
      )}
    </form>
  );
}

function ImportCard({ setViewPopUp }) {
  const [state, setState] = useState(STATES.idle);
  const [error, setError] = useState(null);

  async function importProjectData() {
    setState(STATES.working);

    const filepath = await open({
      multiple: false,
      directory: false,
    });

    try {
      await invoke("import_project", { filepath });
      setState(STATES.done);
    } catch (error) {
      setError(error); // We should add different errors for: the user canceled the dialog, the file extension isn't compatible, the project couldn't be read
      setState(STATES.error); // We should add different errors for: the user canceled the dialog, the file extension isn't compatible, the project couldn't be read
    }
  }

  return (
    <div className="start-screen__card">
      <div className="start-screen__popup">
        <h2 className="start-screen__title--dialog">
          {translate("import_project")}
        </h2>
        <span className="start-screen__label">
          {translate("import_project_subtitle")}
        </span>
        {state === STATES.done && (
          <span className="start-screen__message start-screen__message--success">
            {translate("import_done")}
          </span>
        )}
        {state === STATES.working && (
          <span className="start-screen__message start-screen__message--spinner">
            {translate("import_in_progress")}
          </span>
        )}
        {state === STATES.error && (
          <span className="start-screen__message start-screen__message--error">
            {translateError(error)}
          </span>
        )}
        <div className="start-screen__button-container">
          <button
            className="start-screen__button"
            type="button"
            onClick={() => importProjectData()}
          >
            {translate("import_choose_file")}
          </button>
          <button
            type="button"
            className="start-screen__button start-screen__button--light"
            onClick={() => setViewPopUp(null)}
          >
            {translate("close_button")}
          </button>
        </div>
      </div>
    </div>
  );
}

function ExportCard({ setViewPopUp, selectedProject }) {
  const [state, setState] = useState(STATES.error);
  const [error, setError] = useState(null);

  async function exportData() {
    setState(STATES.working);

    const projectKey = selectedProject;

    const filepath = await save({
      filters: [
        {
          name: "export",
          extensions: ["xlsx"],
        },
      ],
    });
    if (filepath === null) {
      setError(translate("start_export_no_filepath_error"));
      return;
    }

    try {
      await invoke("save_export", { filepath, projectKey });
      setState(STATES.done);
    } catch (e) {
      setError(e);
      setState(STATES.error);
    }
  }

  return (
    <div className="start-screen__card">
      <div className="start-screen__popup">
        <h2 className="start-screen__title--dialog">
          {translate("start_export_title")}
        </h2>
        <span className="start-screen__label">
          {translate("export_project_data_subtitle")}
        </span>
        {state === STATES.working && (
          <span className="start-screen__message start-screen__message--spinner">
            {translate("start_export_working")}
          </span>
        )}
        {state === STATES.done && (
          <span className="start-screen__message start-screen__message--success">
            {translate("start_export_done")}
          </span>
        )}
        {state === STATES.error && (
          <span className="start-screen__message start-screen__message--error">
            {error}
          </span>
        )}
        <div className="start-screen__button-container">
          <button
            className="start-screen__button"
            type="button"
            onClick={() => exportData()}
          >
            {translate("start_export_button")}
          </button>
          <button
            type="button"
            className="start-screen__button start-screen__button--light"
            onClick={() => setViewPopUp(null)}
          >
            {translate("close_button")}
          </button>
        </div>
      </div>
    </div>
  );
}

function DeleteData({ setViewPopUp, selectedProject }) {
  const [state, setState] = useState(STATES.idle);
  const [error, setError] = useState(null);

  async function DeleteProjectData() {
    const projectKey = selectedProject;
    try {
      await invoke("delete_project_data", { projectKey }); // this is a placeholder function
      setState(STATES.done);
    } catch (e) {
      setError(e);
      setState(STATES.error);
    }
  }

  return (
    <div className="start-screen__card">
      <div className="start-screen__popup">
        <h2 className="start-screen__title--red">
          {translate("start_delete_title")}
        </h2>
        <span className="start-screen__label">
          {translate("delete_project_data_subtitle")}
        </span>
        {state === STATES.done && (
          <span className="start-screen__message start-screen__message--success">
            {translate("start_delete_done")}
          </span>
        )}
        {state === STATES.error && (
          <span className="start-screen__message start-screen__message--error">
            {translateError(error)}
          </span>
        )}
        <div className="start-screen__button-container">
          <button
            className="start-screen__button start-screen__button--red"
            type="button"
            onClick={() => DeleteProjectData()}
          >
            <img src={deleteIcon} alt="delete_icon" />
            {translate("delete_button")}
          </button>
          <button
            type="button"
            className="start-screen__button start-screen__button--light"
            onClick={() => setViewPopUp(null)}
          >
            {translate(
              state === STATES.done ? "close_button" : "cancel_button",
            )}
          </button>
        </div>
      </div>
    </div>
  );
}
