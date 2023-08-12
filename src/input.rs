use crate::err::ErrKind;
use crate::err::IssueParserErr;
use clap::Parser;
use std::path::Path;

/*
    Defines the inputs supported by the application, using clap.
*/

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about,
    long_about = "GitHub issues parser and exporter from JSON to csv"
)]
pub struct Args {
    /// --output=file
    #[arg(
        long,
        short,
        default_value = "out.csv",
        help = "Specify a file to store the csv."
    )]
    pub output: String,

    /// --label=label
    #[arg(
        long,
        short,
        default_value = "(all)",
        help = "Filter the issues based on a label."
    )]
    pub label: String,

    /// --print-labels
    #[arg(long, help = "Print all available labels in the repository.")]
    pub print_labels: bool,

    /// --start-date=date
    #[arg(
        long,
        default_value = "(oldest)",
        help = "Only consider issues updated after this date. Format: YYYY-MM-DD"
    )]
    pub start_date: String,

    /// --end-date=date
    #[arg(
        long,
        default_value = "(newest)",
        help = "Only consider issues updated before this date. Format: YYYY-MM-DD"
    )]
    pub end_date: String,

    /// --state=state
    #[arg(
        long,
        short,
        default_value = "(any)",
        help = "Only consider issues that have a particular state."
    )]
    pub state: String,

    /// -u, --url
    #[arg(long, short, help = "GET the JSON file from GitHub API.")]
    pub get: bool,

    /// Positional argument
    #[arg(help = "Required JSON file link (local or from GitHub API).")]
    pub json: String,
}

pub fn check_inputs(
    filepath_str: &str,
    filename: &str,
    dates: [&str; 2],
    get: &bool,
) -> Result<(), IssueParserErr> {
    if !get {
        // Convert the json file to a Path object
        let filepath = Path::new(filepath_str);

        // Check if the specified path exists
        if !filepath.exists() {
            let e = IssueParserErr {
                msg: format!("'{}' does not exist!", filepath.display()),
                kind: ErrKind::Input,
            };
            return Err(e);
        }

        // Check if the specified path is a JSON file
        if filepath.extension().and_then(|ext| ext.to_str()) != Some("json") {
            let e = IssueParserErr {
                msg: format!("'{}' is not a json file!", filepath.display()),
                kind: ErrKind::Input,
            };
            return Err(e);
        }
    } else if !filepath_str.contains('/') {
        let e = IssueParserErr {
            msg: format!("'{}' is not a valid GitHub repository.", &filepath_str),
            kind: ErrKind::Input,
        };
        return Err(e);
    }

    // Check if the output filename contains rejectable characters
    for part in filename.split('.') {
        if !part.chars().all(char::is_alphanumeric) {
            let e = IssueParserErr {
                msg: format!("{}: filename contains special characters.", &filename),
                kind: ErrKind::Input,
            };
            return Err(e);
        }
    }

    let mut is_default: bool;
    let mut date_num: Vec<&str>;

    for date in dates {
        is_default = date == "(oldest)" || date == "(newest)";

        if !is_default {
            date_num = date.split('-').collect();

            if date_num.len() != 3 {
                let e = IssueParserErr {
                    msg: format!("{}: date is not at the right format (YYYY-MM-DD).", &date),
                    kind: ErrKind::Input,
                };
                return Err(e);
            }

            for num in &date_num {
                if !num.chars().all(char::is_numeric) {
                    let e = IssueParserErr {
                        msg: format!("{}: date contains non-numbers characters.", &date),
                        kind: ErrKind::Input,
                    };
                    return Err(e);
                }
            }

            if (date_num[0].len() != 4) || (date_num[1].len() != 2) || (date_num[2].len() != 2) {
                let e = IssueParserErr {
                    msg: format!("{}: date is not at the right format (YYYY-MM-DD).", &date),
                    kind: ErrKind::Input,
                };
                return Err(e);
            }
        }
    }

    Ok(())
}
