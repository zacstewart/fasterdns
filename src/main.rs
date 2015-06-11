use std::net::SocketAddr;
use std::net::UdpSocket;
use std::sync::mpsc::{channel, Sender};
use std::thread;

const DNS_SERVERS: [&'static str; 4] = [
    "8.8.4.4:53",
    "8.8.8.8:53",
    "208.67.222.222:53",
    "208.67.220.220:53"
];

fn perform_dns_lookup(dns_server: &str, src: SocketAddr, size: usize, query: [u8; 512], tx: Sender<(SocketAddr, usize, [u8; 512])>) {
    let client = UdpSocket::bind("0.0.0.0:0").unwrap();
    match client.send_to(&query[.. size], dns_server) {
        Ok(_) => {
            let mut outbuf = [0; 512];
            let (amt, _) = client.recv_from(&mut outbuf).unwrap();

            println!("{} has the response", dns_server);

            match tx.send((src, amt, outbuf)) {
                Ok(_) => {},
                Err(_) => {}
            };
        }
        Err(e) => println!("Error querying DNS: {}", e)
    }
    drop(client);
}

fn handle_request(src: SocketAddr, size: usize, query: [u8; 512], respond: Sender<(SocketAddr, usize, [u8; 512])>) {
    let(tx, rx) = channel();
    for dns_server in DNS_SERVERS.iter() {
        let my_dns_server = dns_server.clone();
        let my_src = src.clone();
        let my_tx = tx.clone();
        thread::spawn(move || {
            perform_dns_lookup(my_dns_server, my_src, size, query, my_tx);
        });
    }

    respond.send(rx.recv().unwrap()).unwrap();
}

fn main() {
    let server = UdpSocket::bind("0.0.0.0:53").unwrap();
    let mut inbuf = [0; 512];
    let (tx, rx) = channel();

    loop {
        match server.recv_from(&mut inbuf) {
            Ok((amt, src)) => {
                println!("got request");
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
