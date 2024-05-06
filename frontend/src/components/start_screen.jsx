import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { save } from "@tauri-apps/api/dialog";
import projects from "../../../projects";
import translate from "../locales";

const previousHostname = localStorage.getItem("circles.last_hostname");

const STATES = {
  idle: "IDLE",
  working: "WORKING",
  error: "ERROR",
  done: "DONE",
};

export function StartScreen({ setProject }) {
  return (
    <div className="start-screen">
      <StartProject setProject={setProject} />
      <ExportCard />
    </div>
  );
}

function StartProject({ setProject }) {
  const [state, setState] = useState(STATES.idle);
  const [error, setError] = useState(null);

  async function handleSubmit(e) {
    e.preventDefault();
    setState(STATES.working);

    const data = new FormData(e.target);
    const projectKey = data.get("projectKey");
    const hostname = data.get("hostname");
    try {
      await invoke("select_project", { projectKey, hostname });
      setProject(projects[projectKey]);
      localStorage.setItem("circles.last_hostname", hostname);
    } catch (e) {
      setState(STATES.error);
      setError(e);
    }
  }

  return (
    <form
      action=""
      onSubmit={handleSubmit}
      className="start-screen__card"
      disabled={[STATES.working, STATES.done].includes(state)}
    >
      <h1 className="start-screen__title">
        {translate("start_project_title")}
      </h1>
      <div className="start-screen__input">
        <label className="start-screen__label" htmlFor="projectKey">
          {translate("start_project_key")}
        </label>
        <input
          className="start-screen__field"
          type="text"
          name="projectKey"
          id="projectKey"
          autoCapitalize="false"
          required
        />
      </div>
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
      <button
        type="submit"
        className="start-screen__button"
        disabled={[STATES.working, STATES.done].includes(state)}
      >
        {translate("start_project_button")}
      </button>
      {state === STATES.working && (
        <span className="start-screen__message start-screen__message--spinner">
          {translate("start_connecting")}
        </span>
      )}
      {state === STATES.error && (
        <span className="start-screen__message start-screen__message--error">
          {error}
        </span>
      )}
    </form>
  );
}

function ExportCard() {
  const [state, setState] = useState(STATES.error);
  const [error, setError] = useState(null);

  async function saveExport() {
    setState(STATES.working);
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
      await invoke("save_export", { filepath });
      setState(STATES.done);
    } catch (e) {
      setError(e);
      setState(STATES.error);
    }
  }

  return (
    <div className="start-screen__card">
      <h2 className="start-screen__title">{translate("start_export_title")}</h2>
      <button
        className="start-screen__button"
        onClick={saveExport}
        disabled={[STATES.working, STATES.done].includes(state)}
      >
        {translate("start_export_button")}
      </button>
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
    </div>
  );
}
