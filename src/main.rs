use std::path::Path;
use structopt::StructOpt;

use issue_parser::filters::Filters;
use issue_parser::input::check_inputs;
use issue_parser::input::Args;
use issue_parser::parser::parse_json_input;
use issue_parser::parser::Repository;
use issue_parser::writer::build_csv;
use issue_parser::writer::build_output_file;

/*
    Main thread of the application.
    Arguments are processed here and external functions are called to build the output.
*/

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Retrieve arguments
    let args = Args::from_args();

    // Convert the json file to a Path object
    let json_file = Path::new(&args.json);

    // Check if the inputs are correct, return error if not
    match check_inputs(&json_file, &args.output) {
        Err(error) => return Err(error),
        Ok(()) => {}
    };

    // Parse the JSON file and store its data into a Repository structure
    // Returns if an error occured
    let repository_issues: Repository = match parse_json_input(&json_file) {
        Ok(repository) => repository,
        Err(error) => return Err(error),
    };

    // Build the output file path into which we'll write data
    // Returns if an error occured
    let filename: String = match build_output_file(String::from(&args.output)) {
        Ok(file) => file,
        Err(error) => return Err(error),
    };

    // Write the csv output file from the repository structure, filename and potentially filters
    // Returns if an error occured
    match build_csv(
        repository_issues.issues,
        &filename,
        Filters::from_args(&args),
    ) {
        Ok(()) => {
            // Display created file path
            println!("Built {} from {}.", &filename, &args.json);
        }
        Err(e) => {
            return Err(format!("Could not build csv: {:#?}", e).into());
        }
    }

    // ... exit with no error
    Ok(())
}
