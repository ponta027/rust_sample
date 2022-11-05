use std::{thread, time};
fn main() {
    for n in 1..1001 {
        let mut handlers: Vec<thread::JoinHandle<()>> = Vec::with_capacity(n);
        let start = time::Instant::now();

        for _m in 0..n {
            let handler = thread::spawn(|| {
                /*
                let pause = time::Duration::from_millis(20);
                thread::sleep(pause);
                */
                let start = time::Instant::now();
                let pause = time::Duration::from_millis(20);
                while start.elapsed() < pause {
                    thread::yield_now();
                }
            });

            handlers.push(handler);
        }
        while let Some(handle) = handlers.pop() {
            handle.join();
        }
        let finish = time::Instant::now();
        println!("{} {:02?}", n, finish.duration_since(start));
    }
}
