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

pub fn parse_json_input(json_file: &Path) -> Result<Repository, Box<dyn std::error::Error>> {
    // Load the first file into a string
    let text = std::fs::read_to_string(&json_file).unwrap();

    // Early read of the JSON read as a string to check if it contains GitHub issues
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

pub fn print_repo_labels(issues: Vec<Issue>) {
    let mut repository_labels = Vec::new();

    for issue in issues {
        for label in issue.labels {
            if !repository_labels.contains(&label.name) {
                repository_labels.push(label.name);
            }
        }
    }

    repository_labels.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));

    for label in repository_labels {
        println!("  {}", label);
    }
}
