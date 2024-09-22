use crate::wc::count::word_count;
use crate::wc::count::WcOption;
use anyhow::Result;
use clap::Parser;

mod wc;

/// Word Count Command
/// オプションを指定しない場合はすべてtrueとする
/// FILE が指定されないか、 FILE が - の場合、 標準入力から読み込みます。
/// 単語は、空白類文字で区切られる、長さが 0 でない表示可能文字の列です。
#[derive(Parser, Debug)]
struct Args {
    ///ファイルパス
    #[arg(value_name = "FILE", default_value = "-")]
    files: Vec<String>,
    /// 行数を表示するかどうかの判定
    #[arg(short, long)]
    lines: bool,
    ///word カウント数を表示するかどうか
    #[arg(short, long)]
    words: bool,
    ///バイト数を表示するかどうか
    #[arg(short, long)]
    bytes: bool,
    /// charactor数を表示するかどうか
    #[arg(short, long)]
    chars: bool,
}

fn main() {
    if let Err(e) = run(Args::parse()) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

///
fn run(arg: Args) -> Result<()> {
    let _ = word_count(
        arg.files,
        WcOption {
            lines: arg.lines,
            words: arg.words,
            bytes: arg.bytes,
            chars: arg.chars,
        },
    );

    Ok(())
}
