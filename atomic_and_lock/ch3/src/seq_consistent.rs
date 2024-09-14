use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::SeqCst;
use std::thread;
static A: AtomicBool = AtomicBool::new(false);
static B: AtomicBool = AtomicBool::new(false);

static mut S: String = String::new();

/// 最も強いメモリオーダリングがSequentially Consistent( 逐次整合）
/// ロードに対するAcquireオーダリングとすっとあに対するReleaseオーダリングによる保証を
/// すべて含み、さらにグローバルに一貫した操作順序を保証する
/// プログラム中に存在するSeqCstオーダリングのすべての操作はスレッドが合意する
/// 単一の全順序を構成する。この全順序は、個々の変数の全変更順序と整合している。
fn main() {
    for _ in 0..10 {
        let a = thread::spawn(|| {
            A.store(true, SeqCst);
            if !B.load(SeqCst) {
                unsafe { S.push('a') };
            }
            //
        });
        let b = thread::spawn(|| {
            B.store(true, SeqCst);
            if !A.load(SeqCst) {
                unsafe { S.push('b') };
            }
            //
        });

        a.join().unwrap();
        b.join().unwrap();

        let s = unsafe { &S };
        println!("{:?}", s);
    }
}
