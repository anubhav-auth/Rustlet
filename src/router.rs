use hyper::{Body, Request, Response, StatusCode, Method};

pub async fn route_request(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let response = match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => Response::new(Body::from("Welcome to Rustlet!")),
        (&Method::GET, "/hello") => Response::new(Body::from("Hello, world!")),
        _ => {
            let mut not_found = Response::default();
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            not_found
        }
    };
    Ok(response)
}
