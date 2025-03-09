use anyhow::Result;
use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

const PRG: &str = "find";

#[test]
fn usage() -> Result<()> {
    for flag in &["-h", "--help"] {
        Command::cargo_bin(PRG)?
            .arg(flag)
            .assert()
            .stdout(predicate::str::contains("Usage: find [OPTIONS] [PATH]"));
    }
    Ok(())
}
#[test]
fn command() -> Result<()> {
    let _ = run(&["tests/sample"], &"tests/expected/simple_command.txt");
    Ok(())
}
#[test]
fn test_type_file() -> Result<()> {
    run(
        &["tests/name_filter", "--type", "f"],
        &"tests/expected/filter_type.txt",
    );
    Ok(())
}
///
#[test]
fn test_type_file_and_name() -> Result<()> {
    run(
        &["tests/name_filter", "--type", "f", "--name", "abc*"],
        &"tests/expected/filter_type_name.txt",
    );
    Ok(())
}
///
#[test]
fn test_type_dir() -> Result<()> {
    run(
        &["tests/name_filter", "--type", "d"],
        &"tests/expected/filter_dir.txt",
    );
    Ok(())
}

fn run(args: &[&str], expected_file: &str) -> Result<()> {
    let expected = fs::read_to_string(expected_file)?;
    /*
    Command::cargo_bin(PRG)?
        .args(args)
        .assert()
        .stdout(predicate::str::is_match(&expected)?);
    */
    let cmd = Command::cargo_bin(PRG)?.args(args).assert().success();
    let out = cmd.get_output();
    let stdout = String::from_utf8(out.stdout.clone())?;
    assert_eq!(stdout, expected);
    Ok(())
}
