use std::net::UdpSocket;
use std::time::Instant;
use std::time::Duration;

fn main() {
    let socket = UdpSocket::bind("0.0.0.0:50505").expect("the address is not bind");
    let mut counter = 0;
    let mut last_report  = Instant::now();
    loop {
        let mut buf = [0u8; 2048];
        let (number_of_bytes, src_addr) = socket.recv_from(&mut buf).expect("recv_from failed");
        let filled_buf = &buf[..number_of_bytes];
        
        counter += 1;
        if last_report.elapsed() >= Duration::from_secs(1) {
            println!("{}", counter);
            counter = 0;
            last_report = Instant::now();
        }
    }
}
