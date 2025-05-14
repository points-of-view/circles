import { useState } from "react";
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
  const [viewPopUp, setViewPopUp] = useState(null);
  const [selectedProjectKey, setSelectedProjectKey] = useState(null);

  return (
    <div className="start-screen">
      {viewPopUp === "start" && (
        <StartProject
          setProjectKey={setProjectKey}
          setDarkMode={setDarkMode}
          setViewPopUp={setViewPopUp}
          selectedProjectKey={selectedProjectKey}
        />
      )}
      {viewPopUp === "import" && <ImportCard setViewPopUp={setViewPopUp} />}
      {viewPopUp === "export" && (
        <ExportCard
          setViewPopUp={setViewPopUp}
          selectedProjectKey={selectedProjectKey}
        />
      )}
      {viewPopUp === "delete" && (
        <DeleteData
          setViewPopUp={setViewPopUp}
          selectedProjectKey={selectedProjectKey}
        />
      )}
      <div className="start-screen__title">Circles</div>
      {/* <button
        className="start-screen__button"
        onClick={() => setViewPopUp("import")}
      >
        <svg width="20" height="21" viewBox="0 0 20 21" xmlns="http://www.w3.org/2000/svg">
          <path d="M9.16667 18.3652V11.6986H2.5V10.0319H9.16667V3.36523H10.8333V10.0319H17.5V11.6986H10.8333V18.3652H9.16667Z" fill="currentColor" />
        </svg>
        {translate("import_project")}
      </button> */}
      <button className="start-screen__button" onClick={toggleFullScreen}>
        <svg
          width="16"
          height="16"
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
        setViewPopUp={setViewPopUp}
        setSelectedProjectKey={setSelectedProjectKey}
      />
    </div>
  );
}

function ProjectView({ projects, setViewPopUp, setSelectedProjectKey }) {
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
            setSelectedProjectKey={setSelectedProjectKey}
          />
        ))}
      </ul>
    </div>
  );
}

function ProjectItem({ projectKey, setViewPopUp, setSelectedProjectKey }) {
  return (
    <li className="start-screen__project-item">
      <span className="project-item__title">{projectKey}</span>
      <div className="project-item__controls">
        <button
          className="start-screen__button start-screen__button--start"
          onClick={() => {
            setViewPopUp("start");
            setSelectedProjectKey(projectKey);
          }}
        >
          {translate("start_project_button")}
        </button>
        <span className="project-item__spacer"></span>
        <button
          className="start-screen__button start-screen__button--link"
          onClick={() => {
            setViewPopUp("export");
            setSelectedProjectKey(projectKey);
          }}
        >
          {translate("start_export_title")}
        </button>
        {/* <button
          className="start-screen__button start-screen__button--link"
          onClick={() => {
            setViewPopUp("delete");
            setSelectedProjectKey(projectKey);
          }}
        >
          {translate("start_delete_title")}
        </button> */}
      </div>
    </li>
  );
}
