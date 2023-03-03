use std::collections::VecDeque;
use std::net::UdpSocket;
use std::{env};
use std::io::{self, Write};
use std::time::{Duration, Instant};
use std::{thread, time};

// const UDP_HEADER: usize = 8;
// const IP_HEADER: usize = 20;
const MAX_DATA_LENGTH: usize = 64 * 1024;
const MAX_BYTES: usize = 1024 * 1024 * 2;
const MAX_ITERS: usize = 2000;
const N_MEAN: usize = 20;

fn main() -> std::io::Result<()> {
    let addr = env::args()
    .nth(1)
    .unwrap_or_else(|| "127.0.0.1:8080".to_string());
    let socket = UdpSocket::bind(addr)?;
    socket.set_nonblocking(true).expect("Call to set_nonblocking failed");
    println!("Listening on: {}", socket.local_addr()?);
    // Receives a single datagram message on the socket. If `buf` is too small to hold
    // the message, it will be cut off.
    let mut mean_array = VecDeque::with_capacity(N_MEAN);
    let mut buf = [0; MAX_DATA_LENGTH];
    'Big: loop {
        let mut bytes_read: usize = 0;
        let mut n_reads: usize = 0;
        let duration: Duration;
        let start = Instant::now();
        loop {
            match socket.recv_from(&mut buf) {
                Ok((amount, _)) => {
                    bytes_read += amount;
                }
                Err(e) => {
                    if e.kind() != std::io::ErrorKind::WouldBlock {
                        println!("Error: {}", e);
                        break 'Big;
                    }
                    thread::sleep(time::Duration::from_micros(100));
                }
            }
            n_reads += 1;
            if bytes_read > MAX_BYTES || n_reads > MAX_ITERS{
                duration = start.elapsed();
                break;
            }
            // println!("Time elapsed in recv is: {:?}", total_time);
        }
        let mut rate: f64 = 0.0;
        let total_time = duration.as_secs_f64();
        if total_time > 0.0 {
            rate = bytes_read as f64 / total_time / 1000.0 / 1000.0 * 8.0;
        }
        mean_array.push_back(rate);
        if mean_array.len() > N_MEAN {
            mean_array.pop_front();
        }
        let mean = mean_array.iter().sum::<f64>() / mean_array.len() as f64;
        print!("\r");
        print!("Rate: {rate:>7.3} Mbps.    Mean: {mean:>7.3} Mbps.");
        io::stdout().flush().unwrap();
    }

    //// Redeclare `buf` as slice of the received data and send reverse data back to origin.
    //let buf = &mut buf[..amt];
    //buf.reverse();
    //socket.send_to(buf, &src)?;
    Ok(())
}
