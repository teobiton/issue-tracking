use crate::err::ErrKind;
use crate::err::IssueParserErr;
use serde_derive::{Deserialize, Serialize};
use std::path::Path;

/*
    Functions to process the json input.
    Structs defined to store data from the json.
    json data is analyzed and packed in structures.
*/

#[derive(Serialize, Deserialize, Debug)]
pub struct Repository {
    pub issues: Vec<Issue>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Issue {
    pub title: String,
    pub number: i32,
    pub labels: Vec<Label>,
    pub state: String,
    pub created_at: String,
    pub updated_at: String,
    pub closed_at: Option<String>,
    pub draft: Option<bool>,
}

impl Issue {
    pub fn format_date(date: String) -> String {
        // Slice the orifinal date to only keep YYYY-MM-DD
        date[..10].to_string()
    }

    pub fn is_labeled(&self, pattern: &str) -> bool {
        // Detect if an issue contains a specified label
        for label in &self.labels {
            if label.name == pattern {
                return true;
            }
        }

        false
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Label {
    pub name: String,
}

fn has_github_issues(text: &str) -> bool {
    let fields: [&str; 7] = [
        "\"issues\"",
        "\"title\"",
        "\"number\"",
        "\"labels\"",
        "\"state\"",
        "\"created_at\"",
        "\"closed_at\"",
    ];

    // Return false if one of the fields is not detected
    for field in fields {
        if !text.contains(field) {
            return false;
        }
    }

    true
}

pub fn parse_json_input(json_file: &Path) -> Result<Repository, IssueParserErr> {
    // Load the first file into a string
    let text = match std::fs::read_to_string(json_file) {
        Ok(text) => text,
        Err(error) => {
            let e = IssueParserErr {
                msg: error.to_string(),
                kind: ErrKind::Parser,
            };
            return Err(e);
        }
    };

    // Early read of the JSON read as a string to check if it contains GitHub issues
    if !has_github_issues(&text) {
        let e = IssueParserErr {
            msg: format!(
                "'{}' does not seem to contain GitHub issues.",
                &json_file.display()
            ),
            kind: ErrKind::Parser,
        };
        return Err(e);
    }

    // Parse the string into a static JSON structure
    match serde_json::from_str::<Repository>(&text) {
        Ok(repository) => Ok(repository),
        Err(e) => Err(IssueParserErr {
            msg: e.to_string(),
            kind: ErrKind::Parser,
        }),
    }
}

pub fn print_repo_labels(issues: Vec<Issue>) {
    let mut repository_labels = Vec::new();

    for issue in issues {
        for label in issue.labels {
            if !repository_labels.contains(&label.name) {
                repository_labels.push(label.name);
            }
        }
    }

    repository_labels.sort_by_key(|a| a.to_lowercase());

    for label in repository_labels {
        println!("  {}", label);
    }
}
