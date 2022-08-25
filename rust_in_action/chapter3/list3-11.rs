#![allow(unused_variables)]

#[derive(Debug, PartialEq)]
enum FileState {
    Open,
    Close,
}
#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
    state: FileState,
}

impl File {
    fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: Vec::new(),
            state: FileState::Close,
        }
    }
    fn new_with_data(name: &str, data: &Vec<u8>) -> File {
        /*
        File {
            name: String::from(name),
            data: data.clone(),
        }
        */
        let mut f = File::new(name);
        f.data.clone_from(data);
        // f.data = data.clone();
        // f.data = *data;
        f
    }

    fn read(self: &File, save_to: &mut Vec<u8>) -> Result<usize, String> {
        if self.state != FileState::Open {
            return Err(String::from("File must be open for reading"));
        }
        let read_length = 0;
        let mut tmp = self.data.clone();

        let read_length = tmp.len();
        save_to.reserve(read_length);
        save_to.append(&mut tmp);

        Ok(read_length)
    }
}
fn open(mut file: File) -> Result<File, String> {
    file.state = FileState::Open;
    Ok(file)
}
fn close(mut file: File) -> Result<File, String> {
    file.state = FileState::Close;
    Ok(file)
}

fn main() {
    let f5_data: Vec<u8> = vec![114, 117, 115, 116, 33];
    let mut f5 = File::new_with_data("5.txt", &f5_data);
    let mut buffer: Vec<u8> = vec![];

    if f5.read(&mut buffer).is_err() {
        println!("Error checking is working");
    }

    f5 = open(f5).unwrap();
    let f5_length = f5.read(&mut buffer).unwrap();
    f5 = close(f5).unwrap();

    let text = String::from_utf8_lossy(&buffer);

    println!("{:?}", f5);
    println!("{} is {} byte long", &f5.name, f5_length);
    println!("{}", text);

    //
    //
}
