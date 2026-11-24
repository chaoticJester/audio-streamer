use std::net::UdpSocket;

fn main() {
    let socket = UdpSocket::bind("0.0.0.0:50505").expect("the address is not bind");
    loop {
        let mut buf = [0u8; 2048];
        let (number_of_bytes, src_addr) = socket.recv_from(&mut buf).expect("recv_from failed");
        let filled_buf = &buf[..number_of_bytes];
        println!("Number of bytes = {}", filled_buf.len());
        println!("{} {}", filled_buf[0], filled_buf[1]);
        println!("{} is the sender", src_addr);
    }
}
