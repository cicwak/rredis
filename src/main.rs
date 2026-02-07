mod core;
mod server;
use crate::core::time_manager::TimeManager;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let time_manager = TimeManager::new();
    let server_start_time = time_manager.get_start_time();

    let server = server::server::Server::new(core::storage::Storage::new(time_manager));

    println!("Start time of server: {}", server_start_time);

    server.listen().await?;

    Ok(())
}
