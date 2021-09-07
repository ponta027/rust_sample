use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "example", about = "sample program")]
struct Cli {
    #[structopt(short, long)]
    client: bool,
}

//
fn handle_client(mut stream: TcpStream) {
    println!("handle ");
    let mut buf = [0; 128];
    let _result = stream.read(&mut buf);
    println!(" read data :{}", String::from_utf8(buf.to_vec()).unwrap());
}
fn tcp_server() -> std::io::Result<()> {
    //
    let port = "127.0.0.1:8080";
    let listener = TcpListener::bind(port)?;
    for stream in listener.incoming() {
        handle_client(stream?);
    }
    Ok(())
}

fn tcp_client() {
    let port = "127.0.0.1:8080";
    let mut stream = TcpStream::connect(port).unwrap();
    let buf = "Hello World";
    stream.write(buf.as_bytes()).unwrap();
}
//
fn main() {
    println!("Hello, world!");
    let args = Cli::from_args();

    match args.client {
        false => {
            tcp_server().unwrap();
        }
        true => {
            tcp_client();
        }
    }
}
