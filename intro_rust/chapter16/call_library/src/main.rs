#[link(name = "hello", kind = "static")]
use std::os::raw::c_char;
use std::ffi::CString;


extern "C" {
    fn puts(s: *const c_char);
    fn strlen(s: *const c_char) -> u32;
}
extern "C" {
    fn hello();
    fn c_add( a:i32, b :i32) -> i32;
}
fn main() {
    println!("Hello, world!");
    unsafe {
        hello();
    };
    let ans = unsafe{
        c_add(10,20)
    };
    println!("10 + 20 ={}",ans);
    //
    //
    /* */
    let s = "puts hello rust world.";
    let s_null_terminates = CString::new(s).unwrap();
    unsafe {
        puts( s_null_terminates.as_ptr());
    };

    let n = unsafe {
        strlen( s_null_terminates.as_ptr())
    };
    println!("s.len is {}", n );

}
