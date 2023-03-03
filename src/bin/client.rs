use std::net::UdpSocket;
use std::{env};

const UDP_HEADER: usize = 8;
const IP_HEADER: usize = 20;
const MAX_DATA_LENGTH: usize = 32 * 1024 - UDP_HEADER - IP_HEADER;

fn main() -> std::io::Result<()> {
    {
        let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8080".to_string());
        let socket = UdpSocket::bind(&addr)?;
        println!("Binding to on: {}", socket.local_addr()?);
        // the message, it will be cut off.
        loop {
            socket.send_to(&[0; MAX_DATA_LENGTH], &addr).expect("couldn't send data");
        }
    } // the socket is closed here
    Ok(())
}
