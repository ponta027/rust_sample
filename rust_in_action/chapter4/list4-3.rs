#![allow(unused_variables)]
#![allow(dead_code)]

#[derive(Debug)]
enum StatusMessage {
    Ok,
}

fn cheeck_status(sat_id: u64) -> StatusMessage {
    StatusMessage::Ok
}

fn main() {
    // 衛生群を2回チェックするプログラム
    println!("Hello Worrld");
    let sat_a = 0;
    let sat_b = 1;
    let sat_c = 2;
    let a_status = cheeck_status(sat_a);
    let b_status = cheeck_status(sat_b);
    let c_status = cheeck_status(sat_c);

    println!("a: {:?} b:{:?} c:{:?}", a_status, b_status, c_status);
    let a_status = cheeck_status(sat_a);
    let b_status = cheeck_status(sat_b);
    let c_status = cheeck_status(sat_c);
    println!("a: {:?} b:{:?} c:{:?}", a_status, b_status, c_status);
}
