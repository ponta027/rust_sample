const BIAS: i32 = 127;
const RADIX: f32 = 2.0;

fn to_parts(n: f32) -> (u32, u32, u32) {
    let bits = n.to_bits();
    println!(
        "{} is bits \n{:032b}\n0123456789abcdef0123456789abcdef",
        n, bits
    );
    let sign = (bits >> 31) & 1;
    let exponent = (bits >> 23) & 0xff;
    let fraction = bits & 0x7FFFFF;
    (sign, exponent, fraction)
}

fn decode(sign: u32, exponent: u32, frac: u32) -> (f32, f32, f32) {
    let signed_1 = (-1.0_f32).powf(sign as f32);
    let mut mantissa: f32 = 1.0;
    let exponent = (exponent as i32) - BIAS;
    let exponent = RADIX.powf(exponent as f32);

    for i in 0..23 {
        let mask = 1 << i;
        let one_at_bit_1 = frac & mask;
        if one_at_bit_1 != 0 {
            let i_ = i as f32;
            let weight = 2_f32.powf(i_ - 23.0);
            mantissa += weight;
        }
    }

    (signed_1, exponent, mantissa)
}

fn from_parts(sign: f32, exponent: f32, mantissa: f32) -> f32 {
    sign * exponent * mantissa
}
fn main() {
    let n: f32 = 42.42;
    let (sign, exp, frac) = to_parts(n);
    println!("{:b}\n{:b}\n{:b}", sign, exp, frac);
    println!("{}\n{}\n{}", sign, exp, frac);

    let (sign_, exp_, mant) = decode(sign, exp, frac);

    println!("{},{},{}", sign_, exp_, mant);

    let n_ = from_parts(sign_, exp_, mant);
    println!("{}", n_);

    //
    println!("************************************");
    let n: f32 = -42.42;
    let (sign, exp, frac) = to_parts(n);
    println!("{:b},{:b},{:b}", sign, exp, frac);

    let (sign_, exp_, mant) = decode(sign, exp, frac);

    println!("{},{},{}", sign_, exp_, mant);

    let n_ = from_parts(sign_, exp_, mant);
    println!("{}", n_);
}
