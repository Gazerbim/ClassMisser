use std::net::{TcpListener, TcpStream};
use std::sync::mpsc;
use std::thread;
use std::time;

use crate::server::Server;

mod server;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:9001").unwrap();

    let (tx, rx) = mpsc::channel();

    let mut server = Server::start(rx);

    //thread qui cherche de nouvelles connections
    thread::spawn(move || {
        look_for_connections(&listener, &tx);
    });

    //thread principal du serveur
    server.run();
}

fn look_for_connections(listener: &TcpListener, tx: &std::sync::mpsc::Sender<TcpStream>) {
    loop {
        match listener.accept() {
            Ok((_socket, addr)) => {
                println!("new client: {addr:?}");
                tx.send(_socket).unwrap();
            }
            Err(e) => println!("couldn't get client: {e:?}"),
        }
        thread::sleep(time::Duration::from_millis(1));
    }
}
