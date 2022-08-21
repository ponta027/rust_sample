/*
fn add<T>(i: T, j: T) -> T {
    i + j
}
*/
fn add<T: std::ops::Add<Output = T>>(i: T, j: T) -> T {
    i + j
}

fn main() {
    //
    println!("{}", add(1.2, 3.4));
    println!("{}", add(10, 20));
}
