use std::{thread, time};

fn main() {
    let pause = time::Duration::from_millis(300);
    let start = time::Instant::now();
    let handler_1 = thread::spawn(move || {
        thread::sleep(pause);
    });
    let handler_2 = thread::spawn(move || {
        thread::sleep(pause);
    });
    handler_1.join().unwrap();
    handler_2.join().unwrap();
    let finish = time::Instant::now();
    println!("END {:02?}", finish.duration_since(start));
}
