use assert_cmd::prelude::*;
use std::process::Command;

/*
   This test only checks if the application compiles.
   It should always pass CI; if it does not than the
   application did not build correctly.
*/

#[test]
fn default_run() -> Result<(), Box<dyn std::error::Error>> {
    Command::cargo_bin("issue-tracking")
        .expect("binary exists")
        .assert()
        .failure(); /* We expect a failure because no args are provided */

    Ok(())
}
