use std::net::{TcpListener, TcpStream};
use std::sync::mpsc;
use std::thread;
use std::time;

use crate::server::Server;

mod server;

pub mod services;

fn main() {
    let server = server::Server::new(9001);
    server.start_server();
}
