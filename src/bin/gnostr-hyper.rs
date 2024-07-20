use bytes::Bytes;
use hyper::{
    body::to_bytes,
    service::{make_service_fn, service_fn},
    Body, Request, Server,
};

//use crate::param_handler;
use gnostr_web::handler::param_handler;
//use crate::send_handler;
use gnostr_web::handler::send_handler;
//use crate::test_handler;
use gnostr_web::handler::test_handler;

use route_recognizer::Params;
//use router::Router;
use gnostr_web::router::Router;
use std::sync::Arc;

//mod handler;
//mod router;

type Response = hyper::Response<hyper::Body>;
type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

#[derive(Clone, Debug)]
pub struct AppState {
    pub state_thing: String,
}

#[tokio::main]
async fn main() {
    let some_state = "state".to_string();

    let mut router: Router = Router::new();
    //router.get("/test", Box::new(handler::test_handler));
    router.get("/test", Box::new(test_handler));
    //router.post("/send", Box::new(handler::send_handler));
    router.post("/send", Box::new(send_handler));
    //router.get("/params/:some_param", Box::new(handler::param_handler));
    router.get("/params/:some_param", Box::new(param_handler));

    let shared_router = Arc::new(router);
    let new_service = make_service_fn(move |_| {
        let app_state = gnostr_web::AppState {
            state_thing: some_state.clone(),
        };

        let router_capture = shared_router.clone();
        async {
            Ok::<_, Error>(service_fn(move |req| {
                route(router_capture.clone(), req, app_state.clone())
            }))
        }
    });

    use std::env;
    //use std::net::SocketAddr;
    //use std::net::ToSocketAddrs;

    // let mut addr: SocketAddr = 
        // "0.0.0.0".parse().expect("address creation works");
    use std::net::SocketAddr;
    //let mut addr = "0.0.0.0:8081".parse().expect("address creation works");
    let args: Vec<String> = env::args().collect();
    if args.len() == 0 {
    let mut addr: SocketAddr = 
        "0.0.0.0:8080".parse().expect("address creation works");
        print!("\n0 {}",args.len());
    let server = Server::bind(&addr).serve(new_service.clone());
    println!("\ngnostr-hyper listening on http://{}\n", addr);
    let _ = server.await;
    }
    if args.len() == 1 {
    let mut addr: SocketAddr = 
        "0.0.0.0:8081".parse().expect("address creation works");
        print!("\n0 {}",args.len());
        print!("\n >=1 {}",&args[0]);
    let server = Server::bind(&addr).serve(new_service.clone());
    println!("\ngnostr-hyper listening on http://{}\n", addr);
    let _ = server.await;
    }
    if args.len() == 2 {
    let mut addr: SocketAddr = 
        "0.0.0.0:8082".parse().expect("address creation works");
        print!("\n0 {}",args.len());
        print!("\n >=1 {}",&args[0]);
        print!("\n >=2 {}",&args[1]);
    let server = Server::bind(&addr).serve(new_service.clone());
    println!("\ngnostr-hyper listening on http://{}\n", addr);
    let _ = server.await;
    }
    if args.len() == 3 {
    let mut addr: SocketAddr = 
        "0.0.0.0:8083".parse().expect("address creation works");
        print!("\n0 {}",args.len());
        print!("\n >=1 {}",&args[0]);
        print!("\n >=2 {}",&args[1]);
        print!("\n >=3 {}",&args[2]);
        let query = &args[1];
        let file_path = &args[2];
        //addr = format!("0.0.0.0:{:}", &args[2].to_socket_addrs());
        //addr =  &args[2].to_socket_addrs().unwrap();
        println!("\nSearching for {query}\n");
        println!("\nIn file {file_path}\n");

    let server = Server::bind(&addr).serve(new_service.clone());
    println!("\ngnostr-hyper listening on http://{}\n", addr);
    let _ = server.await;


    }
    // print!("\nhost=0.0.0.0 addr={}\n",addr);
    // let server = Server::bind(&addr).serve(new_service);
    // println!("\ngnostr-hyper listening on http://{}\n", addr);
    // let _ = server.await;
}

async fn route(
    router: Arc<Router>,
    req: Request<hyper::Body>,
    app_state: gnostr_web::AppState,
) -> Result<Response, Error> {
    let found_handler = router.route(req.uri().path(), req.method());
    let resp = found_handler
        .handler
        .invoke(gnostr_web::Context::new(app_state, req, found_handler.params))
        .await;
    Ok(resp)
}

#[derive(Debug)]
pub struct Context {
    pub state: gnostr_web::AppState,
    pub req: Request<Body>,
    pub params: Params,
    body_bytes: Option<Bytes>,
}

impl Context {
    pub fn new(state: gnostr_web::AppState, req: Request<Body>, params: Params) -> Context {
        Context {
            state,
            req,
            params,
            body_bytes: None,
        }
    }

    pub async fn body_json<T: serde::de::DeserializeOwned>(&mut self) -> Result<T, Error> {
        let body_bytes = match self.body_bytes {
            Some(ref v) => v,
            _ => {
                let body = to_bytes(self.req.body_mut()).await?;
                self.body_bytes = Some(body);
                self.body_bytes.as_ref().expect("body_bytes was set above")
            }
        };
        Ok(serde_json::from_slice(&body_bytes)?)
    }
}
