use exitfailure::ExitFailure;
use std::path::Path;
use std::process;
use structopt::StructOpt;

mod structs;

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

fn has_github_issues(text: &str) -> bool {
    let fields: [&str; 7] = [
        "\"issues\"",
        "\"title\"",
        "\"number\"",
        "\"labels\"",
        "\"state\"",
        "\"created_at\"",
        "\"closed_at\"",
    ];

    for field in fields {
        if !text.contains(field) {
            println!("{}", field);
            return false;
        }
    }

    true
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

    let repository_issues = {
        // Load the first file into a string
        let text = std::fs::read_to_string(&json_file).unwrap();

        if !has_github_issues(&text) {
            eprintln!("'{}' does not seem to contain GitHub issues.", &args.json);
            process::exit(1);
        }
        // Parse the string into a static JSON structure
        serde_json::from_str::<structs::Repository>(&text).unwrap()
    };

    // Debug
    println!("JSON input: {}", json_file.display());
    println!("Output file: {}", &args.output);
    println!("Test issue name: {}", &repository_issues.issues[0].title);
    println!(
        "Test issue date: {}",
        &repository_issues.issues[0].created_at
    );
    Ok(())
}
