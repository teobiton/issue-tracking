use exitfailure::ExitFailure;
use std::fs;
use std::io::Write;

use issue_parser::parser::Issue;

/*
    From packed stuctures, write the serialized data into a csv file.
*/

const CSV_HEADER: &[u8] = b"\"ID\", \"Created at\", \"Last update\", \"Status\", \"Comment\"\n";
const CSV_EXT: &str = ".csv";

pub fn build_csv(issues: Vec<Issue>, filename: &str) -> Result<(), ExitFailure> {
    fs::create_dir("out")?;
    let mut file = fs::File::create("out/".to_owned() + filename + CSV_EXT)?;
    file.write_all(CSV_HEADER)?;

    Ok(())
}
