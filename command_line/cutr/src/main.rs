mod cutr;
use clap::ArgAction;
use clap::Parser;

#[derive(Debug, Parser)]
struct Args {
    /// files
    #[arg(default_value = "-")]
    files: Vec<String>,
    /// delimiter
    #[arg(short, long, value_name = "DELIMITER", default_value = "\t")]
    delimiter: String,
    #[command(flatten)]
    extract: ArgExtract,
}

#[derive(Debug, Parser)]
struct ArgExtract {
    /// selected Field
    #[arg(short, long, value_name = "FIELDS")]
    fields: Option<String>,

    /// selected bytes
    #[arg(short, long, value_name = "BYTES")]
    bytes: Option<String>,

    /// selected chars
    #[arg(short, long, value_name = "CHARS")]
    chars: Option<String>,
}

fn main() {
    let args = Args::parse();
    let _ = cutr::mycut::run(args);
}
