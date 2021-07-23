use std::fs::File;
use std::io::{Read,Write,BufReader,BufWriter};
use std::env;

pub fn sample(){
    println!("[START] 12.3");
    sample_12_3_3();
//    sample_12_3_4();
    sample_12_3_5();
    println!("[END] 12.3");
}


fn sample_12_3_3 (){
    let args = env::args().collect::<Vec<String>>();
    if args.len()<=1{
    }else{
        let file = File::open(&args[1]).expect(" file not found");
        let reader = BufReader::new(file);
        let mut writer = BufWriter::new(std::io::stdout());
        for it in reader.bytes(){
            /* difference from text. unwrap*/
            //let ch = it.unwrap();
            writer.write( &[it.unwrap()] );
            //writer.write( &[ch] );
        }
    }
}

fn sample_12_3_4 (){
    println!("sample_12_3_4");
    let reader = BufReader::new( std::io::stdin());
    let mut writer = BufWriter::new ( std::io::stdout() );
    println!( " read from stdin:");
    for it in reader.bytes(){
        writer.write( &[it.unwrap()] );
    }
}

fn sample_12_3_5 (){
    println!("sample_12_3_5");
    let reader = BufReader::new( std::io::stdin() );
    do_print( reader );

}

fn do_print<R>( reader : BufReader<R>) where R: std::io::Read{
    let mut writer = BufWriter::new( std::io::stdout());
    for it in reader.bytes(){
        writer.write(&[it.unwrap()] );
    }

}
