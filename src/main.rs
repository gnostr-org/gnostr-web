use tejmagar::paths::{Path, Paths};
use tejmagar::request::Request;
use tejmagar::response::Response;
use tejmagar::server::run_server;
use tejmagar::status::Status;

fn home(_request: Request, mut response: Response) {
    response.html(Status::Ok, "<a href=\"/\">Home</a> <a href=\"/about\">About</a>".to_string()).send();
}

fn about(_request: Request, mut response: Response) {
    response.html(Status::Ok, "<a href=\"/\">Home</a> <a href=\"/about\">About</a>".to_string()).send();
}

fn main() {
    let paths: Paths = vec![
        Path::new("/", home),
        Path::new("/about", about),
    ];

    run_server("0.0.0.0:8080", paths);
}
