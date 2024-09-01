use std::sync::atomic::AtomicU64;

use std::sync::atomic::Ordering::Relaxed;

fn main() {
    let result = get_x();
    println!("Hello World:{result}\n");
}

fn calculate_x() -> u64 {
    100
}

fn get_x() -> u64 {
    static X: AtomicU64 = AtomicU64::new(0);
    let mut x = X.load(Relaxed);
    if x == 0 {
        x = calculate_x();
        X.store(x, Relaxed);
    }
    x
}