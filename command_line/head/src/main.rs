use anyhow::Result;
use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug, Parser)]
struct Args {
    //
    #[arg(default_value = "-", value_name = "FILE")]
    files: Vec<String>,

    #[arg(short('n'),
        long,
        default_value="10", 
        value_name="LINES",
        value_parser= clap::value_parser!(u64).range(1..))
    ]
    lines: u64,

    #[arg(short('c'),
        long,
        value_name="BYTES",
        conflicts_with("lines"),
        value_parser=clap::value_parser!(u64).range(1..))
    ]
    bytes: Option<u64>,
}

fn main() {
    let args = Args::parse();
    if let Err(e) = run(args) {
        eprintln!("{e}");
        std::process::exit(1)
    }
}
fn run(args: Args) -> Result<()> {
    let num_files = args.files.len();
    for (file_num, filename) in args.files.iter().enumerate() {
        match open(filename) {
            Err(e) => eprintln!("{:?}", e),
            Ok(mut file) => {
                if num_files > 1 {
                    println!(
                        "{}==> {filename} <== ",
                        if file_num > 0 { "\n" } else { "" },
                    );
                }
                let _result = parsefile(&args, &mut file);
            }
        }
    }
    Ok(())
}

fn parsefile(args: &Args, file: &mut dyn BufRead) -> Result<()> {
    //
    if let Some(byte_num) = args.bytes {
        let mut buffer = vec![0; byte_num as usize];
        let byte_read = file.read(&mut buffer)?;
        println!("{}", String::from_utf8_lossy(&buffer[..byte_read]));
        //
    } else {
        //
        let mut line = String::new();
        for _ in 0..args.lines {
            let bytes = file.read_line(&mut line)?;
            if bytes == 0 {
                break;
            }
            println!("{line}");
            line.clear();
        }
    }
    Ok(())
}
fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
