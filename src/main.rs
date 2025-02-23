mod server;
mod handler;
mod router;
mod middleware;
mod config;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let config = config::Config::from_env();
    if let Err(e) = server::run(config).await {
        eprintln!("Server encountered an error: {:?}", e);
    }
}
