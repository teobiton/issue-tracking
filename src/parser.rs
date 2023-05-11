use std::path::Path;
use std::process;
use serde_derive::{Deserialize, Serialize};

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

#[derive(Serialize, Deserialize, Debug)]
pub struct Label {
    name: String,
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
            println!("{}", field);
            return false;
        }
    }

    true
}

pub fn parse_json_input(json_file: &Path) -> Repository {
    // Load the first file into a string
    let text = std::fs::read_to_string(&json_file).unwrap();

    if !has_github_issues(&text) {
        eprintln!("'{}' does not seem to contain GitHub issues.", &json_file.display());
        process::exit(1);
    }
    // Parse the string into a static JSON structure
    serde_json::from_str::<Repository>(&text).unwrap()
}

#[test]
fn parse_correct_json() {
    
    let json_file = Path::new("tests/doc/cocotb-cocotb_issues.json");

    let repository = parse_json_input(&json_file);

    assert_eq!(repository.issues.len(), 1545);
    assert_eq!(repository.issues[0].title, "SIM_ROOT requires setting prior to make");
    assert_eq!(repository.issues[53].state, "closed");
}