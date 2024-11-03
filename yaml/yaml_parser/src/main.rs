use crate::openapi::parser::OpenApi;
use linked_hash_map::LinkedHashMap;
use std::fs;
use yaml_rust::yaml::Array;
use yaml_rust::Yaml;
use yaml_rust::YamlLoader;

use clap::Parser;

mod openapi;

#[derive(Debug, Parser)]
struct Args {
    #[arg(value_name = "FILE", default_value = "sample.yaml")]
    files: String,
}

///
/// refs: https://docs.rs/yaml-rust/latest/yaml_rust/
fn main() {
    let args = Args::parse();
    let filename = args.files;
    let s = fs::read_to_string(&filename).unwrap().to_string();
    let docs = YamlLoader::load_from_str(&s);
    match docs {
        Ok(docs) => {
            let doc = &docs[0];
            search_hash(doc.as_hash().unwrap(), 0);
        }
        Err(e) => {
            eprintln!("{e}");
        }
    };
    let ret = OpenApi::new(filename);
    println!("{:?}", ret);
    //    parse(&filename);
}
///
fn search_array(array: &Array, depth: usize) {
    //
    for val in array {
        match val {
            Yaml::Hash(val) => {
                search_hash(val, depth + 1);
            }
            Yaml::Array(val) => {
                for v in val {
                    println!("{}{:?}", "\t".repeat(depth + 1), v.as_str());
                }
            }
            Yaml::String(val) => {
                println!("{}<string>{:?}", "\t".repeat(depth + 1), val.as_str());
            }
            Yaml::Boolean(val) => {
                println!("{}<boolean>{:?}", "\t".repeat(depth + 1), val);
            }
            Yaml::Integer(val) => {
                println!("{}<int>{:?}", "\t".repeat(depth + 1), val);
            }
            Yaml::Real(val) => {
                println!("{}<real>{:?}", "\t".repeat(depth + 1), val);
            }

            _ => {}
        }
    }
}

fn search_hash(hash: &LinkedHashMap<Yaml, Yaml>, depth: usize) {
    for v2 in hash.into_iter() {
        println!("{}{:?}", "\t".repeat(depth), v2.0.as_str().unwrap());
        let val = v2.1;
        match val {
            Yaml::Hash(val) => {
                search_hash(val, depth + 1);
            }
            Yaml::Array(val) => {
                search_array(val, depth + 1);
            }
            Yaml::String(val) => {
                println!("{}<string>{:?}", "\t".repeat(depth + 1), val.as_str());
            }
            Yaml::Boolean(val) => {
                println!("{}<boolean>{:?}", "\t".repeat(depth + 1), val);
            }
            Yaml::Integer(val) => {
                println!("{}<int>{:?}", "\t".repeat(depth + 1), val);
            }
            Yaml::Real(val) => {
                println!("{}<real>{:?}", "\t".repeat(depth + 1), val);
            }

            _ => {}
        }
    }
}
