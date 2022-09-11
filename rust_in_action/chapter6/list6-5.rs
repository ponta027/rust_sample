use std::mem::size_of;
fn main() {
    let a: i64 = 42;
    let a_ptr = &a as *const i64;

    let a_addr: usize = unsafe { std::mem::transmute(a_ptr) };

    println!("a:{},({:p},{:x})", a, a_ptr, a_addr + 7);
    //
    {
        let ptr = 42 as *const Vec<String>;

        println!("size:{:?} byte", size_of::<Vec<String>>());
        unsafe {
            //let new_addr = ptr.offset(4);
            for n in 0..10 {
                let new_addr = ptr.offset(n);
                println!("{:p}->  {}:{:p} ", ptr, n, new_addr,);
            }
        }
        //
    }
}
