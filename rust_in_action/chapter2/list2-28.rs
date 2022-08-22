use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let f = File::open("readme.md").unwrap();

    let reader = BufReader::new(f);

    for line_ in reader.lines() {
        let line = line_.unwrap();
        //let line = line_;
        println!("{}({} byte long)", line, line.len());
    }
}
