use std::io::Read;
use std::io::{BufRead,BufReader};

pub fn sample(){
    println!("[START] sample_12_1_1 ");
    sample_12_1_1();
    sample_12_1_3();
    sample_12_1_4();
    sample_12_1_5();
    sample_12_1_6();
    sample_12_1_7();
    println!("[END]read");
}

fn sample_12_1_1 (){
    let path = "sample.txt";
    println!("read all lines");
    if let Ok(data) = std::fs::read_to_string(path) {
        println!("data is {}", data);
    } else {
        println!("read error");
    }
}
fn sample_12_1_3 (){
    println!(" error handling");
    let path = "unknown.txt";
    println!("read all lines.");
    match std::fs::read_to_string( path ){
        Ok(data) => {println!("data is {}",data)},
        _       => { println!(" cannot open {}",path)},
    }
}

fn sample_12_1_4 (){

    let path = "sample.txt";
    println!( "read all lines by buffer");
    let mut file = std::fs::File::open( path ).expect("file not found");
    let mut data = String::new();
    file.read_to_string( &mut data).expect(" read error");

    println!(" data is {}", data );

}
//

fn sample_12_1_5 (){
    let path = "sample.txt";
    println!("read all lines by buffer");
    if let Ok(mut file) = std::fs::File::open(path){
        let mut data = String::new();
        if let Ok(_) = file.read_to_string( &mut data ) {
            println!(" data is {}", data );
        }
    }
}
//
fn sample_12_1_6(){
    // read byte
    let path="sample.txt";
    println!( "read 16 bytes by buffer");
    let mut file = std::fs::File::open(path).expect("file not found");
    let mut buf:[u8; 1 ]= [0;1];
    for i in 0 ..16{
        file.read( &mut buf);
        println!("buf is {}:{}", i , buf[0] as char);
    }
}
//
fn sample_12_1_7 (){
    println!(" read every one line.");
    let path = "sample.txt";
    let file = std::fs::File::open(path).expect("file not found");
    for line in BufReader::new(file).lines(){
        if let Ok(l) = line {
        println!("line is {}",l);
        }
    }
}
