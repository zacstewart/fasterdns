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

fn perform_dns_lookup(socket: UdpSocket, dns_server: &str, src: SocketAddr, size: usize, query: [u8; 512], tx: Sender<(SocketAddr, usize, [u8; 512])>) {
    match socket.send_to(&query[.. size], dns_server) {
        Ok(_) => {
            let mut outbuf = [0; 512];
            let (amt, _) = socket.recv_from(&mut outbuf).unwrap();

            match tx.send((src, amt, outbuf)) {
                Ok(_) => println!("{} has the response", dns_server),
                Err(e) => println!("failed to report: {}", e)
            };
        }
        Err(e) => println!("Error querying DNS: {}", e)
    }
}

fn handle_request(src: SocketAddr, size: usize, query: [u8; 512], respond: Sender<(SocketAddr, usize, [u8; 512])>) {
    let(tx, rx) = channel();
    let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
    for dns_server in DNS_SERVERS.iter() {
        let my_socket = socket.try_clone().unwrap();
        let my_dns_server = dns_server.clone();
        let my_src = src.clone();
        let my_tx = tx.clone();
        thread::spawn(move || {
            perform_dns_lookup(my_socket, my_dns_server, my_src, size, query, my_tx);
        });
    }
    match rx.recv() {
        Ok(msg) => {
            drop(socket);
            respond.send(msg).unwrap();
        }
        Err(e) => panic!("Failed receiving: {}", e)
    }
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
