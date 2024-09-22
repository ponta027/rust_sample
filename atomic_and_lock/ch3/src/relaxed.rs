use std::sync::atomic::AtomicI32;
use std::sync::atomic::Ordering::Relaxed;
use std::thread;

static X: AtomicI32 = AtomicI32::new(0);

///Relaxed メモリオーダリング
///先行発生関係はつくらないが、個々のアトミック変数にトウする「全変更順序」を保証
fn a() {
    X.fetch_add(5, Relaxed);
    X.fetch_add(10, Relaxed);
}

fn b() {
    let a = X.load(Relaxed);
    let b = X.load(Relaxed);
    let c = X.load(Relaxed);
    let d = X.load(Relaxed);
    println!("{a},{b},{c},{d}");
}
fn main() {
    X.store(1, Relaxed);

    for _t in 0..100 {
        let __t = thread::spawn(a);
        b();
    }
}
