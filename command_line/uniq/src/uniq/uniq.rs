use anyhow::anyhow;
use anyhow::Result;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

pub struct UniqOption {
    pub in_file: String,
    pub out_file: Option<String>,
    pub count: bool,
}

pub fn run(option: UniqOption) -> Result<()> {
    let mut file = open(&option.in_file).map_err(|e| anyhow!("{}:{e}", option.in_file))?;

    let mut out_file: Box<dyn Write> = match &option.out_file {
        Some(out_name) => Box::new(File::create(out_name)?),
        _ => Box::new(io::stdout()),
    };
    let mut print = |num: u64, text: &str| {
        //
        if num > 0 {
            write!(out_file, "{num:>4} {text}").unwrap();
        } else {
            write!(out_file, "{text}").unwrap();
        }
    };

    let mut line = String::new();
    let mut previous = String::new();
    let mut count: u64 = 0;
    loop {
        let bytes = file.read_line(&mut line)?;
        if bytes == 0 {
            break;
        }
        // compare previous line.
        if line.trim_end() != previous.trim_end() {
            //
            print(count, &previous);
            previous = line.clone();
            count = 0;
        }
        count += 1;
        line.clear();
    }
    // print last line
    print(count, &previous);

    Ok(())
}
fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
