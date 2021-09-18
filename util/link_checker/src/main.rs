#[macro_use]
extern crate html5ever;

use html5ever::parse_document;
use html5ever::tendril::*;
use std::collections::HashMap;
use std::default::Default;
use std::path::Path;
use std::str;

mod parser;

fn main() {
    let sink = parser::Sink {
        next_id: 1,
        names: HashMap::new(),
        a_name: Vec::new(),
        a_href: Vec::new(),
    };
    let input = Path::new("sample.html");
    let result = parse_document(sink, Default::default())
        .from_utf8()
        .from_file(input)
        .unwrap();
    for (key, name) in result.names {
        println!("{}={:?}", key, name);
        println!("{:?}", str::from_utf8(&name.local.as_bytes()).unwrap());
    }

    for key in result.a_name {
        if result.a_href.contains(&key) {
            println!("[OK]{}", key);
        } else {
            println!("[NG]{}", key);
        }
    }
}
