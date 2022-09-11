fn main() {
    let a: i64 = 42;
    let a_ptr = &a as *const i64;
    let a_ptr2 = &a;
    println!("a={}, a_ptr:{:p},a_ptr2:{:p}", a, a_ptr, a_ptr2);

    //
}
