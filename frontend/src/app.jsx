import { useState } from "react";
import { SelectProject } from "./components/select_project";
import Session from "./components/session";

export const PHASES = {
  pickTheme: "pickTheme",
  showQuestion: "showQuestion",
  showOpinionQuestion: "showOpinionQuestion",
};

export default function App() {
  const [project, setProject] = useState(null);
  const language = project?.availableLanguages[0];

  return project ? (
    <Session
      project={project}
      language={language}
      resetProject={() => setProject(null)}
    />
  ) : (
    <SelectProject setProject={setProject} language={language} />
  );
}
