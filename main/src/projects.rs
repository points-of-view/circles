use include_dir::{include_dir, Dir, File};
use serde::Deserialize;

// NOTE: This path is relative to the Cargo root
const PROJECTS_DIR: Dir = include_dir!("../projects");

// NOTE: The structure for projects, themes, ... is not fully mapped here.
// We only map the properties that we need in our rust app
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub key: String,
    pub themes: Vec<Theme>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Theme {
    pub key: String,
    pub questions: Vec<Question>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Question {
    pub key: String,
    pub options: Option<Vec<OptionItem>>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OptionItem {
    pub key: String,
}

impl Project {
    pub fn build(file: &File) -> Project {
        serde_json::from_str(file.contents_utf8().unwrap()).expect("error while reading")
    }

    pub fn build_all() -> Vec<Project> {
        let mut result: Vec<Project> = vec![];
        let glob = "*.json";

        for entry in PROJECTS_DIR.find(glob).unwrap() {
            // NOTE: We only search one level deep, so every result should be a file
            result.push(Self::build(
                entry
                    .as_file()
                    .expect("Could not find file in projects folder"),
            ));
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse_project() {
        let file = PROJECTS_DIR
            .get_file("test.json")
            .expect("File `projects/test.json` does not exist!");
        let project = Project::build(&file);

        assert_eq!(project.key, "test");
        assert_eq!(project.themes.len(), 1);

        let theme = &project.themes[0];
        assert_eq!(theme.key, "theme-one");
        assert_eq!(theme.questions.len(), 1);

        let question = &theme.questions[0];
        assert_eq!(question.key, "question-one");
        assert_eq!(question.options.as_ref().unwrap().len(), 2);
    }

    #[test]
    fn can_parse_all_projects() {
        // NOTE: We simply assert that no file will give an error when parsing
        Project::build_all();
    }
}
