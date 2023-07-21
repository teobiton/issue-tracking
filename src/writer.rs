use csv::Writer;

use crate::err::ErrKind;
use crate::err::IssueParserErr;
use crate::filters::Filters;
use crate::parser::Issue;

/*
    From packed stuctures, write the serialized data into a csv file.
*/

const CSV_HEADER: [&str; 5] = ["ID", "Created at", "Last update", "State", "Comment"];
const CSV_EXT: &str = ".csv";

pub fn build_output_filename(filename: String) -> String {
    let extensions: [&str; 7] = [".txt", ".csv", ".text", ".dat", ".log", ".xls", ".xlsx"];

    // If no filename was specified, always return 'out.csv'
    if filename == "out.csv" {
        return filename;
    }

    // If a known extension was specified, we return the filename as is
    for ext in extensions {
        if filename.contains(ext) {
            return filename;
        }
    }

    // Add the 'csv' extension if the file does not contain one
    filename + CSV_EXT
}

pub fn write_csv(
    issues: Vec<Issue>,
    filename: &str,
    filters: Filters,
) -> Result<(), IssueParserErr> {
    let mut wtr = match Writer::from_path(filename) {
        Ok(writer) => writer,
        Err(error) => {
            return Err(IssueParserErr {
                msg: error.to_string(),
                kind: ErrKind::Writer,
            });
        }
    };

    // The header is always the same
    if let Err(error) = wtr.write_record(CSV_HEADER) {
        return Err(IssueParserErr {
            msg: error.to_string(),
            kind: ErrKind::Writer,
        });
    };

    // Parse the array of issues
    for issue in issues {
        // Only write the issues that are not rejected by the filters
        if !filters.reject(&issue) {
            if let Err(error) = wtr.write_record(&[
                issue.number.to_string(),
                Issue::format_date(issue.created_at),
                Issue::format_date(issue.updated_at),
                issue.state,
                issue.title,
            ]) {
                return Err(IssueParserErr {
                    msg: error.to_string(),
                    kind: ErrKind::Writer,
                });
            };
        }
    }

    if let Err(error) = wtr.flush() {
        return Err(IssueParserErr {
            msg: error.to_string(),
            kind: ErrKind::Writer,
        });
    };

    Ok(())
}
