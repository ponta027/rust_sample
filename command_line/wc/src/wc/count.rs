use anyhow::Result;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;

#[derive(Debug)]
pub struct FileInfo {
    num_lines: usize,
    num_words: usize,
    num_bytes: usize,
    num_chars: usize,
}

pub struct WcOption {
    pub lines: bool,
    ///word カウント数を表示するかどうか
    pub words: bool,
    ///バイト数を表示するかどうか
    pub bytes: bool,
    /// charactor数を表示するかどうか
    pub chars: bool,
}
fn count(mut file: Box<dyn BufRead>) -> Result<FileInfo> {
    let mut num_bytes = 0;
    let mut num_lines = 0;
    let mut num_chars = 0;
    let mut num_words = 0;
    loop {
        let mut line = String::new();
        let line_byte = file.read_line(&mut line)?;
        if line_byte == 0 {
            break;
        }
        num_bytes += line_byte;
        num_lines += 1;
        num_words += line.split_whitespace().count();
        num_chars += line.chars().count();
        line.clear();
    }
    Ok(FileInfo {
        num_lines,
        num_words,
        num_bytes,
        num_chars,
    })
}
fn word_count_file(file: Box<dyn BufRead>) -> Result<FileInfo> {
    //
    count(file)
}
///
pub fn word_count(files: Vec<String>, mut args: WcOption) -> Result<()> {
    let mut total_lines = 0;
    let mut total_words = 0;
    let mut total_bytes = 0;
    let mut total_chars = 0;

    //オプションがすべてfalseの場合はすべてtrueにする
    if [args.words, args.bytes, args.chars, args.lines]
        .iter()
        .all(|v| v == &false)
    {
        args.lines = true;
        args.words = true;
        args.bytes = true;
    }
    for filename in &files {
        dbg!("{}", filename);
        match open(filename) {
            Err(e) => {
                eprintln!("{filename}:{e}")
            }
            Ok(file) => {
                if let Ok(info) = word_count_file(file) {
                    println!(
                        "{}{}{}{}{}",
                        format_field(info.num_lines, args.lines),
                        format_field(info.num_words, args.words),
                        format_field(info.num_bytes, args.bytes),
                        format_field(info.num_chars, args.chars),
                        if filename == "-" {
                            "".to_string()
                        } else {
                            format!(" {filename}")
                        },
                    );

                    total_lines += info.num_lines;
                    total_words += info.num_words;
                    total_bytes += info.num_bytes;
                    total_chars += info.num_chars;
                }
            }
        }
    }
    if files.len() > 1 {
        println!("===========================");
        println!(
            "{}{}{}{} total",
            format_field(total_lines, args.lines),
            format_field(total_words, args.words),
            format_field(total_bytes, args.bytes),
            format_field(total_chars, args.chars)
        );
    }
    Ok(())
}

fn format_field(value: usize, show: bool) -> String {
    if show {
        format!("{value:>8}")
    } else {
        "".to_string()
    }
}

fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

/*******************************************/
/*******************************************/
/// Boxにしている理由
/// * Stdin
/// * File
/// を両方使えるようにするため
/// コンパイル時にサイズが確定していないため
/// https://doc.rust-jp.rs/rust-by-example-ja/std/box.html
/// dyn:トレイとを返すため
/// https://doc.rust-jp.rs/rust-by-example-ja/trait/dyn.html

/*
fn open2(filename: &str) -> Box<dyn BufRead> {
    match filename {
        "-" => Box::new(BufReader::new(io::stdin())),
        _ => {
            let a = open_file(filename).unwrap();
            Box::new(create_buf_reader(a))
        }
    }
}

fn create_buf_reader(file: File) -> BufReader<File> {
    BufReader::new(file)
}
fn open_file(filename: &str) -> Result<File> {
    Ok(File::open(filename)?)
}
*/

#[cfg(test)]
mod test {
    use super::*;
    /// test pattern
    /// num_lines
    /// num_bytes
    /// num_chars
    /// num_word
    #[test]
    fn test_word_count_empty() -> Result<(), String> {
        let file = open("tests/empty.txt").unwrap();
        let ret = count(file).unwrap();
        assert_eq!(ret.num_lines, 0);
        assert_eq!(ret.num_bytes, 0);
        assert_eq!(ret.num_chars, 0);
        assert_eq!(ret.num_words, 0);
        Ok(())
    }
    #[test]
    fn test_word_count_one_line() -> Result<(), String> {
        let file = open("tests/test_one_line.txt").unwrap();
        let ret = count(file).unwrap();
        assert_eq!(ret.num_lines, 1);
        assert_eq!(ret.num_bytes, 15);
        assert_eq!(ret.num_chars, 15);
        assert_eq!(ret.num_words, 4);
        Ok(())
    }
    #[test]
    fn test_word_count_two_line() -> Result<(), String> {
        let file = open("tests/test_two_line.txt").unwrap();
        let ret = count(file).unwrap();
        assert_eq!(ret.num_lines, 2);
        assert_eq!(ret.num_bytes, 20);
        assert_eq!(ret.num_chars, 20);
        assert_eq!(ret.num_words, 5);
        Ok(())
    }
}
