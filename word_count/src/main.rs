use clap::{App, Arg};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

fn main() {
    let matches = App::new("File Reader")
        .version("1.0")
        .author("ponta27")
        .about("Reads the contents of a file")
        .arg(
            Arg::with_name("file")
                .help("Sets the input file to read")
                .required(true)
                .index(1),
        )
        .get_matches();

    let file_path = matches.value_of("file").unwrap();
    let mut file = match File::open(file_path) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Error opening file {}: {}", file_path, err);
            std::process::exit(1);
        }
    };

    //    let reader = io::BufReader::new(file);
    let mut contents = String::new();
    let file_text = file.read_to_string(&mut contents);
    println!("file:{}:{:?}", file_path, file_text);
    // 分析する文章
    let text = contents;
    // ピリオドをスペースに置換
    let text = text.replace(".", " ");

    // 単語の出現回数を記録するHashMapを作成
    let mut word_counts = HashMap::new();

    // 文章を単語に分割して単語ごとの出現回数をカウント
    for word in text.split_whitespace() {
        // 単語を小文字に変換してからカウント
        let word = word.to_lowercase();

        // 単語の出現回数を更新
        let count = word_counts.entry(word).or_insert(0);
        *count += 1;
    }

    // 結果を表示
    println!("Word frequencies:");
    for (word, count) in &word_counts {
        println!("{}: {}", word, count);
    }
}
