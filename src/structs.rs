use serde_derive::{Deserialize, Serialize};

/*
    Structs defined to store data from the json
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
