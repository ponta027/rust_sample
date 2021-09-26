extern crate redis;

use redis::Commands;
use std::collections::HashMap;
use std::env;

//
fn connection(address: &str) -> redis::RedisResult<redis::Connection> {
    let param = format!("redis://{}/", address);
    let client = redis::Client::open(param)?;
    client.get_connection()
}

fn get_data_by_key(con: &mut redis::Connection, key: &str) -> redis::RedisResult<isize> {
    con.get(key)
}
//
fn set_data_u32(con: &mut redis::Connection, key: &str, val: u32) {
    let _: () = con.set(key, val).unwrap();
}

//
fn set_map_data(con: &mut redis::Connection, key: &str, data: &[(&str, u32)]) {
    redis::cmd("HMSET").arg(key).arg(data).execute(con);
}
fn get_map_data(
    con: &mut redis::Connection,
    key: &str,
) -> redis::RedisResult<HashMap<String, u32>> {
    con.hgetall(key)
}

//
fn get_keys(con: &mut redis::Connection) -> redis::RedisResult<Vec<String>> {
    redis::cmd("KEYS").arg("my*").query(con)
}
//
fn clear_data(con: &mut redis::Connection) {
    let result = get_keys(con);
    match result {
        Ok(i) => {
            for key in &i {
                println!("result:{}", key);
                del_data(key, con);
            }
        }
        Err(err) => {
            println!("Error:{:?}", err);
        }
    }
}
//
fn del_data(key: &str, con: &mut redis::Connection) {
    redis::cmd("DEL").arg(key).execute(con);
}

fn main() {
    let param = env::var("REDIS_HOST").expect("VAR:REDIS_HOST is not defined");
    let mut con = connection(&param).unwrap();
    let _ = set_data_u32(&mut con, "my_key", 10);
    let result = get_data_by_key(&mut con, "my_key");
    match result {
        Ok(i) => {
            println!("result:{}", i);
        }
        Err(err) => {
            println!("Error:{:?}", err);
        }
    }
    let _ = set_map_data(&mut con, "my_keyX", &[("field_1", 10), ("field_2", 20)]);
    let result = get_map_data(&mut con, "my_keyX");
    match result {
        Ok(i) => {
            for (key, val) in &i {
                println!("result:{},{}", key, val);
            }
        }
        Err(err) => {
            println!("Error:{:?}", err);
        }
    }
    // let _ = set_data2(&mut con);
    let _ = set_map_data(&mut con, "my_keyX", &[("field_2", 22), ("field_3", 30)]);

    let result = get_map_data(&mut con, "my_keyX");
    match result {
        Ok(i) => {
            for (key, val) in &i {
                println!("result:{},{}", key, val);
            }
        }
        Err(err) => {
            println!("Error:{:?}", err);
        }
    }
    println!("[Clear Data]");
    clear_data(&mut con);
}
