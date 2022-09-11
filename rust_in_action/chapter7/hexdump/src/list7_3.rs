use std::io::prelude::*;

const BYTES_PER_LINE: usize = 16;
const INPUT: &'static [u8] = br#"
fn main(){
    println!("Hello World");
}
"#;

pub fn list7_3() {
    let mut buffer: Vec<u8> = vec![];

    //https://doc.rust-lang.org/std/io/trait.Read.html#method.read_to_end
    INPUT.read_to_end(&mut buffer).unwrap();

    let mut position_in_input = 0;
    for line in buffer.chunks(BYTES_PER_LINE) {
        print!("[0x{:08x}] ", position_in_input);
        for byte in line {
            print!("{:02x} ", byte);
        }
        println!("");
        position_in_input += BYTES_PER_LINE;
        //
    }
}
