import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import translate, { translateError } from "../../locales";

const STATES = {
  idle: "IDLE",
  working: "WORKING",
  error: "ERROR",
  done: "DONE",
};

export default function DeleteData({ deleteDialog, selectedProjectKey }) {
  const [state, setState] = useState(STATES.idle);
  const [error, setError] = useState(null);

  async function DeleteProjectData() {
    const projectKey = selectedProjectKey;
    try {
      await invoke("delete_project_data", { projectKey }); // this is a placeholder function
      setState(STATES.done);
    } catch (e) {
      setError(e);
      setState(STATES.error);
    }
  }

  return (
    <dialog className="dialog" ref={deleteDialog}>
      <div className="start-screen__popup">
        <h2 className="dialog__title--red">
          {translate("start_delete_title")}
        </h2>
        <span className="dialog__label">
          {translate("delete_project_data_subtitle")}
        </span>
        {state === STATES.done && (
          <span className="dialog__message dialog__message--success">
            {translate("start_delete_done")}
          </span>
        )}
        {state === STATES.error && (
          <span className="dialog__message dialog__message--error">
            {translateError(error)}
          </span>
        )}
        <div className="dialog__button-container">
          <button
            className="start-screen__button start-screen__button--red"
            type="button"
            onClick={() => DeleteProjectData()}
          >
            <svg
              className="dialog__icon--delete"
              viewBox="0 0 20 21"
              xmlns="http://www.w3.org/2000/svg"
            >
              <path
                d="M5.8335 18.2803C5.37516 18.2803 4.9828 18.1171 4.65641 17.7907C4.33002 17.4643 4.16683 17.0719 4.16683 16.6136V5.78027H3.3335V4.11361H7.50016V3.28027H12.5002V4.11361H16.6668V5.78027H15.8335V16.6136C15.8335 17.0719 15.6703 17.4643 15.3439 17.7907C15.0175 18.1171 14.6252 18.2803 14.1668 18.2803H5.8335ZM14.1668 5.78027H5.8335V16.6136H14.1668V5.78027ZM7.50016 14.9469H9.16683V7.44694H7.50016V14.9469ZM10.8335 14.9469H12.5002V7.44694H10.8335V14.9469Z"
                fill="currentColor"
              />
            </svg>
            {translate("delete_button")}
          </button>
          <button
            type="button"
            className="start-screen__button start-screen__button--outline"
            onClick={() => deleteDialog.current?.close()}
          >
            {translate(
              state === STATES.done ? "close_button" : "cancel_button",
            )}
          </button>
        </div>
      </div>
    </dialog>
  );
}
