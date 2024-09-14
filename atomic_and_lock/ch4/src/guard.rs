use std::cell::UnsafeCell;
use std::ops::Deref;
use std::ops::DerefMut;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::Acquire;
use std::sync::atomic::Ordering::Release;
use std::thread;
use std::time::Duration;

pub struct SpinLock<T> {
    locked: AtomicBool,
    value: UnsafeCell<T>,
}

unsafe impl<T> Sync for SpinLock<T> where T: Send {}
impl<T> SpinLock<T> {
    pub const fn new(value: T) -> Self {
        Self {
            locked: AtomicBool::new(false),
            value: UnsafeCell::new(value),
        }
    }
    pub fn lock(&self) -> Guard<T> {
        while self.locked.swap(true, Acquire) {
            std::hint::spin_loop();
        }
        //        unsafe { &mut *self.value.get() }
        Guard { lock: self }
    }
    pub fn unlock(&self) {
        self.locked.store(false, Release);
    }
}

pub struct Guard<'a, T> {
    lock: &'a SpinLock<T>,
}

/*
unsafe impl<T> Sync for Guard<'_, T> where T: Send {}
unsafe impl<T> Send for Guard<'_, T> where T: Sync {}
*/

impl<T> Drop for Guard<'_, T> {
    fn drop(&mut self) {
        self.lock.locked.store(false, Release);
    }
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
fn main() {
    println!("Hello World\n");
    let x = SpinLock::new(Vec::new());
    thread::scope(|s| {
        s.spawn(|| {
            println!("thread create\n");
            let mut vec = x.lock();
            vec.push(1);
            thread::sleep(Duration::from_millis(10000));
            vec.push(4);
            println!("thread finished");
        });
        s.spawn(|| {
            println!("thread create\n");
            println!("challenge");
            let mut vec = x.lock();
            println!("locked");
            vec.push(2);
            x.unlock();
            x.lock().push(3);
        });
    });
    let g = x.lock();
    println!("Hello, world!{:?}", g.as_slice());
}
