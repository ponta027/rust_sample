/**
 * error code of global variables
 */
#![allow(unused_variables)]
static mut ERROR: i32 = 0;

#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
}

impl File {
    fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: Vec::new(),
        }
    }
    fn read(self: &File, save_to: &mut Vec<u8>) -> usize {
        let read_length = 0;
        let mut tmp = self.data.clone();

        let read_length = tmp.len();
        save_to.reserve(read_length);
        save_to.append(&mut tmp);

        read_length
    }
}
fn close(file: &mut File) -> bool {
    true
}

fn main() {
    let mut f = File::new("something.txt");
    let mut buffer: Vec<u8> = vec![];

    f.read(&mut buffer);

    unsafe {
        if ERROR != 0 {
            panic!("error occurs in reading file.");
        }
    }

    close(&mut f);
    unsafe {
        if ERROR != 0 {
            panic!("error occurs in closing file.");
        }
    }
}
