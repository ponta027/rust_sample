use rand::random;

static mut ERROR: isize = 0;
// no member struct
struct File;

#[allow(unused_variables)]
fn read(f: &File, save_to: &mut Vec<u8>) -> usize {
    if random() && random() && random() {
        unsafe {
            ERROR = 1;
        }
    }
    0
}

pub fn list3_7() {
    let mut f = File;
    let mut buffer = vec![];
    read(&f, &mut buffer);
    unsafe {
        if ERROR != 0 {
            panic!("an error has occcured!");
        }
    }
}
