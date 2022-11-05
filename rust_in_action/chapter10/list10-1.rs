fn add(a: i32, b: i32) -> i32 {
    //
    a + b
}
fn main() {
    println!("HelloWorld");
    let lamda_add = |a, b| a + b;
    assert_eq!(add(1, 2), lamda_add(1, 2));
    println!("END");
}
