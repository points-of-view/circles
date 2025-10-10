use include_dir::{include_dir, Dir, File};
use serde::{Deserialize, Serialize};

// NOTE: This path is relative to the Cargo root
const PROJECTS_DIR: Dir = include_dir!("../projects");

// NOTE: The structure for projects, themes, ... is not fully mapped here.
// We only map the properties that we need in our rust app
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub key: String,
    pub themes: Vec<Theme>,
    pub name: TranslatedProperty,
    pub available_languages: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Theme {
    pub key: String,
    pub name: TranslatedProperty,
    pub questions: Vec<Question>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Question {
    pub title: TranslatedProperty,
    pub explanation: Option<TranslatedProperty>,
    pub r#type: Option<String>,
    pub key: String,
    pub options: Option<Vec<QuestionOption>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct QuestionOption {
    pub key: String,
    pub value: TranslatedProperty,
    pub correct: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TranslatedProperty {
    pub nl: Option<String>,
    pub sl: Option<String>,
    pub po: Option<String>,
    pub en: Option<String>,
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

    pub fn find_by_key(project_key: &str) -> Option<Project> {
        Project::build_all()
            .iter()
            .find(|project| project.key == project_key)
            .cloned()
    }
}

impl Project {
    pub fn find_theme_by_key(&self, key: &str) -> Option<Theme> {
        let theme_key: String = key.into();
        self.themes
            .iter()
            .find(|theme| theme.key == theme_key)
            .cloned()
    }
}

impl Theme {
    pub fn find_question_by_key(&self, key: &str) -> Option<Question> {
        self.questions.iter().find(|q| q.key == key).cloned()
    }
}

impl Question {
    pub fn find_option_by_antenna_index(&self, index: usize) -> Option<QuestionOption> {
        self.options
            .clone()
            // Our antenna's use 1-based indexing
            .and_then(|opts| opts.get(index - 1).cloned())
    }
}

impl TranslatedProperty {
    pub fn get(&self, language: &str) -> Option<String> {
        match language {
            "nl" => self.nl.clone(),
            "sl" => self.sl.clone(),
            "po" => self.po.clone(),
            "en" => self.en.clone(),
            _ => None,
        }
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
        assert_eq!(theme.questions.len(), 5);

        let question = &theme.questions[0];
        assert_eq!(question.key, "question-one");
        assert!(question.options.is_some());
        assert_eq!(question.options.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn can_parse_all_projects() {
        // NOTE: We simply assert that no file will give an error when parsing
        Project::build_all();
    }

    #[test]
    fn should_be_able_to_find_project_by_key() {
        let project = Project::find_by_key("test");

        assert!(project.is_some());
        assert_eq!(project.unwrap().key, "test");
    }
}
