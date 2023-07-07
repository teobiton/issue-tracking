use std::path::Path;
use structopt::StructOpt;

/*
    Defines the inputs supported by the application, using StructOpt.
*/

#[derive(StructOpt)]
#[structopt(
    name = "issue-parser",
    about = "GitHub issues parser and exporter from JSON to csv"
)]
pub struct Args {
    /// --output=file
    #[structopt(
        long = "--output",
        short = "-o",
        default_value = "out.csv",
        help = "Specify a file to store the csv."
    )]
    pub output: String,

    /// --label=label
    #[structopt(
        long = "--label",
        short = "-l",
        default_value = "(all)",
        help = "Filter the issues based on a label."
    )]
    pub label: String,

    /// --print-labels
    #[structopt(long, help = "Print all available labels in the repository.")]
    pub print_labels: bool,

    /// --start-date=date
    #[structopt(
        long = "--start-date",
        default_value = "(oldest)",
        help = "Only consider issues updated after this date. Format: YYYY-MM-DD"
    )]
    pub start_date: String,

    /// --end-date=date
    #[structopt(
        long = "--end-date",
        default_value = "(newest)",
        help = "Only consider issues updated before this date. Format: YYYY-MM-DD"
    )]
    pub end_date: String,

    /// --state=state
    #[structopt(
        long = "--state",
        short = "-s",
        default_value = "(any)",
        help = "Only consider issues that have a particular state."
    )]
    pub state: String,

    /// Positional argument
    #[structopt(help = "Required JSON file.")]
    pub json: String,
}

pub fn check_inputs(
    filepath: &Path,
    filename: &str,
    dates: [&str; 2],
) -> Result<(), Box<dyn std::error::Error>> {
    // Check if the specified path exists
    if !filepath.exists() {
        return Err(format!("'{}' does not exist!", filepath.display()).into());
    }

    // Check if the specified path is a JSON file
    if filepath.extension().and_then(|ext| ext.to_str()) != Some("json") {
        return Err(format!("'{}' is not a json file!", filepath.display()).into());
    }

    // Check if the output filename contains rejectable characters
    for part in filename.split(".") {
        if !part.chars().all(char::is_alphanumeric) {
            return Err(format!("{}: filename contains special characters.", &filename).into());
        }
    }

    let mut is_default: bool;
    let mut date_num: Vec<&str>;

    for date in dates {
        is_default = date == "(oldest)" || date == "(newest)";

        if !is_default {
            date_num = date.split("-").collect();

            if date_num.len() != 3 {
                return Err(
                    format!("{}: date is not at the right format (YYYY-MM-DD).", &date).into(),
                );
            }

            for num in &date_num {
                if !num.chars().all(char::is_numeric) {
                    return Err(format!("{}: date contains non-numbers characters.", &date).into());
                }
            }

            if (&date_num[0].len() != &4)
                || (&date_num[1].len() != &2)
                || (&date_num[2].len() != &2)
            {
                return Err(
                    format!("{}: date is not at the right format (YYYY-MM-DD).", &date).into(),
                );
            }
        }
    }

    Ok(())
}
