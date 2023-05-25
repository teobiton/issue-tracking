use csv::Writer;
use exitfailure::ExitFailure;

use crate::filters::Filters;
use crate::parser::Issue;

/*
    From packed stuctures, write the serialized data into a csv file.
*/

const CSV_HEADER: [&str; 5] = ["ID", "Created at", "Last update", "Status", "Comment"];
const CSV_EXT: &str = ".csv";

pub fn build_output_filename(filename: String) -> Result<String, Box<dyn std::error::Error>> {
    let extensions: [&str; 7] = [".txt", ".csv", ".text", ".dat", ".log", ".xls", ".xlsx"];

    // If no filename was specified, always return 'out.csv'
    if filename == "out.csv" {
        return Ok(filename);
    }

    // If a known extension was specified, we return the filename as is
    for ext in extensions {
        if filename.contains(ext) {
            return Ok(filename);
        }
    }

    // Add the 'csv' extension if the file does not contain one
    Ok(String::from(filename.to_owned() + CSV_EXT))
}

pub fn write_csv(issues: Vec<Issue>, filename: &str, filters: Filters) -> Result<(), ExitFailure> {
    let mut wtr = Writer::from_path(filename)?;

    // The header is always the same
    wtr.write_record(&CSV_HEADER)?;

    // Parse the array of issues
    for issue in issues {
        // Only write the issues that are accepted by the filters
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
