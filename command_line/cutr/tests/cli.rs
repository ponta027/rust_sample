use anyhow::Result;
use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

const PRG: &str = "cutr";

#[test]
fn test_usage() -> Result<()> {
    let expected = "Usage: cutr [OPTIONS] [FILES]...";

    for flag in &["-h", "--help"] {
        Command::cargo_bin(PRG)?
            .arg(flag)
            .assert()
            .stdout(predicate::str::contains(expected));
    }
    Ok(())
}
#[test]
fn test_command1() -> Result<()> {
    let expected_file = "tests/expected/test_csv_result.txt";
    let args = ["tests/inputs/sample.csv", "-d", ","];
    let cmd = Command::cargo_bin(PRG)?.args(args).assert().success();
    let out = cmd.get_output();
    let stdout = String::from_utf8(out.stdout.clone())?;

    let expected = fs::read_to_string(expected_file)?;
    assert_eq!(stdout, expected);
    Ok(())
}
