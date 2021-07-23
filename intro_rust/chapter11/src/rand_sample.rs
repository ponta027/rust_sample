use rand::prelude::*;

pub fn sample(){
    println!("[START] rand sample");
    let x :i32 = random();
    println!("x is {}", x );
    let mut rng = thread_rng();
    let y :i32=rng.gen_range(0,10);

    println!("y is {}", y );
    println!("[END] rand sample");
}
