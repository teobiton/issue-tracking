use std::path::Path;
use structopt::StructOpt;

use issue_tracking::filters::Filters;
use issue_tracking::get::request_json;
use issue_tracking::input::check_inputs;
use issue_tracking::input::Args;
use issue_tracking::parser::parse_json_input;
use issue_tracking::parser::print_repo_labels;
use issue_tracking::parser::Repository;
use issue_tracking::writer::build_output_filename;
use issue_tracking::writer::write_csv;
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
    check_inputs(
        &args.json,
        &args.output,
        [&args.start_date, &args.end_date],
        &args.get,
    )?;

    // Parse the JSON file and store its data into a Repository structure
    // Returns if an error occured
    let repository_issues: Repository = if !args.get {
        match parse_json_input(json_file) {
            Ok(repository) => repository,
            Err(error) => return Err(error),
        }
    } else {
        match request_json(&args.json) {
            Ok(repository) => repository,
            Err(error) => return Err(error),
        }
    };

    // Parse the issues and display used labels
    // Returns after execution
    if args.print_labels {
        println!("Available labels from {}:", &args.json);
        print_repo_labels(repository_issues.issues);
        return Ok(());
    };

    // Build the output file path into which we'll write data
    // Returns if an error occured
    let filename: String = match build_output_filename(String::from(&args.output)) {
        Ok(file) => file,
        Err(error) => return Err(error),
    };

    // Write the csv output file from the repository structure, filename and potentially filters
    // Returns if an error occured
    match write_csv(
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
