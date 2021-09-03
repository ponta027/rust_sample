use std::net::UdpSocket;


fn udp_sample() -> std::io::Result<()> {
    println!("upd sample");
    println!("open port :{}", 34254);
    let socket = UdpSocket::bind("127.0.0.1:34254")?;
    let mut buf = [0; 1024];

    let (amt, src) = socket.recv_from(&mut buf)?;
    let recv_data = String::from_utf8( buf.to_vec() );
    println!("size : {}, recv data:{}",amt, recv_data.unwrap());
    Ok(())
}

fn main() {
    udp_sample().unwrap();
}
