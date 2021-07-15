pub fn do_result(){
    println!("[START]Result");
    result_sample();
    println!("\t unwrap");
    result_unwrap();
    println!("\t result_sample");
    match half_number( "100") {
        Ok(n) => println!("Ok:{}",n),
        Err(err) => println!("Erro:{:?} ",err),
    }
    match half_number( "xxx") {
        Ok(n) => println!("Ok:{}",n),
        Err(err) => println!("Erro:{:?} ",err),
    }

     match half_number_2( "100") {
        Ok(n) => println!("Ok:{}",n),
        Err(err) => println!("Erro:{:?} ",err),
    }
    match half_number_2( "xxx") {
        Ok(n) => println!("Ok:{}",n),
        Err(err) => println!("Erro:{:?} ",err),
    }


    match half_number_8_3_5 ( "100" ){
        Ok(n) => println!("Ok:{}",n),
        Err(err) => println!("Erro:{:?} ",err),
    }
    match half_number_8_3_5 ( "xxxx" ){
        Ok(n) => println!("Ok:{}",n),
        Err(err) => println!("Erro:{:?} ",err),
    }

    
    sample_8_3_6();


    println!("[END]Result");
}


use std::num::ParseIntError;
// Result Type alias
type Result<T> = std::result::Result<T , ParseIntError>;

fn half_number_2( s:&str) -> Result<i32>{
    match s.parse::<i32>(){
        Ok(n) => Ok(n/2),
        Err(err) => Err(err),
    }
}



//fn half_number(s:&str) -> Result<i32,ParseIntError>{
fn half_number(s:&str) -> Result<i32>{
   match s.parse::<i32>(){
    Ok(n) => Ok(n/2),
    Err(err)    => Err(err),
   }
}
//
fn result_unwrap(){
    println!("{}", "100".parse::<i32>().unwrap());
}
fn result_sample(){
    let r = "100".parse::<i32>();
    match r {
        Ok(n) => println!("n is {} ", n ),
        Err(e) => println!("error : {:?}",e)
    }
    let r = "a".parse::<i32>();
    match r {
        Ok(n) => println!("n is {} ", n ),
        Err(e) => println!("error : {:?}",e)
    }
}


/**
 * Parser Combinator 8.3.5
 */

fn half_number_8_3_5( s: &str ) -> Result<i32>{
    s.parse::<i32>().map(|n| n / 2)
}

fn half_number_8_3_6( s:&str) -> Result<i32>{
    let n = s.parse::<i32>()?;
    Ok(n/2)
}
fn sample_8_3_6(){
    println!("sample 8.3.6");
    match half_number_8_3_6("100"){
        Ok(n) => println!("Ok:{}", n ),
        Err(err) => println!("Err:{:?}",err),
    }
    match half_number_8_3_6("xxxx"){
        Ok(n) => println!("Ok:{}", n ),
        Err(err) => println!("Err:{:?}",err),
    }

}


//pub mod sample_lib;
