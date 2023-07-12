use std::path::Path;

use issue_tracking::parser::parse_json_input;
use issue_tracking::parser::Repository;

/*
   Tests for the parse_correct_json function.
*/

#[test]
fn test_parse_correct_json() -> Result<(), Box<dyn std::error::Error>> {
    let json_file = Path::new("tests/doc/cocotb-cocotb_issues.json");

    let repository: Repository = match parse_json_input(json_file) {
        Ok(repository) => repository,
        Err(_) => {
            panic!("Parser did not recognize a legititmate json.")
        }
    };

    assert_eq!(repository.issues.len(), 1545);
    assert_eq!(
        repository.issues[0].title,
        "SIM_ROOT requires setting prior to make"
    );
    assert_eq!(repository.issues[53].state, "closed");

    Ok(())
}

#[test]
fn test_parse_wrong_json() -> Result<(), Box<dyn std::error::Error>> {
    let json_file = Path::new("tests/doc/bogus.json");

    if let Ok(_) = parse_json_input(json_file) {
        panic!("Parser did not catch a wrong json file.")
    }

    Ok(())
}
