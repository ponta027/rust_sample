fn main() {
    let three = 0b11;
    let thiry = 0o36;
    let three_hundred = 0x12C;

    println!(" base 10:{} {} {}", three, thiry, three_hundred);
    println!(" base 2:{:b} {:b} {:b}", three, thiry, three_hundred);
    println!(" base 8:{:o} {:o} {:o}", three, thiry, three_hundred);
    println!(" base 16:{:x} {:x} {:x}", three, thiry, three_hundred);
}
