use clap::Parser;
mod uniq;
use crate::uniq::uniq::run;
use crate::uniq::uniq::UniqOption;

#[derive(Debug, Parser)]
struct Args {
    /// Input  file
    #[arg(value_name = "FILE", default_value = "-")]
    in_file: String,
    /// Output file
    #[arg(value_name = "OUT_FILE")]
    out_file: Option<String>,

    /// Show counts
    #[arg(short, long)]
    count: bool,
}
fn main() {
    let arg = Args::parse();

    let _ = run(UniqOption {
        in_file: arg.in_file,
        out_file: arg.out_file,
        count: arg.count,
    });
}
