use include_dir::{include_dir, Dir, File};
use serde::Deserialize;

// NOTE: This path is relative to the Cargo root
const PROJECTS_DIR: Dir = include_dir!("../projects");

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub name: String,
    pub key: String,
    pub themes: Vec<Theme>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Theme {
    pub name: String,
    pub key: String,
    pub questions: Vec<Question>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Question {
    pub title: String,
    pub key: String,
    pub options: Vec<Option>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Option {
    pub key: String,
    pub value: String,
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

        assert_eq!(project.name, "Test project");
        assert_eq!(project.themes.len(), 1);

        let theme = &project.themes[0];
        assert_eq!(theme.name, "My first theme");
        assert_eq!(theme.questions.len(), 1);

        let question = &theme.questions[0];
        assert_eq!(question.title, "How is your day going?");
        assert_eq!(question.options.len(), 2);
    }

    #[test]
    fn can_parse_all_projects() {
        // NOTE: We simply assert that no file will give an error when parsing
        Project::build_all();
    }
}
