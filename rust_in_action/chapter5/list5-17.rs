fn mock_rand(n: u8) -> f32 {
    let base: u32 = 0b0_01111110_00000000000000000000000;
    let large_n = (n as u32) << 15;
    let f32_bits = base | large_n;
    /*
    println!("{:032b}", base);
    println!("{:032b}", large_n);
    println!("{:032b}", f32_bits);
    */
    let m = f32::from_bits(f32_bits);
    /*
    println!("{}", m);
    */

    2.0 * (m - 0.5)
}
fn main() {
    println!("HOGE");
    println!("max : {:08b}->{:?}", 0xff, mock_rand(0xff));
    println!("mean: {:08b}->{:?}", 0x77, mock_rand(0x77));
    println!("min : {:08b}->{:?}", 0x00, mock_rand(0x00));
}
