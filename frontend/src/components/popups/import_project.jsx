import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { open } from "@tauri-apps/api/dialog";
import translate, { translateError } from "../../locales";

const STATES = {
  idle: "IDLE",
  working: "WORKING",
  error: "ERROR",
  done: "DONE",
};

export default function ImportCard({ setViewPopUp }) {
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
            className="start-screen__button start-screen__button--outline"
            onClick={() => setViewPopUp(null)}
          >
            {translate("close_button")}
          </button>
        </div>
      </div>
    </div>
  );
}
