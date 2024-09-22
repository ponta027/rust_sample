use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::Relaxed;
use std::thread;
use std::time::Duration;

fn main() {
    let num_done = &AtomicUsize::new(0);

    thread::scope(|s| {
        for t in 0..4 {
            s.spawn(move || {
                for i in 0..25 {
                    progress_item(t * 25 + i);
                    num_done.fetch_add(1, Relaxed);
                }
                //
            });
        }

        loop {
            let n = num_done.load(Relaxed);
            if n == 100 {
                break;
            }
            println!("Working {n}/100 Done");
            thread::sleep(Duration::from_secs(1));
        }
    });
    println!("Done\n");
}
fn progress_item(i: u32) {
    println!("{i}");
    thread::sleep(Duration::from_secs(1));
}
