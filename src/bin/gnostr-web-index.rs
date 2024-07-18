use gnostr_web::paths::{Path, Paths};
use gnostr_web::request::Request;
use gnostr_web::response::Response;
use gnostr_web::server::run_server;
use gnostr_web::status::Status;

static BOOTSTRAP_CSS: &'static str = "
<link href=\"https://cdn.jsdelivr.net/npm/bootstrap@5.3.2/dist/css/bootstrap.min.css\" rel=\"stylesheet\" integrity=\"sha384-T3c6CoIi6uLrA9TneNEoa7RxnatzjcDSCmG1MXxSR1GAsXEV/Dwwykc2MPK8M2HN\" crossorigin=\"anonymous\">
";
static HOME_HTML: &'static str = "
<div class=\"container-fluid\">
<a class=\"navbar-brand\" href=\"#\">
<a href=\"/\">Home</a> <a href=\"/about\">About</a>
</a>
<div class=\"position-fixed mb-3 me-3 bd-mode-toggle\">
HOME CONTENT
</div>
</div>
";
static ABOUT_HTML: &'static str = "
<div class=\"container-fluid\">
<a class=\"navbar-brand\" href=\"#\">
<a href=\"/\">Home</a> <a href=\"/about\">About</a>
</a>
<div class=\"position-fixed mb-3 me-3 bd-mode-toggle\">
ABOUT CONTENT
</div>
</div>
";

fn home(_request: Request, mut response: Response) {
    response
        .html(
            Status::Ok,
            format!("{}{}", BOOTSTRAP_CSS.to_string(), HOME_HTML.to_string()),
        )
        .send();
}

fn about(_request: Request, mut response: Response) {
    response
        .html(
            Status::Ok,
            format!("{}{}", BOOTSTRAP_CSS.to_string(), ABOUT_HTML.to_string()),
        )
        .send();
}

fn main() {
    let paths: Paths = vec![Path::new("/", home), Path::new("/about", about)];

    run_server("0.0.0.0:8080", paths);
}
