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

fn cheeck_status(sat_id: CubeSat) -> StatusMessage {
    //imply drop(sat_id);
    StatusMessage::Ok
}

fn main() {
    // 衛生群を2回チェックするプログラム
    println!("Hello Worrld");
    // no implement copy trait
    let sat_a = CubeSat { id: 0 };
    let sat_b = CubeSat { id: 1 };
    let sat_c = CubeSat { id: 2 };

    let a_status = cheeck_status(sat_a);
    let b_status = cheeck_status(sat_b);
    let c_status = cheeck_status(sat_c);

    println!("a: {:?} b:{:?} c:{:?}", a_status, b_status, c_status);
    // move occurs
    let a_status = cheeck_status(sat_a);
    let b_status = cheeck_status(sat_b);
    let c_status = cheeck_status(sat_c);
    println!("a: {:?} b:{:?} c:{:?}", a_status, b_status, c_status);
}
