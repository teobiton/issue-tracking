use std::path::Path;
use std::process;
use structopt::StructOpt;
use exitfailure::ExitFailure;

/* 
    Main thread of the application.
    Arguments are processed here and external functions
    are called to build the output.
*/

#[derive(StructOpt)]
#[structopt(name = "GitHub issues parser",
            about = "GitHub issues parser and exporter from JSON to csv"
)]
struct Args {
    /// --output=file
    #[structopt(long = "--output",
                short = "-o", 
                default_value="",
                help = "Specify a file to store the csv")]
    output: String,

    /// Positional argument
    #[structopt(help = "Required JSON file")]
    json: String

}

fn main() -> Result<(), ExitFailure>{
    println!("Retrieving arguments ... ");
    
    let args = Args::from_args();

    let json_file = Path::new(&args.json);

    if !json_file.exists() { 
        eprintln!("'{}' does not exist!", &args.json);
        process::exit(1);
    }
    
    // Debug
    println!("JSON input: {}", json_file.display());
    println!("Output file: {}", &args.output);

    Ok(())
}
