mod server;

fn main() {
    let server = server::Server::new(9001);
    server.start_server();
}
