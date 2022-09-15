// implementation parity bit
//
fn parity_bit(bytes: &[u8]) -> u8 {
    //
    let mut n_one: u32 = 0;
    for byte in bytes {
        let ones = byte.count_ones();
        n_one += ones;
        println!("{} , {:08b} , {} ", byte, byte, ones);
    }
    ((n_one % 2) == 0) as u8
}
fn main() {
    //
    let abc = b"abc";
    println!("(input :{:?}", abc);
    println!("output :{:?}", parity_bit(abc));
    println!();

    let abcd = b"abcd";
    println!("input :{:?}", abcd);
    println!("output:{:?}", parity_bit(abcd));
    println!("Hello World");
}
