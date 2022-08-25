#[derive(Debug)]
struct Demo {
    a: i32,
}

fn use_value(_val: Demo) {}
fn main() {
    let demo = Demo { a: 10 };

    use_value(demo);
    // error, move occurs.
    println!("{}", demo.a);
}
