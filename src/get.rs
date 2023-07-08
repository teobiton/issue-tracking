use reqwest::header::ACCEPT;
use reqwest::header::CONTENT_TYPE;
use reqwest::header::USER_AGENT;

use crate::parser::Issue;
use crate::parser::Repository;

pub fn request_json(url: &str) -> Result<Repository, Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();
    let resp = client
        .get(url)
        .header(USER_AGENT, "access_header")
        .header(CONTENT_TYPE, "application/vnd.github+json")
        .header(ACCEPT, "application/vnd.github+json")
        .send()?;

    match resp.status() {
        reqwest::StatusCode::OK => {
            // on success, parse our JSON to issues
            match resp.json::<Vec<Issue>>() {
                Ok(issues) => {
                    let repository = Repository { issues: issues };
                    return Ok(repository);
                }
                Err(e) => return Err(format!("Unexpected format: {}", e).into()),
            };
        }
        reqwest::StatusCode::UNAUTHORIZED => {
            return Err(format!("Unauthorized token").into());
        }
        reqwest::StatusCode::NOT_FOUND => {
            return Err(format!("'{}': not found.", url).into());
        }
        _ => {
            return Err(format!("'{}': Something unexpected happened.", url).into());
        }
    }
}
