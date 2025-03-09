use anyhow::Result;
use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

const PRG: &str = "uniq";
#[test]
fn test() -> Result<()> {
    let expected_file = "tests/sample_result.txt";
    let expected = fs::read_to_string(expected_file)?;
    let arg = "tests/sample.txt";
    Command::cargo_bin(PRG)?
        .arg(&arg)
        .assert()
        .success()
        .stdout(predicate::str::contains(expected));
    Ok(())
}
///
#[test]
fn test_argerror() -> Result<()> {
    let expected = format!("error: unexpected argument '--sample' found");
    let arg = "--sample";
    Command::cargo_bin(PRG)?
        .arg(&arg)
        .assert()
        .stderr(predicate::str::contains(expected));
    Ok(())
}
///
#[test]
fn test_arg_file_not_found() -> Result<()> {
    let bad = "not_found.txt";
    //    let expected = format!("{bad}:.*[(]os error 2[)]");
    let expected = format!("{bad}:No such file or directory (os error 2)");
    Command::cargo_bin(PRG)?
        .arg(&bad)
        .assert()
        .stderr(predicate::str::contains(expected));
    Ok(())
}
