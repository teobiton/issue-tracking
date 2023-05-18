use csv::Writer;
use exitfailure::ExitFailure;

use crate::filters::Filters;
use crate::parser::Issue;

/*
    From packed stuctures, write the serialized data into a csv file.
*/

const CSV_HEADER: [&str; 5] = ["ID", "Created at", "Last update", "Status", "Comment"];
pub const CSV_EXT: &str = ".csv";

pub fn build_output_file(filename: String) -> Result<String, Box<dyn std::error::Error>> {
    let extensions: [&str; 7] = [".txt", ".csv", ".text", ".dat", ".log", ".xls", ".xlsx"];

    if filename == "" {
        return Ok(String::from("out.csv"));
    }

    for ext in extensions {
        if filename.contains(ext) {
            return Ok(filename);
        }
    }

    Ok(String::from(filename.to_owned() + CSV_EXT))
}

pub fn build_csv(issues: Vec<Issue>, filename: &str, filters: Filters) -> Result<(), ExitFailure> {
    let mut wtr = Writer::from_path(filename)?;
    wtr.write_record(&CSV_HEADER)?;

    for issue in issues {
        if filters.is_filtered(&issue) {
            wtr.write_record(&[
                issue.number.to_string(),
                Issue::format_date(issue.created_at),
                Issue::format_date(issue.updated_at),
                issue.state,
                issue.title,
            ])?;
        }
    }

    wtr.flush()?;

    Ok(())
}
