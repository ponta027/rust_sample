#[macro_use]
extern crate crossbeam;
use crossbeam::channel::unbounded;
use std::thread;

use crate::ConnectivityCheck::*;

#[derive(Debug)]
enum ConnectivityCheck {
    Ping,
    Pong,
    Pang,
}

fn main() {
    println!("Hello, world!");

    let n_messages = 3;

    let (requests_tx, request_rx) = unbounded();

    let (response_tx, response_rx) = unbounded();

    thread::spawn(move || loop {
        match request_rx.recv().unwrap() {
            Pong => eprintln!("unexpected pong respose "),
            Ping => response_tx.send(Pong).unwrap(),
            Pang => return,
        }
    });

    for _ in 0..n_messages {
        requests_tx.send(Ping).unwrap();
    }
    requests_tx.send(Pang).unwrap();

    for _ in 0..n_messages {
        select! {
            recv( response_rx) -> msg => println!("{:?}",msg),
        }
    }
}
