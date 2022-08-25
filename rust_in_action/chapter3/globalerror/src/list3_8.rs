use rand::prelude::*;

#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
}

fn one_in(denominator: u32) -> bool {
    thread_rng().gen_ratio(1, denominator)
}

impl File {
    fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: vec![],
        }
    }
    fn new_with_data(name: &str, data: &Vec<u8>) -> File {
        let mut f = File::new(name);
        f.data.clone_from(data);
        f
    }
    fn read(self: &File, save_to: &mut Vec<u8>) -> Result<usize, String> {
        let mut tmp = self.data.clone();
        let read_lenth = self.data.len();
        save_to.reserve(read_lenth);
        save_to.append(&mut tmp);
        Ok(read_lenth)
    }
}

fn open(f: File) -> Result<File, String> {
    if one_in(10_000) {
        let err_msg = String::from("permission denied");
        return Err(err_msg);
    }
    //
    Ok(f)
}
fn close(f: File) -> Result<File, String> {
    if one_in(100_000) {
        let err_msg = String::from("interrupted by signal");
        return Err(err_msg);
    }
    //
    Ok(f)
}

pub fn list3_8() {
    let f4_data: Vec<u8> = vec![114, 117, 115, 116, 33];
    let mut f4 = File::new_with_data("4.txt", &f4_data);

    let mut buffer: Vec<u8> = vec![];

    f4 = open(f4).unwrap();
    let f4_length = f4.read(&mut buffer).unwrap();
    f4 = close(f4).unwrap();

    let text = String::from_utf8_lossy(&buffer);

    println!("{:?}", f4);
    println!("{} is {} byte long", f4.name, f4_length);
    println!("{}", text);
}
