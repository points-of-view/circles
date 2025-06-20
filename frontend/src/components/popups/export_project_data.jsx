import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { save } from "@tauri-apps/api/dialog";
import translate, { translateError } from "../../locales";

const STATES = {
  idle: "IDLE",
  working: "WORKING",
  error: "ERROR",
  done: "DONE",
};

export default function ExportCard({ exportDialog, selectedProjectKey }) {
  const [state, setState] = useState(STATES.idle);
  const [error, setError] = useState(null);

  async function exportData() {
    setState(STATES.working);

    const projectKey = selectedProjectKey;

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
    <dialog className="dialog" ref={exportDialog}>
      <div className="start-screen__popup">
        <h2 className="dialog__title">{translate("start_export_title")}</h2>
        <span className="dialog__label">
          {translate("export_project_data_subtitle")}
        </span>
        {state === STATES.working && (
          <span className="dialog__message dialog__message--spinner">
            {translate("start_export_working")}
          </span>
        )}
        {state === STATES.done && (
          <span className="dialog__message dialog__message--success">
            {translate("start_export_done")}
          </span>
        )}
        {state === STATES.error && (
          <span className="dialog__message dialog__message--error">
            {translateError(error)}
          </span>
        )}
        <div className="dialog__button-container">
          <button
            className="start-screen__button"
            type="button"
            onClick={() => exportData()}
          >
            {translate("start_export_button")}
          </button>
          <button
            type="button"
            className="start-screen__button start-screen__button--outline"
            onClick={() => {
              exportDialog.current?.close();
              setState(STATES.idle);
            }}
          >
            {translate("close_button")}
          </button>
        </div>
      </div>
    </dialog>
  );
}
