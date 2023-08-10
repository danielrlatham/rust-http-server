use http::Method;
use http::Request;
use server::Server;

mod http;
mod server;

fn main() {
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run();
}

/*
* GET /user?id=10 HTTP/1.1\n
* HEADERS \n
* BODY
*/
