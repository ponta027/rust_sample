use std::convert::TryInto;

// https://doc.rust-jp.rs/rust-by-example-ja/conversion/try_from_try_into.html

fn main() {
    let a: i32 = 0;
    let b: u16 = 100;
    let _b = b.try_into().unwrap();
    if a < _b {
        println!(" 10 is less than 100");
    }

    /* */
    //    assert!(0.1 + 0.2 == 0.3);
    let abc: (f32, f32, f32) = (0.1, 0.2, 0.3);
    let xyz: (f64, f64, f64) = (0.1, 0.2, 0.3);
    println!("0.1+0.2:{:x}", (abc.0 + abc.1).to_bits());
    println!("0.3 :{:x}", (abc.2).to_bits());

    println!("xyz(f64)");
    println!("0.1+0.2:{:x}", (xyz.0 + xyz.1).to_bits());
    println!("0.3 :{:x}", (xyz.2).to_bits());
    assert!(abc.0 + abc.1 == abc.2);

    //    assert!(xyz.0 + xyz.1 == xyz.2);
}
