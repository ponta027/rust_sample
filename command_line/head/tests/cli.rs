use anyhow::Result;
use assert_cmd::Command;
use predicates::prelude::*;
use rand::{distributions::Alphanumeric, Rng};
use std::fs;

const PRG: &str = "head";
const EMPTY: &str = "./tests/inputs/empty.txt";
const ONE: &str = "./tests/inputs/one.txt";

/**
 * make file name not exist.
 */
fn gen_bad_file() -> String {
    loop {
        let filename = random_string();
        if fs::metadata(&filename).is_err() {
            return filename;
        }
    }
}

fn random_string() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect()
    //
}
#[test]
fn die_bad_bytes() -> Result<()> {
    let bad = random_string();
    let expected = format!(
        "invalid value '{bad}' for \
                        '--bytes <BYTES>': invalid digit found in string"
    );
    Command::cargo_bin(PRG)?
        .args(["-c", &bad, EMPTY])
        .assert()
        .failure()
        .stderr(predicate::str::contains(expected));
    Ok(())
}

#[test]
fn die_bad_line() -> Result<()> {
    let bad = random_string();

    let expected = format!(
        "invalid value '{bad}' for \
        '--lines <LINES>': invalid digit found in string"
    );
    Command::cargo_bin(PRG)?
        .args(["-n", &bad, EMPTY])
        .assert()
        .failure()
        .stderr(predicate::str::contains(expected));

    Ok(())
}

#[test]
fn die_bytes_and_lines() -> Result<()> {
    let msg = "the argument '--lines <LINES>' cannot be \
               used with '--bytes <BYTES>'";
    Command::cargo_bin(PRG)?
        .args(["-n", "1", "-c", "2", EMPTY])
        .assert()
        .failure()
        .stderr(predicate::str::contains(msg));
    Ok(())
}

#[test]
fn skips_bad_file() -> Result<()> {
    let bad = gen_bad_file();
    let expected = format!("{bad}: .* [(]os error 2[)]");
    Command::cargo_bin(PRG)?
        .args([EMPTY, &bad, ONE])
        .assert()
        .stderr(predicate::str::is_match(expected)?);

    Ok(())
}
