import { useState, useRef } from "react";
import translate from "../locales";
import DeleteData from "./popups/delete_project_data";
import ExportCard from "./popups/export_project_data";
import StartProject from "./popups/start_project";
import ImportCard from "./popups/import_project";

export function StartScreen({
  setProjectKey,
  setDarkMode,
  toggleFullScreen,
  projects,
}) {
  const [selectedProjectKey, setSelectedProjectKey] = useState(null);
  const startDialog = useRef();
  const exportDialog = useRef();
  const importDialog = useRef();
  const deleteDialog = useRef();

  return (
    <div className="start-screen">
      <div className="start-screen__title">
        {translate("installation_name")}
      </div>
      <button
        className="start-screen__button"
        onClick={() => importDialog.current?.showModal()}
        style={{ visibility: "hidden" }}
      >
        <svg
          className="dialog__icon--import"
          viewBox="0 0 20 21"
          xmlns="http://www.w3.org/2000/svg"
        >
          <path
            d="M9.16667 18.3652V11.6986H2.5V10.0319H9.16667V3.36523H10.8333V10.0319H17.5V11.6986H10.8333V18.3652H9.16667Z"
            fill="currentColor"
          />
        </svg>
        {translate("import_project")}
      </button>
      <button className="start-screen__button" onClick={toggleFullScreen}>
        <svg
          className="dialog__icon--fullscreen"
          viewBox="0 0 16 16"
          xmlns="http://www.w3.org/2000/svg"
        >
          <path
            d="M0.5 15.3652V8.69857H2.16667V12.5319L12.6667 2.0319H8.83333V0.365234H15.5V7.0319H13.8333V3.19857L3.33333 13.6986H7.16667V15.3652H0.5Z"
            fill="currentColor"
          />
        </svg>
        {translate("start_fullscreen_button")}
      </button>
      <ProjectView
        projects={projects}
        setSelectedProjectKey={setSelectedProjectKey}
        startDialog={startDialog}
        exportDialog={exportDialog}
        deleteDialog={deleteDialog}
      />
      <StartProject
        setProjectKey={setProjectKey}
        setDarkMode={setDarkMode}
        selectedProjectKey={selectedProjectKey}
        startDialog={startDialog}
      />
      <ImportCard importDialog={importDialog} />
      <ExportCard
        exportDialog={exportDialog}
        selectedProjectKey={selectedProjectKey}
      />
      <DeleteData
        deleteDialog={deleteDialog}
        selectedProjectKey={selectedProjectKey}
      />
    </div>
  );
}

function ProjectView({
  projects,
  setSelectedProjectKey,
  startDialog,
  exportDialog,
  deleteDialog,
}) {
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
            setSelectedProjectKey={setSelectedProjectKey}
            startDialog={startDialog}
            exportDialog={exportDialog}
            deleteDialog={deleteDialog}
          />
        ))}
      </ul>
    </div>
  );
}

function ProjectItem({
  projectKey,
  setSelectedProjectKey,
  startDialog,
  exportDialog,
  deleteDialog,
}) {
  return (
    <li className="start-screen__project-item">
      <span className="project-item__title">{projectKey}</span>
      <div className="project-item__controls">
        <button
          className="start-screen__button start-screen__button--start"
          onClick={() => {
            startDialog.current?.showModal();
            setSelectedProjectKey(projectKey);
          }}
        >
          {translate("start_project_button")}
        </button>
        <span className="project-item__spacer"></span>
        <button
          className="start-screen__button start-screen__button--link"
          onClick={() => {
            exportDialog.current?.showModal();
            setSelectedProjectKey(projectKey);
          }}
        >
          {translate("start_export_title")}
        </button>
        <button
          className="start-screen__button start-screen__button--link"
          onClick={() => {
            deleteDialog.current?.showModal();
            setSelectedProjectKey(projectKey);
          }}
          style={{ visibility: "hidden" }}
        >
          {translate("start_delete_title")}
        </button>
      </div>
    </li>
  );
}
