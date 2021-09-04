use std::net::UdpSocket;
use std::io;

use structopt::StructOpt;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Debug, StructOpt)]
#[structopt(name = "example", about = "sample program")]
struct Cli {
    /// The pattern to look for
    #[structopt(short, long)\
    client: bool,
}

fn udp_sample() -> std::io::Result<()> {
    println!("upd sample");
    println!("open port :{}", 34254);
    let socket = UdpSocket::bind("127.0.0.1:34254")?;
    let mut buf = [0; 1024];

    match socket.recv_from(&mut buf) {
        Ok((amt, src)) => {
            let recv_data = String::from_utf8(buf.to_vec());
            println!("size : {}, recv data:{}", amt, recv_data.unwrap());
            socket.send_to(&buf, src)?;
        }
        Err(e) => {
            eprintln!("recv error{}", e);
        }
    }
    Ok(())
}

fn udp_client() {
    println!("UDP Client");
    let socket = UdpSocket::bind("127.0.0.1:34255").expect("fail to bind");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    socket
        .send_to(input.as_bytes(), "127.0.0.1:34254")
        .expect("couldn't send data ");
}

fn main() {
    let args = Cli::from_args();
    match args.client {
        false => {
            udp_sample().unwrap();
        }
        true => {
            udp_client();
        }
    }
}
