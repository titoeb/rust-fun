mod http;
mod server;
use http::Request;
use server::Server;

fn main() {
    // Initiate http server.
    let server = Server::new("127.0.0.1:7777".to_string());
    server.run();
}
