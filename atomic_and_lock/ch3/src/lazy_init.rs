use std::sync::atomic::AtomicPtr;
use std::sync::atomic::Ordering::{Acquire, Release};
use std::thread;

#[derive(Debug)]
struct Data {
    data: u32,
}

fn generate_data() -> Data {
    Data { data: 10 }
}

fn get_data() -> &'static Data {
    static PTR: AtomicPtr<Data> = AtomicPtr::new(std::ptr::null_mut());
    let mut p = PTR.load(Acquire);
    if p.is_null() {
        p = Box::into_raw(Box::new(generate_data()));
        if let Err(e) = PTR.compare_exchange(std::ptr::null_mut(), p, Release, Acquire) {
            drop(unsafe { Box::from_raw(p) });
            p = e;
        }
    }
    unsafe { &*p }
    //
}
fn main() {
    for _ in 0..10 {
        thread::spawn(|| {
            let a = get_data();
            println!("{:?}", a.data);
        });
    }
}
