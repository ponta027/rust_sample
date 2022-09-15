//use std::fs::File;
use std::fs::OpenOptions;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;
use std::io::Write;
//
use std::path::Path;
use std::path::PathBuf;

fn main() {
    //https://stackoverflow.com/questions/69341037/openoptions-write-not-working-properly-when-writing-to-filehttps://stackoverflow.com/questions/69341037/openoptions-write-not-working-properly-when-writing-to-file
    let path = "sample.txt";
    let mut f = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .append(true)
        .open(path)
        .unwrap();

    f.write_all(String::from("HOGE").as_bytes()).unwrap();

    let _result = f.seek(SeekFrom::Start(0));
    let mut content = String::new();
    f.read_to_string(&mut content).unwrap();
    println!("content:{}", content);
    println!("Hello World");
    {
        println!("7.4.2 std::fs::Path");
        let mut hello = PathBuf::from("/tmp/hello.txt");
        println!("{:?}", hello.extension());
        hello.pop();
        println!("{:?}", hello.display());
    }
}
