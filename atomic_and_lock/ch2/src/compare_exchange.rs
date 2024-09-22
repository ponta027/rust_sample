use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering::Relaxed;
use std::thread;

use std::time::Duration;
fn main() {
    thread::scope(|s| {
        for t in 0..5 {
            let _ = thread::Builder::new()
                .name(format!("first{t}").to_string())
                .spawn_scoped(s, move || {
                    for _i in 0..100 {
                        let id = allocate_new_id();
                        println!("issue ID= {id} at {t}");
                        thread::sleep(Duration::from_millis(100));
                    }
                });
        }
    });
    println!("Done");
}

fn increment(a: &AtomicU32) {
    let mut current = a.load(Relaxed);
    loop {
        let new = current + 1;
        match a.compare_exchange(current, new, Relaxed, Relaxed) {
            Ok(_) => return,
            Err(v) => current = v,
        }
    }
}
fn allocate_new_id() -> u32 {
    static NEXT_ID: AtomicU32 = AtomicU32::new(0);
    increment(&NEXT_ID);
    NEXT_ID.load(Relaxed)
}
