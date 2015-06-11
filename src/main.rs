use std::net::ToSocketAddrs;
use std::net::UdpSocket;
use std::sync::mpsc::{channel, Sender};
use std::thread;

fn handle_request<T:ToSocketAddrs>(src: T, size: usize, query: [u8; 512], tx: Sender<(T, usize, [u8; 512])>) {
    let client = UdpSocket::bind("0.0.0.0:0").unwrap();

    match client.send_to(&query[.. size], &"8.8.8.8:53") {
        Ok(_) => {
            let mut outbuf = [0; 512];
            let (amt, _) = client.recv_from(&mut outbuf).unwrap();
            match tx.send((src, amt, outbuf)) {
                Ok(_) => {},
                Err(e) => println!("Error responding {}", e)
            };
        }
        Err(e) => println!("Error querying DNS: {}", e)
    }
    drop(client);
}

fn main() {
    let server = UdpSocket::bind("0.0.0.0:8889").unwrap();
    let mut inbuf = [0; 512];
    let (tx, rx) = channel();

    loop {
        match server.recv_from(&mut inbuf) {
            Ok((amt, src)) => {
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
