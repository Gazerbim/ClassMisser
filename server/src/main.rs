mod server;

pub mod services;

fn main() {
    let mut server = server::Server::new(9001);
    server.start_server().unwrap();
}
