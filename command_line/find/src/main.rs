mod find_cmd;
use crate::find_cmd::find::EntryType;
use clap::ArgAction;
use clap::Parser;
use regex::Regex;

#[derive(Debug, Parser)]
/// find command
struct Args {
    /// search paths.
    #[arg(value_name = "PATH", default_value = ".")]
    paths: Vec<String>,
    /// Names
    #[arg(short('n'), long("name"), value_name = "NAME",value_parser(Regex::new),action(ArgAction::Append),num_args(0..))]
    names: Vec<Regex>,

    #[arg(
        short('t'),
        long("type"),
        value_name="TYPE",
        value_parser(clap::value_parser!(EntryType)),
        action(ArgAction::Append),
        num_args(0..2)
        )]
    entry_types: Vec<EntryType>,
}
//
use crate::find_cmd::find::run;
fn main() {
    let arg = Args::parse();
    //    println!("{:?}", arg);
    if let Err(e) = run(arg) {
        eprintln!("{:?}", e);
        std::process::exit(1);
    }
}
