use std::fs::*;
pub fn sample(){
    println!("[START]read 2");
    sample_12_1_1_3();
    println!("[END]read 2");
}

fn sample_12_1_1_3 (){
    let path = "sample.txt";
    println!("read all lines");
    if let Ok(data) = read_to_string(path) {
        println!("data is {}", data);
    } else {
        println!("read error");
    }
}
