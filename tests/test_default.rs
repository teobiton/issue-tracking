use std::process::Command;
use assert_cmd::prelude::*;

#[test]
fn default_run() -> Result<(), Box<dyn std::error::Error>> {
    Command::cargo_bin("issue-parser")
        .expect("binary exists")
        .assert()
        .success();
    
    Ok(())
}