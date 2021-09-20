#[macro_use]
extern crate html5ever;
use html5ever::parse_document;
use html5ever::tendril::*;
use std::collections::HashMap;
use std::default::Default;
use std::fs;
use std::path::Path;
use structopt::StructOpt;

mod parser;
mod walk_dir;

#[derive(Debug, StructOpt)]
#[structopt(name = "link checker", about = "HTML inner link checker.")]
struct Cli {
    #[structopt(short = "f", conflicts_with_all(&["d"]))]
    f: bool,
    #[structopt(short = "d", conflicts_with_all(&["f"]))]
    d: bool,

    #[structopt(short = "p", long = "path")]
    path: std::path::PathBuf,
}

fn check_link(path: String) {
    println!("[START] check {}", path);
    let sink = parser::Sink {
        next_id: 1,
        names: HashMap::new(),
        a_name: Vec::new(),
        a_href: Vec::new(),
    };
    let input = Path::new(&path);
    let result = parse_document(sink, Default::default())
        .from_utf8()
        .from_file(input)
        .unwrap();
    for key in result.a_href {
        if result.a_name.contains(&key) {
            println!("[OK]{}", key);
        } else {
            println!("[NG]{}", key);
        }
    }
}

fn main() {
    let cli = Cli::from_args();
    let path = cli.path;
    if cli.d {
        let file_extension = ".html";
        if path.is_dir() {
            for file in walk_dir::read_dir(Path::new(&path)).unwrap() {
                match file.rfind(file_extension) {
                    Some(size) => {
                        if size == file.len() - file_extension.len() {
                            check_link(file);
                        }
                    }
                    _ => {}
                }
            }
        } else {
            panic!("{} is not directory", path.to_string_lossy().into_owned());
        }
    }
    if cli.f {
        check_link(path.to_string_lossy().into_owned());
    }
}
