use redis::Commands;
use redis::FromRedisValue;
use redis::RedisResult;
use redis::{RedisWrite, ToRedisArgs};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct RedisSample {
    //    pub connection: redis::Connection,
    addr: String,
}
const PRIMITIVE_KEY: &str = "my_key";
const COMPLEX_KEY: &str = "my_keyX";

#[derive(Serialize, Deserialize, Debug)]
struct Person {
    id: u32,
    name: String,
}
impl FromRedisValue for Person {
    fn from_redis_value(data: &redis::Value) -> RedisResult<Self> {
        let json = match data {
            redis::Value::Data(bytes) => std::string::String::from_utf8(bytes.to_vec())?,
            _ => "".to_string(),
        };
        let result: Person = serde_json::from_str(&json).unwrap();
        Ok(result)
    }
}

impl ToRedisArgs for Person {
    //
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        let content = serde_json::to_string(&self).unwrap();
        out.write_arg(content.as_bytes());
    }
}

impl RedisSample {
    pub fn new(address: &str) -> Self {
        Self {
            addr: String::from(address),
        }
    }
    fn connect(&self, address: &str) -> redis::RedisResult<redis::Connection> {
        let param = format!("redis://{}/", address);
        let client = redis::Client::open(param)?;
        client.get_connection()
    }

    pub fn execute(&self) {
        let mut conn = self.connect(&self.addr).unwrap();
        self.test_simple_set(&mut conn);
        self.test_map_set(&mut conn);
        self.test_map_set2(&mut conn);
        self.test_custom_data(&mut conn);
        self.test_delete(&mut conn);
    }
    pub fn test_delete(&self, con: &mut redis::Connection) {
        println!("## delete サンプル");
        //        let result = self.get_keys(con);
        let result: redis::RedisResult<Vec<String>> = redis::cmd("KEYS").arg("my*").query(con);
        let mut keys: Vec<String> = Vec::new();
        match result {
            Ok(i) => {
                for key in &i {
                    println!("result:{}", key);
                    keys.push(key.clone());
                    redis::cmd("DEL").arg(key).execute(con);
                }
            }
            Err(err) => {
                println!("Error:{:?}", err);
            }
        }
        for key in keys {
            let result: redis::RedisResult<u32> = con.get(&key);
            println!("key={},{:?}", key, result);
        }
    }
    pub fn test_custom_data(&self, con: &mut redis::Connection) {
        println!("## カスタムデータの登録");
        let key = "my_HOGE";
        let data = Person {
            id: 10,
            name: "aaaa".to_string(),
        };

        println!("set data key={},value:{:?}", key, data);
        redis::cmd("SET").arg(key).arg(&data).execute(con);
        let result: Person = con.get(key).unwrap();
        println!("get data :{:?}", result);
        if result.id != data.id {
            println!("NG:{}:{}", result.id, data.id);
        }
        if result.name != data.name {
            println!("NG:{}:{}", result.name, data.name);
        }
    }

    pub fn test_map_set(&self, con: &mut redis::Connection) {
        println!("## mapデータのサンプル");
        let test_data = [("field_1", 10), ("field_2", 20)];
        println!("set data key={},value:{:?}", COMPLEX_KEY, test_data);
        let _ = redis::cmd("HMSET")
            .arg(COMPLEX_KEY)
            .arg(&test_data)
            .execute(con);
        let result: redis::RedisResult<HashMap<String, u32>> = con.hgetall(&COMPLEX_KEY);
        println!("get data :{:?}", result);
        match result {
            Ok(i) => {
                for (key, val) in &i {
                    println!("key:{},value:{}", key, val);
                }
            }
            Err(err) => {
                println!("Error:{:?}", err);
            }
        }
    }
    pub fn test_map_set2(&self, con: &mut redis::Connection) {
        println!("## mapデータのサンプル2");
        let test_data = [("field_2", 22), ("field_3", 30)];
        println!("set data key={},value:{:?}", COMPLEX_KEY, test_data);
        let _ = redis::cmd("HMSET")
            .arg(COMPLEX_KEY)
            .arg(&test_data)
            .execute(con);
        let result: redis::RedisResult<HashMap<String, u32>> = con.hgetall(&COMPLEX_KEY);
        println!("get data :{:?}", result);
        match result {
            Ok(i) => {
                for (key, val) in &i {
                    println!("key:{},value:{}", key, val);
                }
            }
            Err(err) => {
                println!("Error:{:?}", err);
            }
        }
    }

    pub fn test_simple_set(&self, con: &mut redis::Connection) {
        println!("## set/get のサンプル ");
        let test_value = 12;
        let _ret: () = con.set(PRIMITIVE_KEY, test_value).unwrap();
        println!("set data key={},value:{:?}", PRIMITIVE_KEY, test_value);
        let result: redis::RedisResult<u32> = con.get(PRIMITIVE_KEY);
        println!("result:{:?}", result);
        match result {
            Ok(i) => {
                if i == test_value {
                    println!("OK:{}", i);
                } else {
                    println!("NG:{}:{}", i, test_value);
                }
            }
            Err(err) => {
                println!("Error:{:?}", err);
            }
        }
    }
}
