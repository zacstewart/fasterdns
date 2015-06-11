use std::net::SocketAddr;
use std::net::UdpSocket;
use std::sync::mpsc::{channel, Sender};
use std::thread;

const DNS_SERVERS: [&'static str; 4] = [
    "8.8.8.8:53",
    "8.8.4.4:53",
    "208.67.222.222:53",
    "208.67.220.220:53"
];

fn handle_request(src: SocketAddr, size: usize, query: [u8; 512], respond: Sender<(SocketAddr, usize, [u8; 512])>) {
    let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
    for dns_server in DNS_SERVERS.iter() {
        let my_socket = socket.try_clone().unwrap();
        let my_dns_server = dns_server.clone();
        thread::spawn(move || {
            my_socket.send_to(&query[.. size], my_dns_server).unwrap();
        });
    }

    let mut outbuf = [0; 512];
    let (amt, rmt) = socket.recv_from(&mut outbuf).unwrap();
    println!("response by {}", rmt);
    drop(socket);
    respond.send((src, amt, outbuf)).unwrap();
}

fn main() {
    let server = UdpSocket::bind("0.0.0.0:53").unwrap();
    let mut inbuf = [0; 512];
    let (tx, rx) = channel();

    loop {
        match server.recv_from(&mut inbuf) {
            Ok((amt, src)) => {
                println!("\ngot request");
                let my_tx = tx.clone();
                thread::spawn(move || {
                    handle_request(src, amt, inbuf, my_tx);
                });
            }
            Err(e) => println!("Error receiving: {}", e)
        }

        let (src, size, outbuf) = rx.recv().unwrap();
        match server.send_to(&outbuf[.. size], &src) {
            Ok(_) => {},
            Err(e) => println!("Error sending: {}", e)
        }
    }
}
