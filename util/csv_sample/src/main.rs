use std::error::Error;
use std::fs::File;
use std::path::Path;

/**
 */
fn process_csv(input_file: &str) -> Result<(), Box<dyn Error>> {
    let input_path = Path::new(input_file);

    let input_file = File::open(input_path)?;
    let mut reader = csv::Reader::from_reader(input_file);
    let mut result_msg = Vec::new();

    for result in reader.records() {
        let record = result?;
        let mod_record = record
            .iter()
            .enumerate()
            .map(|(i, elem)| {
                if i == 0 {
                    format!("* #{}", elem)
                } else {
                    String::from(elem)
                }
            })
            .collect::<Vec<String>>()
            .join("    ");
        result_msg.push(mod_record);
    }

    println!("{}", result_msg.join("\n"));

    Ok(())
}

fn main() {
    let input_file = "input.csv";

    if let Err(err) = process_csv(input_file) {
        eprintln!("Error: {}", err);
    } else {
        println!("CSV processing complete.");
    }
}
