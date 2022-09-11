static GLOBAL: i32 = 1000;

fn noop() -> *const i32 {
    let noop_local = 12345;
    println!("noop_local : {:016p}", &noop_local);
    &noop_local as *const i32
}
fn main() {
    let local_str = "a";
    let local_int = 123;
    let box_str = Box::new('b');
    let box_int = Box::new(123);
    let fn_int = noop();
    println!("GLOBAL\t{:016p}", &GLOBAL as *const i32);
    println!("local_str\t{:016p}", local_str as *const str);
    println!("local_int\t {:016p}", &local_int as *const i32);
    println!("boxed_int\t{:016p} ", box_int);
    println!("boxed str\t{:016p} ", box_str);
    println!("fn_int\t{:016p}", fn_int);
    //
}
