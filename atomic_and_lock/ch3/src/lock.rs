//use core::time::Duration;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::{Acquire, Relaxed, Release};
use std::thread;

static mut DATA: String = String::new();
static LOCKED: AtomicBool = AtomicBool::new(false);

/// MutexはRelease/Acquireオーダリングの一般的な使用例
/// ロック時には
/// アンロックされているかどうかAcquireオーダリングのアトミック操作でチェック
/// 同時にアトミックに状態をロックに変更する
/// アンロックする際にはReleaseオーダリングを用いて状態を「アンロック」に戻す
///
fn f(i: u32) {
    if LOCKED
        .compare_exchange(false, true, Acquire, Relaxed)
        .is_ok()
    {
        unsafe { DATA.push(char::from_u32(i).unwrap()) };
        LOCKED.store(false, Release);
    }
    //
}
fn main() {
    thread::scope(|s| {
        let mut handles = Vec::new();
        for i in 0..100 {
            let t = s.spawn(move || f(i));
            handles.push(t);
        }
        for handle in handles {
            let _ = handle.join();
        }
    });

    let data = unsafe { &DATA };
    println!("{}", data);
}
