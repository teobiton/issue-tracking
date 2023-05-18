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
}

impl Issue {
    pub fn format_date(date: String) -> String {
        date[..10].to_string()
    }

    pub fn is_labeled(&self, pattern: &str) -> bool {
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

    for field in fields {
        if !text.contains(field) {
            return false;
        }
    }

    true
}

pub fn parse_json_input(json_file: &Path) -> Result<Repository, Box<dyn std::error::Error>> {
    // Load the first file into a string
    let text = std::fs::read_to_string(&json_file).unwrap();

    if !has_github_issues(&text) {
        return Err(format!(
            "'{}' does not seem to contain GitHub issues.",
            &json_file.display()
        )
        .into());
    }
    // Parse the string into a static JSON structure
    Ok(serde_json::from_str::<Repository>(&text).unwrap())
}
