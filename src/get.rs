use reqwest::header::ACCEPT;
use reqwest::header::CONTENT_TYPE;
use reqwest::header::USER_AGENT;

use crate::err::ErrKind;
use crate::err::IssueParserErr;
use crate::parser::Issue;
use crate::parser::Repository;

const PER_PAGE: usize = 100;

fn request_page_issues(repo: &str, page: u16) -> Result<Vec<Issue>, IssueParserErr> {
    // build url
    let url: String = format!(
        "https://api.github.com/repos/{repo}/issues?state=all&page={page}&per_page={per_page}",
        repo = repo,
        page = page,
        per_page = PER_PAGE
    );

    let client: reqwest::blocking::Client = reqwest::blocking::Client::new();
    let resp: reqwest::blocking::Response = match client
        .get(&url)
        .header(USER_AGENT, "access_header")
        .header(CONTENT_TYPE, "application/vnd.github+json")
        .header(ACCEPT, "application/vnd.github+json")
        .send()
    {
        Ok(resp) => resp,
        Err(error) => {
            let e = IssueParserErr {
                kind: ErrKind::Get,
                msg: error.to_string(),
            };
            return Err(e);
        }
    };

    match resp.status() {
        reqwest::StatusCode::OK => {
            // on success, parse our JSON to issues
            match resp.json::<Vec<Issue>>() {
                Ok(issues) => Ok(issues),
                Err(e) => {
                    let e = IssueParserErr {
                        msg: format!("Unexpected format: {}", e),
                        kind: ErrKind::Get,
                    };
                    Err(e)
                }
            }
        }
        reqwest::StatusCode::UNAUTHORIZED => Err(IssueParserErr {
            msg: "Unauthorized token".to_string(),
            kind: ErrKind::Get,
        }),
        reqwest::StatusCode::NOT_FOUND => Err(IssueParserErr {
            msg: format!("'{}': not found.", &url),
            kind: ErrKind::Get,
        }),
        _ => Err(IssueParserErr {
            msg: format!("'{}': Something unexpected happened.", &url),
            kind: ErrKind::Get,
        }),
    }
}

fn remove_pull_requests(elems: &mut Vec<Issue>) {
    elems.retain(|pr| pr.draft.is_none());
}

pub fn request_json(repo: &str) -> Result<Repository, IssueParserErr> {
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
            Err(error) => {
                let e = IssueParserErr {
                    kind: ErrKind::Get,
                    msg: error.to_string(),
                };
                return Err(e);
            }
        }

        page += 1;
    }

    Ok(Repository { issues })
}
