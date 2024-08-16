use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::Relaxed;
use std::thread;
use std::time::Duration;

/**
 *
 */
fn main() {
    /** */
    static STOP: AtomicBool = AtomicBool::new(false);
    let background_thread = thread::spawn(|| {
        while !STOP.load(Relaxed) {
            some_work();
        }
    });

    for line in std::io::stdin().lines() {
        match line.unwrap().as_str() {
            "help" => println!("Commands:help , stop"),
            "stop" => break,
            cmd => println!("unknown command:{cmd:?}"),
        };
    }

    // バッググランドスレッドに停止命令
    STOP.store(true, Relaxed);
    // バッググランドスレッドが終了するまで待つ
    background_thread.join().unwrap();
    println!("Hello, world!");
}

/**
 *
 */
fn some_work() {
    thread::sleep(Duration::from_secs(1));
    dbg!("sleep");
}
