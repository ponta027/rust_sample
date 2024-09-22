use std::sync::atomic::AtomicBool;
//use std::sync::atomic::AtomicU64;
//use std::sync::atomic::Ordering::Relaxed;
use std::sync::atomic::Ordering::{Acquire, Release};
use std::thread;
use std::time::Duration;

//static DATA: AtomicU64 = AtomicU64::new(0);
static mut DATA: u64 = 0;
static READY: AtomicBool = AtomicBool::new(false);

/// Release/Acquireメモリオーダリング
/// 対で使用され、スレッド間の先行発生関係を形成する
/// Acquire:読み込み更新操作や比較操作に対して使用する場合、値をロードする部分 のみ適用される
/// Releas:操作のストア 部分のみに適用される
fn main() {
    for _ in 0..5 {
        thread::spawn(|| {
            //DATA.store(123, Relaxed);
            unsafe { DATA = 123 };
            thread::sleep(Duration::from_millis(1000));
            READY.store(true, Release);
        });
        while !READY.load(Acquire) {
            println!("waiting");
            thread::sleep(Duration::from_millis(100));
        }
        //println!("{}", DATA.load(Relaxed));
        println!("{}", unsafe { DATA });
    }
}
