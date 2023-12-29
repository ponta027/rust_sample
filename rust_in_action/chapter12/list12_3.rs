use std::arch::asm;
/**
 * https://doc.rust-lang.org/beta/unstable-book/
 * https://dev-doc.rust-lang.org/beta/unstable-book/library-features/asm.html
 */
fn main() {
    println!("START");
    unsafe {
        asm!("svc 00000000");
    }
    println!("END");
}
