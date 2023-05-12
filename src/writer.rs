use csv::Writer;
use exitfailure::ExitFailure;
use std::fs;

use crate::parser::Issue;

/*
    From packed stuctures, write the serialized data into a csv file.
*/

const CSV_HEADER: [&str; 5] = ["ID", "Created at", "Last update", "Status", "Comment"];
pub const CSV_EXT: &str = ".csv";

pub fn build_csv(issues: Vec<Issue>, filename: &str) -> Result<(), ExitFailure> {
    fs::create_dir_all("out")?;

    let mut wtr = Writer::from_path("out/".to_owned() + filename + CSV_EXT)?;
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
