extern crate redis;

use redis::Commands;
use std::collections::BTreeMap;

fn get_data(con: &mut redis::Connection) -> redis::RedisResult<isize> {
    let _: () = con.set("my_key", 42)?;
    con.get("my_key")
}
fn connection(address: &str) -> redis::RedisResult<redis::Connection> {
    let param = format!("redis://{}/", address);
    let client = redis::Client::open(param)?;
    client.get_connection()
}

fn main() {
    let mut con = connection("192.168.0.12").unwrap();
    let result = get_data(&mut con);
    match result {
        Ok(i) => {
            println!("result:{}", i);
        }
        Err(_) => {}
    }
}
