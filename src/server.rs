use crate::{handler::handle_request, config::Config, middleware::Logger};
use hyper::{service::service_fn, server::conn::Http};
use tokio::net::TcpListener;
use tower::ServiceBuilder;

pub async fn run(config: Config) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = config.server_addr;
    let listener = TcpListener::bind(addr).await?;
    println!("Server listening on http://{}", addr);

    loop {
        let (socket, _) = listener.accept().await?;

        let service = service_fn(handle_request);

        let service = ServiceBuilder::new()
            .layer_fn(|inner| Logger::new(inner))
            .service(service);

        tokio::spawn(async move {
            if let Err(e) = Http::new().serve_connection(socket, service).await {
                eprintln!("Error serving connection: {:?}", e);
            }
        });
    }
}
