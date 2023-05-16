use csv::Writer;
use exitfailure::ExitFailure;
use std::process;

use crate::parser::Issue;

/*
    From packed stuctures, write the serialized data into a csv file.
*/

const CSV_HEADER: [&str; 5] = ["ID", "Created at", "Last update", "Status", "Comment"];
pub const CSV_EXT: &str = ".csv";

fn is_output_ok(filename: &str) -> bool {
    for part in filename.split(".") {
        if !part.chars().all(char::is_alphanumeric) {
            return false;
        }
    }

    true
}

pub fn build_output_file(filename: String) -> Result<String, Box<dyn std::error::Error>> {
    let extensions: [&str; 7] = [".txt", ".csv", ".text", ".dat", ".log", ".xls", ".xlsx"];

    if filename == "" {
        return Ok(String::from("out.csv"));
    }

    if !is_output_ok(&filename) {
        return Err(format!("{}: filename contains special characters.", &filename).into());
    }

    for ext in extensions {
        if filename.contains(ext) {
            return Ok(filename);
        }
    }

    Ok(String::from(filename.to_owned() + CSV_EXT))
}

pub fn build_csv(issues: Vec<Issue>, filename: &str) -> Result<(), ExitFailure> {
    let mut wtr = Writer::from_path(filename)?;
    wtr.write_record(&CSV_HEADER)?;

    for issue in issues {
        wtr.write_record(&[
            issue.number.to_string(),
            issue.created_at,
            issue.updated_at,
            issue.state,
            issue.title,
        ])?;
    }

    wtr.flush()?;

    Ok(())
}

#[test]
fn test_correct_output_files() -> Result<(), Box<dyn std::error::Error>> {
    let filenames: [&str; 3] = ["out", "out.log", "out.csv"];

    for filename in filenames {
        assert_eq!(is_output_ok(filename), true);
    }

    Ok(())
}

#[test]
fn test_wrong_output_files() -> Result<(), Box<dyn std::error::Error>> {
    let filenames: [&str; 3] = ["out;", "#out", "out/out"];

    for filename in filenames {
        assert_eq!(is_output_ok(filename), false);
    }

    Ok(())
}
