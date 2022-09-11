fn main() {
    println!("======================");
    let mut n_nnonzero = 0;
    for i in 0..10000 {
        let ptr = i as *const u8;
        let byte_at_addr = unsafe { *ptr };
        if byte_at_addr != 0 {
            n_nnonzero += 1;
        }
        println!("{} count", i);
    }
    println!("non-zero bytes in memory : {}", n_nnonzero);
}
