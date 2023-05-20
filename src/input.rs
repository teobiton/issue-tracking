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
    
    /// --from-date=date
    #[structopt(
        long = "--from-date",
        default_value = "",
        help = "Only consider issues updated after this date. Format: YYYY-MM-DD"
    )]
    pub from_date: String,

    /// --until-date=date
    #[structopt(
        long = "--until-date",
        default_value = "",
        help = "Only consider issues updated before this date. Format: YYYY-MM-DD"
    )]
    pub until_date: String,

    /// --status=status
    #[structopt(
        long = "--status",
        short = "-s",
        default_value = "",
        help = "Only consider issues that have a particular status."
    )]
    pub status: String,

    /// Positional argument
    #[structopt(help = "Required JSON file.")]
    pub json: String,
}

pub fn check_inputs(filepath: &Path, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Check if the specified path exists
    if !filepath.exists() {
        return Err(format!("'{}' does not exist!", filepath.display()).into());
    }

    // Check if the specified path is a JSON file
    if filepath.extension().and_then(|ext| ext.to_str()) != Some("json") {
        return Err(format!("'{}' is not a json file!", filepath.display()).into());
    }

    // Check if the output filename contains acceptable characters
    for part in filename.split(".") {
        if !part.chars().all(char::is_alphanumeric) {
            return Err(format!("{}: filename contains special characters.", &filename).into());
        }
    }

    Ok(())
}
