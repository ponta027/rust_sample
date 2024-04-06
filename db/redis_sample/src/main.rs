extern crate redis;

use std::env;

mod redis_sample;

fn main() {
    let param = env::var("REDIS_HOST").expect("VAR:REDIS_HOST is not defined");
    let sample = redis_sample::RedisSample::new(&param);
    sample.execute();
}
