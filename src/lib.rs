pub mod headers;
pub mod parser;
pub mod request;
pub mod response;
pub mod status;

pub mod paths {
    use crate::request::Request;
    use crate::response::Response;

    pub type Paths = Vec<Path<fn(Request, Response)>>;
    pub type SinglePath = Path<fn(Request, Response)>;

    /// Path accepts pathname and view
    pub struct Path<T> {
        pub name: String,
        pub view: T,
    }

    impl<T> Path<T> {
        pub fn new(name: &str, view: T) -> Self {
            let name = name.to_string();

            return Self { name, view };
        }
    }
}

pub mod server {
    use crate::headers::{extract_headers, parse_request_method_header};
    use crate::paths::{Paths, SinglePath};
    use crate::request::Request;
    use crate::response::Response;
    use std::net::{Shutdown, TcpListener, TcpStream, Ipv4Addr, UdpSocket};
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::{Arc, RwLock};
    use std::thread::spawn;

    /// Example usage
    /// ```rust
    /// use gnostr_web::paths::{Path, Paths};
    /// use gnostr_web::request::Request;
    /// use gnostr_web::response::Response;
    /// use gnostr_web::server::run_server;
    /// use gnostr_web::status::Status;
    ///
    /// fn home(request: Request, mut response: Response) {
    ///    response.html(Status::Ok, "Home Page".to_string()).send();
    /// }
    ///
    /// fn main() {
    ///    let paths: Paths = vec![
    ///         Path::new("/", home),
    ///    ];
    ///
    ///    run_server("0.0.0.0:8080", paths);
    /// }
    /// ```


    //use std::net::{Ipv4Addr, TcpListener, UdpSocket};
    use std::str::FromStr;
    pub type Port = u16;


    pub fn is_tcp_port_available(host: &str, p: Port) -> bool {
        matches!(
            TcpListener::bind((Ipv4Addr::from_str(host).unwrap(), p)).is_ok(),
            true
        ) 
    }

    pub fn is_udp_port_available(host: &str, p: Port) -> bool {
        matches!(
            UdpSocket::bind((Ipv4Addr::from_str(host).unwrap(), p)).is_ok(),
            true
        )
    }

    pub fn check_port(host: &str, port: Port) -> bool {
        is_tcp_port_available(host, port) && is_udp_port_available(host, port)
    }


    #[cfg(test)]
    mod tests {
        use check_port;
        #[test]
        fn test_is_free() {
            assert!(check_port("127.0.0.1", 32200));
        }
    }


    pub fn get_available_port() -> Option<u16> {
        (8000..9000).find(|port| port_is_available(*port))
    }

    pub fn port_is_available(port: u16) -> bool {
        match TcpListener::bind(("0.0.0.0", port)) {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    pub fn run_server(listen_address: &str, paths: Paths) {
        println!("\nhttp://{}", listen_address);

        let v: Vec<&str> = listen_address.split(":").collect();
        print!("\nv[0]={:?}", v[0]);
        print!("\nv[1]={:?}", v[1]);

        // use std::ops::Deref;
        // let port: &u16 = v[1] as u16;
        //let port: &u16 = &(v[1] as u16);
        //if port_is_available(port.deref(v[1]) as u16) {}
        if port_is_available(8080 as u16) {
            print!("\n8080 port_is_available");
            //std::process::exit(0);
        } else {
            print!("\nNOT!!! 8080 port_is_available");
            std::process::exit(0);
        }
        // //gnostr-hyper
        // if port_is_available(8081 as u16) {
        //     print!("\n8081 port_is_available");
        //     //std::process::exit(0);
        // } else {
        //     print!("\nNOT!!! 8081 port_is_available");
        //     std::process::exit(0);
        // }

        let tcp = TcpListener::bind(listen_address);

        match tcp {
            Ok(listener) => {
                listen_connections(listener, paths);
            }

            Err(_) => {
                eprintln!("Failed to listen stream");
            }
        }
    }

    pub fn listen_connections(listener: TcpListener, paths: Paths) {
        let paths_lock = Arc::new(RwLock::new(paths));

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let paths = Arc::clone(&paths_lock);

                    spawn(move || {
                        serve_client(stream, paths);
                    });
                }

                Err(error) => {
                    print!("Error receiving stream: {}", error);
                }
            }
        }
    }

    pub struct Context {
        /// A same tcp stream can be used to serve multiple pages. Setting accept_next will continue
        /// to use same connection. Make sure to set `accept_next` to false if request
        /// body is not read completely. It is passed to both Request struct.
        pub accept_next: AtomicBool,
    }

    impl Context {
        pub fn dont_wait(&self) {
            self.accept_next.store(false, Ordering::Relaxed);
        }
    }

    fn serve_client(stream: TcpStream, paths: Arc<RwLock<Paths>>) {
        let context = Context {
            accept_next: AtomicBool::new(true),
        };

        let context_ref = Arc::new(context);

        while context_ref.accept_next.load(Ordering::Relaxed) {
            let stream = stream.try_clone().expect("Error cloning stream");
            decode_request(stream, paths.clone(), context_ref.clone());
        }
    }

    pub fn decode_request(mut stream: TcpStream, paths: Arc<RwLock<Paths>>, context: Arc<Context>) {
        let mut header_start = String::new();
        let mut partial_body_bytes = Vec::new();

        const MAX_HEADER_SIZE: usize = 1024 * 1024; // 1 MiB
        let headers_result = extract_headers(
            &mut stream,
            &mut header_start,
            &mut partial_body_bytes,
            MAX_HEADER_SIZE,
        );

        if !headers_result.is_ok() {
            context.accept_next.store(false, Ordering::Relaxed);
            return;
        }

        let headers = headers_result.unwrap();

        let request_info = parse_request_method_header(&header_start.as_str());
        if !request_info.is_some() {
            context.accept_next.store(false, Ordering::Relaxed);
            let _ = stream.shutdown(Shutdown::Both);
            return;
        }

        let (method, raw_path) = request_info.unwrap();

        // These states are shared among request and response
        let body_read = Arc::new(AtomicBool::from(false));
        let body_parsed = Arc::new(AtomicBool::from(false));

        let mut request = Request::new(
            context,
            stream,
            method,
            raw_path,
            headers,
            body_read.clone(),
            body_parsed.clone(),
        );
        request.setup();

        // Some bytes are read unintentionally from the body. Set read value in the struct.
        request.set_partial_body_bytes(partial_body_bytes);

        let mut matched_view: Option<&SinglePath> = None;

        let binding = paths.read().unwrap();
        for path in binding.iter() {
            if request.pathname == path.name {
                matched_view = Some(&path);
            }
        }

        if let Some(view) = matched_view {
            serve_page(request, view);
        } else {
            serve_not_found(request);
        }
    }

    fn serve_page(request: Request, matched_path: &SinglePath) {
        let response = Response::new(request.clone());
        (matched_path.view)(request, response);
    }

    fn serve_not_found(request: Request) {
        let mut response = Response::new(request);
        response.html(404, "404 NOT FOUND".to_string());
        response.send();
    }
}
