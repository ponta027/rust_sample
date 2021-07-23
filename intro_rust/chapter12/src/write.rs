use std::fs::File;
use std::io::Write;


pub fn sample(){
    println!("[START]write");
    sample_12_2_1();
    sample_12_2_2();
    sample_12_2_3();
    sample_12_2_4();
    println!("[END]write");
}

//
fn sample_12_2_1(){
    let path = "sampe-out.txt";
    let mut file = File::create( path ).expect(" file not found");
    writeln!( file , "hello rust world.").expect(" cannnot write");
        
}
//
fn sample_12_2_2 (){
    let path = "sample-out2.txt";
    let mut file = File::create( path ).expect(" file not found");
    file.write(  b"hello rust world.\n").expect(" cannnot write");
}
//
fn sample_12_2_3 (){
    let path= "sample-out3.txt";
    let mut file = File::create(path).expect("file not found");
    let s = "hello rust world \n";
    file.write( s.as_bytes() ).expect(" cannot write");
}
//


fn sample_12_2_4 (){
    let path ="sample-out4.txt";
    let mut file = File::create(path).expect(" file not found");
    let s = "hello rust world\n";
    for it in s.as_bytes(){
        file.write(&[*it]).expect(" cannot write");
    }
}
