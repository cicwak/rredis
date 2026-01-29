mod core;
mod server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error,>,> {
    let server = server::server::Server::new(core::storage::Storage::new(),);
    server.listen().await?;

    Ok((),)
}
