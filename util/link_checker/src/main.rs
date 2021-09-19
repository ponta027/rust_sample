#[macro_use]
extern crate html5ever;
use html5ever::parse_document;
use html5ever::tendril::*;
use std::collections::HashMap;
use std::default::Default;
use std::fs;
use std::path::Path;

mod parser;
mod walk_dir;

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
    for key in result.a_name {
        if result.a_href.contains(&key) {
            println!("[OK]{}", key);
        } else {
            println!("[NG]{}", key);
        }
    }
}
fn main() {
    let file_extension = ".html";
    for file in walk_dir::read_dir(Path::new("./")).unwrap() {
        match file.rfind(file_extension) {
            Some(size) => {
                if size == file.len() - file_extension.len() {
                    check_link(file);
                }
            }
            _ => {}
        }
    }
}
