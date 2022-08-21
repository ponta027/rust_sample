// lifetime annotation
fn add_with_lifetimes<'a, 'b>(i: &'a i32, j: &'b i32) -> i32 {
    *i + *j
}
fn main() {
    let i = 10;
    let j = 20;
    let result = add_with_lifetimes(&i, &j);
    println!("result ={}", result);
    //
}
