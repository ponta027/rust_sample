use std::sync::atomic::fence;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::Acquire;
use std::sync::atomic::Ordering::Relaxed;
use std::sync::atomic::Ordering::Release;
use std::thread;
use std::time::Duration;

/// アトミック変数に対する操作の他にもメモリオーダリングを適用できるものがfence
/// アトミックフェンスを用いるとメモリオーダリングをアトミック操作から切り離す
/// ことができる

static mut DATA: [u64; 10] = [0; 10];
const ATOMIC_FALSE: AtomicBool = AtomicBool::new(false);
static READY: [AtomicBool; 10] = [ATOMIC_FALSE; 10];

fn some_calculate(i: usize) -> u64 {
    i.try_into().unwrap()
}
fn main() {
    for i in 0..10 {
        thread::spawn(move || {
            let data = some_calculate(i);
            unsafe { DATA[i] = data };
            /*
            if i == 5 {
                thread::sleep(Duration::from_millis(100));
            }
            */
            READY[i].store(true, Release);
        });
        //
    }

    thread::sleep(Duration::from_millis(500));
    let ready: [bool; 10] = std::array::from_fn(|i| READY[i].load(Relaxed));
    if ready.contains(&true) {
        fence(Acquire);
        for i in 0..10 {
            if ready[i] {
                println!("data{i} = {}", unsafe { DATA[i] });
            }
        }
    }

    println!("Hello World\n");
}
