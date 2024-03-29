use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::fs;
use std::path::Path;
use std::process::Command;

/*
   These tests are used to check the application's behavior
   depending on arguments passed as input.
*/

const CORRECT_JSON: &str = "tests/doc/cocotb-cocotb_issues.json";
const WRONG_JSON: &str = "tests/doc/bogus.json";
const OUTPUT_ARG: &str = "--output=outunext";
const EXT_OUTPUT_ARG: &str = "--output=outext.csv";

#[test]
fn run_with_existing_file() -> Result<(), Box<dyn std::error::Error>> {
    Command::cargo_bin("issue-tracking")
        .expect("binary exists")
        .args([CORRECT_JSON])
        .assert()
        .success();

    fs::remove_file("out.csv")?;

    Ok(())
}

#[test]
fn run_with_non_existing_file() -> Result<(), Box<dyn std::error::Error>> {
    Command::cargo_bin("issue-tracking")
        .expect("binary exists")
        .args(["no/such/file.txt"])
        .assert()
        .stderr(predicate::str::contains(
            "'no/such/file.txt' does not exist!",
        ))
        .success();

    Ok(())
}

#[test]
fn run_with_wrong_extension_file() -> Result<(), Box<dyn std::error::Error>> {
    Command::cargo_bin("issue-tracking")
        .expect("binary exists")
        .args(["README.md"])
        .assert()
        .stderr(predicate::str::contains("'README.md' is not a json file!"))
        .success();

    Ok(())
}

#[test]
fn run_with_wrong_json_file() -> Result<(), Box<dyn std::error::Error>> {
    Command::cargo_bin("issue-tracking")
        .expect("binary exists")
        .args([WRONG_JSON])
        .assert()
        .stderr(predicate::str::contains(
            "'tests/doc/bogus.json' does not seem to contain GitHub issues.",
        ))
        .success();

    Ok(())
}

#[test]
fn run_with_output() -> Result<(), Box<dyn std::error::Error>> {
    Command::cargo_bin("issue-tracking")
        .expect("binary exists")
        .args([CORRECT_JSON, OUTPUT_ARG])
        .assert()
        .success();

    let output = Path::new(&"outunext.csv");

    if !output.exists() {
        return Err("Expected file 'outunext.csv' does not exist.".into());
    }

    fs::remove_file("outunext.csv")?;

    Ok(())
}

#[test]
fn run_with_extended_output() -> Result<(), Box<dyn std::error::Error>> {
    Command::cargo_bin("issue-tracking")
        .expect("binary exists")
        .args([CORRECT_JSON, EXT_OUTPUT_ARG])
        .assert()
        .success();

    let output = Path::new(&"outext.csv");

    if !output.exists() {
        return Err("Expected file 'outext.csv' does not exist.".into());
    }

    fs::remove_file("outext.csv")?;

    Ok(())
}

#[test]
fn run_with_wrong_outputs() -> Result<(), Box<dyn std::error::Error>> {
    let filenames: [&str; 3] = ["out;", "#out", "out/out"];

    for filename in filenames {
        Command::cargo_bin("issue-tracking")
            .expect("binary exists")
            .args([CORRECT_JSON, "-o", filename])
            .assert()
            .stderr(predicate::str::contains(
                filename.to_owned() + ": filename contains special characters.",
            ))
            .success();
    }

    Ok(())
}

#[test]
fn run_with_label_filter() -> Result<(), Box<dyn std::error::Error>> {
    let filename: &str = "filter.csv";

    Command::cargo_bin("issue-tracking")
        .expect("binary exists")
        .args([CORRECT_JSON, "-o", filename, "-l", "type:feature"])
        .assert()
        .success();

    Ok(())
}

#[test]
fn run_print_labels() -> Result<(), Box<dyn std::error::Error>> {
    Command::cargo_bin("issue-tracking")
        .expect("binary exists")
        .args([CORRECT_JSON, "--print-labels"])
        .assert()
        .stdout(predicate::str::contains(
            "Available labels from tests/doc/cocotb-cocotb_issues.json",
        ))
        .success();

    Ok(())
}
