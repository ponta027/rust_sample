//use anyhow::Result;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

use clap::Parser;

#[derive(Debug, Parser)]
struct Args {
    #[arg(value_name = "FILE", default_value = "-")]
    files: Vec<String>,
    #[arg(short('n'), long("number"), conflicts_with("number_noblank_lines"))]
    nubmber_line: bool,
    #[arg(short('b'), long("number_noblank_lines"))]
    number_noblank_lines: bool,
}
fn main() {
    if let Err(e) = run::<std::io::Error>(Args::parse()) {
        eprintln!("{:?}", e);
        std::process::exit(1);
    }
}

fn run<E>(args: Args) -> Result<(), E> {
    for filename in args.files {
        match open(&filename) {
            Err(e) => eprintln!("{filename}:{e}"),
            Ok(file) => {
                //
                let mut prev_num = 0;
                for (line_num, line_result) in file.lines().enumerate() {
                    //
                    let line = line_result.unwrap();
                    if args.nubmber_line {
                        println!("{line_num}:{line}");
                    } else if args.number_noblank_lines {
                        if line.is_empty() {
                            println!();
                        } else {
                            prev_num += 1;
                            println!("{prev_num:6}\t{line}");
                        }
                    } else {
                        println!("{line}");
                    }
                }
            }
        }
    }
    Ok(())
}

fn open(filename: &str) -> Result<Box<dyn BufRead>, std::io::Error> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
