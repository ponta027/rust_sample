
pub fn sample(){
    println!("[START]Result2");
    sample_8_3_7();
    sample_8_3_8();
    println!("[END]Result2");
}

fn sample_8_3_7(){
    println!("sammple 8.3.7");
    match half_number_8_3_7 ("xxxx"){
            Ok(n) =>  println!("OK:{}",n),
            Err(err) =>  println!("Error:{:?}",err),
    }
}

fn half_number_8_3_7( s:&str) -> Result<i32, &str>{
    match s.parse::<i32>(){
        Ok(n) => Ok(n/2 ),
        Err(err) => Err( "実行時エラー：これは数値ではありません"),
    }
}

fn sample_8_3_8(){
    println!("sammple 8.3.8");
    let n = "100".parse::<i32>().expect("これは数値ではありません");
    println!("n is {}", n );
    /*
    let n = "xxx".parse::<i32>().expect("これは数値ではありません");
    println!("n is {}", n );
    */
    

}

