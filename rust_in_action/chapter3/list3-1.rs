#![allow(unused_variables)]
type File = String;
fn open(f: &mut File) -> bool {
    true
}
fn close(f: File) -> bool {
    true
}
#[allow(dead_code)]

fn read(f: File, save_to: &mut Vec<u8>) -> ! {
    unimplemented!()
}

fn main() {
    let mut f1 = File::from("f1.txt");
    open(&mut f1);
    close(f1);
}
