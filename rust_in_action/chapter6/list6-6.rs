/**
 * String:heap
 * str: stack
 */
//fn is_strong(password: String) -> bool {
// read-only access.
//fn is_strong<T: AsRef<str>>(password: T) -> bool {
// implicit conversion:Into.
//
// https://doc.rust-lang.org/std/string/struct.String.html#method.len
fn is_strong<T: Into<String>>(password: T) -> bool {
    //password.len() > 5
    println!("{}", type_of(&password));
    // move
    let tmp = password.into();
    println!("data:{} , type:{}", &tmp, type_of(&tmp));
    //    password.into().len() > 5
    tmp.len() > 5
}

fn type_of<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
    //
}
/*

fn is_strong2(password: &'static str) -> bool {
    password.into().len() > 5
}
*/
fn main() {
    // &str
    let pw = "justok";
    let is_strong = is_strong(pw);
    let pw2: String = pw.into();
    println!("{}", is_strong);
    //
    println!("data: {} , type: {}", pw, type_of(&pw));
    //
    println!("data: {} , type: {}", pw2, type_of(&pw2));

    // list6-6.rs
    println!("list 6-6");
    let a: i32 = 40;
    let b: Box<i32> = Box::new(a);
    println!("{} + {} = {} ", a, b, a + *b);
}
