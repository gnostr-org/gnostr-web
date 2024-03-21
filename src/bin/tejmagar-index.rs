use tejmagar::paths::{Path, Paths};
use tejmagar::request::Request;
use tejmagar::response::Response;
use tejmagar::server::run_server;
use tejmagar::status::Status;


static HOME_HTML: &'static str = "

<link href=\"https://cdn.jsdelivr.net/npm/bootstrap@5.3.2/dist/css/bootstrap.min.css\" rel=\"stylesheet\" integrity=\"sha384-T3c6CoIi6uLrA9TneNEoa7RxnatzjcDSCmG1MXxSR1GAsXEV/Dwwykc2MPK8M2HN\" crossorigin=\"anonymous\">

<a href=\"/\">HOME_HTML Home</a> <a href=\"/about\">About</a>


";

fn home(_request: Request, mut response: Response) {
    response
        .html(
            Status::Ok,
            HOME_HTML.to_string(),
        )
        .send();
}

fn about(_request: Request, mut response: Response) {
    response
        .html(
            Status::Ok,
            "<a href=\"/\">Home</a> <a href=\"/about\">About</a>".to_string(),
        )
        .send();
}

fn main() {
    let paths: Paths = vec![Path::new("/", home), Path::new("/about", about)];

    run_server("0.0.0.0:8080", paths);
}
