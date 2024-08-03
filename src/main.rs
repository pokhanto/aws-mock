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
    // let size = &response_body.to_string().as_bytes().len();
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
                println!("{}", &request.method);
                println!("{}", &request.path);
                println!("{:?}", &request.headers);

                // let date = request
                //     .headers
                //     .get("X-Amz-Date")
                //     .cloned()
                //     .unwrap_or_default();

                // let response = Response {
                //     code: "200".into(),
                //     headers: vec![
                //         "x-amzn-RequestId: 2121212".into(),
                //         "Content-Type: application/x-amz-json-1.0".into(),
                //         format!("Content-Length: {}", size).into(),
                //         format!("Date: {}", date).into(),
                //     ],
                //     body: Some(response_body.to_string().into()),
                // };

                let response = router.route_request(request).unwrap();

                stream
                    .write_all(response.to_http_response().as_bytes())
                    .unwrap();
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
    println!("Hello, world!");
}
