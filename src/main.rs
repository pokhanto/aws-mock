use std::{io::Write, net::TcpListener};

use request::Request;
use response::Response;
use s3::{get_object, upload};

mod request;
mod response;
mod s3;

struct Route {
    pub predicate: Box<dyn Fn(&Request) -> bool>,
    pub resolver: Box<dyn Fn(Request) -> Response>,
}

#[derive(Default)]
struct Router {
    routes: Vec<Route>,
}

impl Router {
    pub fn register_route(
        &mut self,
        predicate: Box<dyn Fn(&Request) -> bool>,
        resolver: Box<dyn Fn(Request) -> Response>,
    ) {
        self.routes.push(Route {
            predicate,
            resolver,
        });
    }

    pub fn route_request(&self, request: Request) -> Option<Response> {
        for route in &self.routes {
            if (route.predicate)(&request) {
                return Some((route.resolver)(request));
            }
        }

        None
    }
}

fn main() {
    let mut router = Router::default();
    router.register_route(
        Box::new(|request: &Request| {
            return request.method == "PUT";
        }),
        Box::new(upload),
    );
    router.register_route(
        Box::new(|request: &Request| {
            return request.method == "GET";
        }),
        Box::new(get_object),
    );
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let request = Request::new(&mut stream);
                println!(
                    "{} {} {:?}",
                    &request.method, &request.path, &request.headers
                );

                let response = router.route_request(request).unwrap();

                stream.write(response.header.as_slice()).unwrap();
                if let Some(body) = response.body {
                    stream.write(body.as_slice()).unwrap();
                }
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
    println!("Hello, world!");
}
