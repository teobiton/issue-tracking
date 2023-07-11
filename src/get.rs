use reqwest::header::ACCEPT;
use reqwest::header::CONTENT_TYPE;
use reqwest::header::USER_AGENT;

use crate::parser::Issue;
use crate::parser::Repository;

const PER_PAGE: usize = 100;

fn request_page_issues(repo: &str, page: u16) -> Result<Vec<Issue>, Box<dyn std::error::Error>> {
    // build url
    let url: String = format!(
        "https://api.github.com/repos/{repo}/issues?state=all&page={page}&per_page={per_page}",
        repo = repo,
        page = page,
        per_page = PER_PAGE
    );

    let client = reqwest::blocking::Client::new();
    let resp = client
        .get(&url)
        .header(USER_AGENT, "access_header")
        .header(CONTENT_TYPE, "application/vnd.github+json")
        .header(ACCEPT, "application/vnd.github+json")
        .send()?;

    match resp.status() {
        reqwest::StatusCode::OK => {
            // on success, parse our JSON to issues
            match resp.json::<Vec<Issue>>() {
                Ok(issues) => {
                    return Ok(issues);
                }
                Err(e) => return Err(format!("Unexpected format: {}", e).into()),
            };
        }
        reqwest::StatusCode::UNAUTHORIZED => {
            return Err(format!("Unauthorized token").into());
        }
        reqwest::StatusCode::NOT_FOUND => {
            return Err(format!("'{}': not found.", &url).into());
        }
        _ => {
            return Err(format!("'{}': Something unexpected happened.", &url).into());
        }
    }
}

fn remove_pull_requests(elems: &mut Vec<Issue>) {
    elems.retain(|pr| pr.draft == None);
}

pub fn request_json(repo: &str) -> Result<Repository, Box<dyn std::error::Error>> {
    let mut page: u16 = 1;
    let mut issues = vec![];

    let mut fetched_number: usize = PER_PAGE;

    while fetched_number == PER_PAGE {
        match request_page_issues(repo, page) {
            Ok(mut page_elems) => {
                fetched_number = page_elems.len();
                remove_pull_requests(&mut page_elems);
                issues.append(&mut page_elems);
            }
            Err(e) => return Err(e),
        }

        page += 1;
    }

    Ok(Repository { issues: issues })
}
