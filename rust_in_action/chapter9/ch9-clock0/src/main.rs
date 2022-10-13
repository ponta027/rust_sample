use chrono::Local;

fn main() {
    println!("Hello, world!");
    let now = Local::now();
    println!("{:?}", now);
}
