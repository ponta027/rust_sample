fn noop() {}
fn main() {
    println!("START");

    let fn_ptr = noop as usize;
    println!("noop as usize: 0x{:x}", fn_ptr);
    println!("END");
}
