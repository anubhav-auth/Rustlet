use hyper::{Request, Response};
use std::task::{Context, Poll};
use tower::Service;
use std::pin::Pin;
use std::future::Future;
use tracing::info;

#[derive(Clone)]
pub struct Logger<S> {
    inner: S,
}

impl<S> Logger<S> {
    pub fn new(inner: S) -> Self {
        Logger { inner }
    }
}

impl<S, ReqBody, ResBody> Service<Request<ReqBody>> for Logger<S>
where
    S: Service<Request<ReqBody>, Response = Response<ResBody>> + Clone + Send + 'static,
    S::Future: Send + 'static,
    ReqBody: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<ReqBody>) -> Self::Future {
        info!("Incoming request: {} {}", req.method(), req.uri().path());
        let fut = self.inner.call(req);
        Box::pin(async move {
            let response = fut.await?;
            info!("Response status: {}", response.status());
            Ok(response)
        })
    }
}
