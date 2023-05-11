use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

/*
   These tests are used to check the application's behavior
   depending on arguments passed as input.
*/

const JSON_FILE: &str = "tests/doc/cocotb-cocotb_issues.json";

#[test]
fn run_with_existing_file() -> Result<(), Box<dyn std::error::Error>> {
    Command::cargo_bin("issue-parser")
        .expect("binary exists")
        .args(&[JSON_FILE])
        .assert()
        .stdout(predicate::str::contains(JSON_FILE))
        .success();

    Ok(())
}

#[test]
fn run_with_non_existing_file() -> Result<(), Box<dyn std::error::Error>> {
    Command::cargo_bin("issue-parser")
        .expect("binary exists")
        .args(&["no/such/file.txt"])
        .assert()
        .stderr(predicate::str::contains(
            "'no/such/file.txt' does not exist!",
        ))
        .failure();

    Ok(())
}

#[test]
fn run_with_wrong_extension_file() -> Result<(), Box<dyn std::error::Error>> {
    Command::cargo_bin("issue-parser")
        .expect("binary exists")
        .args(&["README.md"])
        .assert()
        .stderr(predicate::str::contains("'README.md' is not a json file!"))
        .failure();

    Ok(())
}
