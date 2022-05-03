use encoding_rs;
use std::collections::BTreeMap;
use std::fs;
use std::io;
use std::iter::FromIterator;
use std::path::Path;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "word count", about = "Word Count Tool")]
struct Cli {
    #[structopt(short = "f", conflicts_with_all(&["d"]))]
    f: bool,
    #[structopt(short = "d", conflicts_with_all(&["f"]))]
    d: bool,
    #[structopt(short = "p", long = "path")]
    path: std::path::PathBuf,
}

/**
 *
 */
fn read_dir(path: &Path) -> io::Result<Vec<String>> {
    Ok(fs::read_dir(path)?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            if entry.file_type().ok()?.is_file() {
                Some(
                    path.to_string_lossy().into_owned()
                        + "/"
                        + &entry.file_name().to_string_lossy().into_owned(),
                )
            } else {
                None
            }
        })
        .collect())
}
/**
 */
fn read_file(file_name: &str) -> String {
    let s = fs::read(file_name).unwrap();
    let (res, _, _) = encoding_rs::SHIFT_JIS.decode(&s);
    let contents = res.into_owned();
    contents
        .replace('"', " ")
        .replace("'", " ")
        .replace("\n", " ") //
        .replace("\r", " ") //
        .replace(";", " ") //
        .replace(":", " ") //
        .replace(":", " ") //
        .replace(",", " ") //
        .replace("<", " ") //
        .replace(">", " ") //
        .to_string()
}

fn main() {
    let cli = Cli::from_args();
    let mut word_map = BTreeMap::new();
    let path = cli.path;
    let mut file_list: Vec<String> = Vec::new();
    if cli.d {
        let file_extension = ".html";
        if path.is_dir() {
            for file in read_dir(Path::new(&path)).unwrap() {
                match file.rfind(file_extension) {
                    Some(size) => {
                        if size == file.len() - file_extension.len() {
                            file_list.push(file);
                        }
                    }
                    _ => {}
                }
            }
        } else {
            panic!("{} is not directory", path.to_string_lossy().into_owned());
        }
    } else if cli.f {
        let target_file = path.to_string_lossy().into_owned();
        file_list.push(target_file);
    }

    for file_path in file_list {
        println!("path={}", file_path);
        let result = read_file(&file_path);
        let split: Vec<&str> = result.split(" ").collect();
        for item in split {
            //for item in data {
            *word_map.entry(item.trim().to_string()).or_insert(0) += 1;
        }
    }
    let mut v = Vec::from_iter(&word_map);
    v.sort_by(|&(_, a), &(_, b)| b.cmp(&a));
    for item in v {
        println!("key [{}] , val [{}]", item.0, item.1);
    }
}
