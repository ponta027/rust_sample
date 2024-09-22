use crate::plantuml::extract;
use clap::Parser;
use plantuml_parser::PlantUmlFileData;
use std::fs::read_to_string;
use std::io;
use std::path::PathBuf;

mod plantuml;

#[derive(Parser, Debug)]
/// description for command.
struct Arg {
    #[clap(short, long)]
    /// description for this argument.
    file: PathBuf,
}

fn main() -> io::Result<()> {
    let args = Arg::parse();
    let data = read_to_string(args.file)?;
    let filedata = PlantUmlFileData::parse_from_str(data).unwrap();
    let content = filedata.get(0).unwrap();
    let result = extract::parse_message(content.inner());
    for item in result {
        println!(
            "{},{},{:?},{}",
            item.from, item.to, item.message, item.message_type
        );
    }
    Ok(())
}
