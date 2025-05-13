import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import translate, { translateError } from "../../locales";
import deleteIcon from "../../assets/visuals/delete.svg";

const STATES = {
    idle: "IDLE",
    working: "WORKING",
    error: "ERROR",
    done: "DONE",
};

export default function DeleteData({ setViewPopUp, selectedProjectKey }) {
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
              <img src={deleteIcon} alt="" />
              {translate("delete_button")}
            </button>
            <button
              type="button"
              className="start-screen__button start-screen__button--outline"
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
  