use crate::Args;
use anyhow::Result;
use csv::{ReaderBuilder, StringRecord, WriterBuilder};
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::ops::Range;

type PositionList = Vec<Range<usize>>;
#[derive(Debug)]
enum Extract {
    Fields(PositionList),
    Bytes(PositionList),
    Chars(PositionList),
}

/// 1. 指定したファイルに対して処理
/// 2. オプション判定
/// 3. Fields指定   
/// 3.1 デリミタ分割で読み出す
/// 3.2 指定したポジションの内容を標準出力する
///
pub fn run(args: Args) -> Result<()> {
    for filename in args.files {
        //
        let _ = parse_with_delimiter(args.delimiter.as_bytes().first(), 1, filename);
    }
    Ok(())
}

fn parse_with_delimiter(delimiter: Option<&u8>, field_pos: usize, filename: String) -> Result<()> {
    match open(&filename) {
        Err(err) => eprintln!("{filename} {err}"),
        Ok(file) => {
            let mut reader = ReaderBuilder::new()
                .delimiter(*delimiter.unwrap())
                .has_headers(false)
                .from_reader(file);
            for record in reader.records() {
                println!(
                    "{:?}",
                    extract_fields(&record?, &[std::ops::Range { start: 1, end: 3 }])
                );
            }
        }
    }

    Ok(())
}

///
fn extract_fields<'a>(record: &'a StringRecord, field_pos: &'a [Range<usize>]) -> Vec<&'a str> {
    field_pos
        .iter()
        .cloned()
        .flat_map(|range| range.filter_map(|i| record.get(i)))
        .collect()
}

fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))), // 標準入力
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))), //ファイル入力
    }
}

fn parse_pos(range: String) -> Result<PositionList> {
    let tmp = Vec::new();
    Ok((tmp))
}
fn parse_index(input: &str) -> Result<usize> {
    Ok((0))
}
