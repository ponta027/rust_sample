use std::cell::UnsafeCell;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::{Acquire, Release};
use std::thread;
use std::time::Duration;

use std::ops::{Deref, DerefMut};

///  生存期間を省略できない
///  Guard型に対して、明示的にSend,Syncを実装しないと、
///  コンパイラがフィールドに基づき自動的に実装してくる。
///
pub struct Guard<'a, T> {
    lock: &'a SpinLock<T>,
}

impl<T> Deref for Guard<'_, T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe { &*self.lock.value.get() }
    }
}

impl<T> DerefMut for Guard<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.lock.value.get() }
    }
}

impl<T> Drop for Guard<'_, T> {
    fn drop(&mut self) {
        self.lock.locked.store(false, Release);
    }
}

unsafe impl<T> Sync for SpinLock<T> where T: Send {}
unsafe impl<T> Send for Guard<'_, T> where T: Send {}
unsafe impl<T> Sync for Guard<'_, T> where T: Sync {}
/////////////////////////////////////////////////
pub struct SpinLock<T> {
    locked: AtomicBool,
    value: UnsafeCell<T>,
}

impl<T> SpinLock<T> {
    pub const fn new(value: T) -> Self {
        Self {
            locked: AtomicBool::new(false),
            value: UnsafeCell::new(value),
        }
    }
    ///　返される参照の生存間が&selfと同じであることが明確になる
    pub fn lock(&self) -> Guard<T> {
        //pub fn lock<'a>(&'a self) -> &'a mut T {
        while self.locked.swap(true, Acquire) {
            //プロセッサに対してなにかが変わるのをスピンして待っていることを知らせる
            // プロセッサコアがこのような状況に合わせて最適化できる特殊な命令にコンパイルされ

            std::hint::spin_loop();
        }
        Guard { lock: self }
        //unsafe { &mut *self.value.get() }
    }
    pub fn unlock(&self) {
        self.locked.store(false, Release);
    }
}
fn main() {
    let x = SpinLock::new(Vec::new());
    thread::scope(|s| {
        s.spawn(|| {
            let mut g = x.lock();
            thread::sleep(Duration::from_millis(10000));
            g.push(1);
            //
        });
        s.spawn(|| {
            println!("try lock\n");
            let mut g = x.lock();
            println!("lock OK\n");
            g.push(2);
            println!("push 2\n");
            g.push(2);
            println!("push 2\n");
            //
        });
    });
    let g = x.lock();
    println!("Hello, world!{:?}", g.as_slice());
}
