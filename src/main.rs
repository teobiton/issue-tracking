use std::path::Path;
use structopt::StructOpt;

use issue_parser::input::Args;
use issue_parser::parser::parse_json_input;
use issue_parser::parser::Repository;
use issue_parser::writer::build_csv;
use issue_parser::writer::build_output_file;

/*
    Main thread of the application.
    Arguments are processed here and external functions are called to build the output.
*/

fn is_json_file_ok(filepath: &Path) -> Result<(), Box<dyn std::error::Error>> {
    if !filepath.exists() {
        return Err(format!("'{}' does not exist!", filepath.display()).into());
    }

    if filepath.extension().and_then(|ext| ext.to_str()) != Some("json") {
        return Err(format!("'{}' is not a json file!", filepath.display()).into());
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::from_args();

    let json_file = Path::new(&args.json);

    match is_json_file_ok(&json_file) {
        Err(error) => return Err(error),
        Ok(()) => {}
    };

    let repository_issues: Repository = match parse_json_input(&json_file) {
        Ok(repository) => repository,
        Err(error) => return Err(error),
    };

    let filename: String = match build_output_file(String::from(&args.output)) {
        Ok(file) => file,
        Err(error) => return Err(error),
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
