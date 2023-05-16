use std::path::Path;
use structopt::StructOpt;

use issue_parser::parser::parse_json_input;
use issue_parser::parser::Repository;
use issue_parser::writer::build_csv;
use issue_parser::writer::build_output_file;

/*
    Main thread of the application.
    Arguments are processed here and external functions are called to build the output.
*/

#[derive(StructOpt)]
#[structopt(
    name = "GitHub issues parser",
    about = "GitHub issues parser and exporter from JSON to csv"
)]
struct Args {
    /// --output=file
    #[structopt(
        long = "--output",
        short = "-o",
        default_value = "",
        help = "Specify a file to store the csv."
    )]
    output: String,

    /// --label=label
    #[structopt(
        long = "--label",
        short = "-l",
        default_value = "",
        help = "Filter the issues based on a label."
    )]
    label: String,

    /// --from-date=date
    #[structopt(
        long = "--from-date",
        default_value = "",
        help = "Only consider issues updated after this date. Format: YYYY-MM-DD"
    )]
    from_date: String,

    /// --until-date=date
    #[structopt(
        long = "--until-date",
        default_value = "",
        help = "Only consider issues updated before this date. Format: YYYY-MM-DD"
    )]
    until_date: String,

    /// --status=status
    #[structopt(
        long = "--status",
        short = "-s",
        default_value = "",
        help = "Only consider issues that have a particular status."
    )]
    status: String,

    /// Positional argument
    #[structopt(help = "Required JSON file.")]
    json: String,
}

fn is_json_file_ok(filepath: &Path) -> i8 {
    if !filepath.exists() {
        return 1;
    }

    if filepath.extension().and_then(|ext| ext.to_str()) != Some("json") {
        return 2;
    }

    0
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::from_args();

    let json_file = Path::new(&args.json);

    match is_json_file_ok(&json_file) {
        1 => {
            return Err(format!("'{}' does not exist!", &args.json).into());
        }
        2 => {
            return Err(format!("'{}' is not a json file!", &args.json).into());
        }
        _ => {}
    };

    let repository_issues: Repository = parse_json_input(&json_file);

    let filename: String = match build_output_file(String::from(&args.output)) {
        Ok(file) => file,
        Err(error) => return Err(error.into()),
    };

    match build_csv(repository_issues.issues, &filename) {
        Ok(()) => {
            println!("Built {} from {}.", &filename, &args.json);
        }
        Err(e) => {
            return Err(format!("Could not build csv: {:#?}", e).into());
        }
    }

    Ok(())
}
