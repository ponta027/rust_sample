use clap::{App, Arg};
use regex::Regex;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

fn list_2_29() {
    let args = App::new("grep-lite")
        .version("0.1")
        .about("searcges for patterns")
        .arg(
            Arg::with_name("pattern")
                .help("The pattern to search for ")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("input")
                .help("File to search")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    let pattern = args.value_of("pattern").unwrap();
    let re = Regex::new(pattern).unwrap();
    let input = args.value_of("input").unwrap();
    let f = File::open(input).unwrap();
    let reader = BufReader::new(f);

    //    println!("{}:{}", input, pattern);
    for line_ in reader.lines() {
        let line = line_.unwrap();
        //        println!("{}", line);
        match re.find(&line) {
            Some(_) => println!("{}", line),
            None => (),
        }
    }

    //
}
fn list_2_30() {
    let args = App::new("grep-lite")
        .version("0.1")
        .about("searcges for patterns")
        .arg(
            Arg::with_name("pattern")
                .help("The pattern to search for ")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("input")
                .help("File to search")
                .takes_value(true)
                .required(true),
        )
        .get_matches();
    let pattern = args.value_of("pattern").unwrap();
    let re = Regex::new(pattern).unwrap();
    let input = args.value_of("input").unwrap();

    if input == "-" {
        let stdin = io::stdin();
        let reader = stdin.lock();
        process_lines(reader, re);
    } else {
        let f = File::open(input).unwrap();
        let reader = BufReader::new(f);
        process_lines(reader, re);
    }
}

fn process_lines<T: BufRead + Sized>(reader: T, re: Regex) {
    for line_ in reader.lines() {
        let line = line_.unwrap();
        match re.find(&line) {
            Some(_) => println!("{}", line),
            None => {} //
        }
    }
    //
}
fn main() {
    //list_2_29();
    list_2_30();
    /*
    let args = App::new("grep-lite")
        .version("0.1")
        .about("searcges for patterns")
        .arg(
            Arg::with_name("pattern")
                .help("The pattern to search for ")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("input")
                .help("File to search")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    /*
        let search_term = "picture";
        let quote = "\
    Every face ,every shop,bed room window, public-house , and
    dark square is a picture feverishlyturned--in search of what?
    It is the same with books.
    What dow wee seek through millions of oages?";
        */
    /*
    for line in quote.lines() {
        if line.contains(search_term) {
            println!("{}", line);
        }
        //
    }
    */

    /*
    let re = Regex::new(pattern).unwrap();
    for line in quote.lines() {
        let contains_substring = re.find(line);
        match contains_substring {
            Some(_) => println!("{}", line),
            None => (),
        }
    }
    */

    // list2-29
    let pattern = args.value_of("pattern").unwrap();
    let re = Regex::new(pattern).unwrap();
    let input = args.value_of("input").unwrap();
    let f = File::open(input).unwrap();
    let reader = BufReader::new(f);

    //    println!("{}:{}", input, pattern);
    for line_ in reader.lines() {
        let line = line_.unwrap();
        //        println!("{}", line);
        match re.find(&line) {
            Some(_) => println!("{}", line),
            None => (),
        }
    }
    */
}
