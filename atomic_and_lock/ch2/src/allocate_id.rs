use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering::Relaxed;
use std::thread;

use std::time::Duration;
fn main() {
    thread::scope(|s| {
        for t in 0..5 {
            /*
            let _ = thread::Builder::new()
                .name(format!("first{t}").to_string())
                .spawn_scoped(s, move || {
                    //            s.spawn(move || {
                    for _i in 0..1000 {
                        let id = allocate_new_id();
                        println!("issue ID= {id} at {t}");
                        thread::sleep(Duration::from_millis(10));
                    }
                });
            */
            s.spawn(move || {
                for _i in 0..1000 {
                    let id = allocate_new_id();
                    println!("issue ID= {id} at {t}");
                    thread::sleep(Duration::from_millis(10));
                }
            });
        }
    });
    println!("Done");
}

fn allocate_new_id() -> u32 {
    static NEXT_ID: AtomicU32 = AtomicU32::new(0);
    let id = NEXT_ID.fetch_add(1, Relaxed);
    assert!(id < 1000, "too many IDs");
    id
}
