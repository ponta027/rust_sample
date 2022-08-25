#![allow(unused_variables)]
#![allow(dead_code)]
#[derive(Debug)]
struct CubeSat {
    id: u64,
}

#[derive(Debug)]
enum StatusMessage {
    Ok,
}
fn cheeck_status(sat_id: CubeSat) -> CubeSat {
    sat_id
}

fn main() {
    // 衛生群を2回チェックするプログラム
    println!("Hello Worrld");
    let sat_a = CubeSat { id: 0 };
    let sat_b = CubeSat { id: 1 };
    let sat_c = CubeSat { id: 2 };

    let sat_a = cheeck_status(sat_a);
    let sat_b = cheeck_status(sat_b);
    let sat_c = cheeck_status(sat_c);

    println!("a: {:?} b:{:?} c:{:?}", sat_a, sat_b, sat_c);
    let sat_a = cheeck_status(sat_a);
    let sat_b = cheeck_status(sat_b);
    let sat_c = cheeck_status(sat_c);
    println!("a: {:?} b:{:?} c:{:?}", sat_a, sat_b, sat_c);
}
