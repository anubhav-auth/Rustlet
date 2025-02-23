use hyper::{Body, Request, Response, Error};
use crate::router::route_request;

pub async fn handle_request(req: Request<Body>) -> Result<Response<Body>, Error> {
    route_request(req).await
}
