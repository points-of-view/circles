import { useState } from "react";
import translate from "../locales";
import DeleteData from "./popups/delete_project_data";
import ExportCard from "./popups/export_project_data";
import StartProject from "./popups/start_project";
import ImportCard from "./popups/import_project";
import importIcon from "../assets/visuals/add.svg";
import fullscreenIcon from "../assets/visuals/fullscreen.svg";

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
      <button
        className="start-screen__button"
        onClick={() => setViewPopUp("import")}
      >
        <img src={importIcon} alt="" />
        {translate("import_project")}
      </button>
      <button className="start-screen__button" onClick={toggleFullScreen}>
        <img src={fullscreenIcon} alt="" />
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
        <button
          className="start-screen__button start-screen__button--link"
          onClick={() => {
            setViewPopUp("delete");
            setSelectedProjectKey(projectKey);
          }}
        >
          {translate("start_delete_title")}
        </button>
      </div>
    </li>
  );
}
