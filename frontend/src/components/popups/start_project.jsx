import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { listen } from "@tauri-apps/api/event";
import translate, { translateError } from "../../locales";

const previousHostname = localStorage.getItem("circles.last_hostname");

const STATES = {
  idle: "IDLE",
  working: "WORKING",
  error: "ERROR",
  done: "DONE",
};

export default function StartProject({
  setProjectKey,
  selectedProjectKey,
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
    const projectKey = selectedProjectKey;
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
          {translate("start_project_title") + selectedProjectKey}
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
            <svg
              width="18"
              height="13"
              viewBox="0 0 18 13"
              xmlns="http://www.w3.org/2000/svg"
            >
              <path
                d="M11.4998 12.1139L10.3123 10.9473L14.1457 7.11393H0.666504V5.44727H14.1457L10.3332 1.61393L11.4998 0.447266L17.3332 6.2806L11.4998 12.1139Z"
                fill="currentColor"
              />
            </svg>
          </button>
          <button
            type="button"
            className="start-screen__button start-screen__button--outline"
            onClick={() => setViewPopUp(null)}
            disabled={state !== STATES.working}
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
