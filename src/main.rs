use exitfailure::ExitFailure;
use std::path::Path;
use std::process;
use structopt::StructOpt;

use issue_parser::parser::*;

mod writer;
use writer::build_csv;

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
        help = "Specify a file to store the csv"
    )]
    output: String,

    /// Positional argument
    #[structopt(help = "Required JSON file")]
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

fn main() -> Result<(), ExitFailure> {
    println!("Retrieving arguments ... ");

    let args = Args::from_args();

    let json_file = Path::new(&args.json);

    match is_json_file_ok(&json_file) {
        1 => {
            eprintln!("'{}' does not exist!", &args.json);
            process::exit(1);
        }
        2 => {
            eprintln!("'{}' is not a json file!", &args.json);
            process::exit(1);
        }
        _ => {}
    };

    let repository_issues: Repository = parse_json_input(&json_file);
  
    match build_csv(repository_issues.issues, "filename") {
        Ok(()) => {},
        Err(_) => {eprintln!("Could not build csv.")}
    }

    Ok(())
}
