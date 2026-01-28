use std::net::SocketAddr;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

use crate::core::storage::Storage;

pub struct Server{
    storage: Storage,
}

impl Server{
    pub fn new(storage: Storage) -> Self{
        Self{ storage }
    }

    pub async fn listen(&self) -> Result<(), Box<dyn std::error::Error>>{
        let addr = "127.0.0.1:8080";
        let listener = TcpListener::bind(addr).await?;

        loop {
            let (socket, addr) = listener.accept().await?;

            self.connect(socket, addr);
        }
    }

    pub fn connect(&self, mut socket: TcpStream, _addr: SocketAddr){
        let storage = self.storage.clone();

        tokio::spawn(async move {
            let mut buf = [0; 1024];

            loop {
                let n = match socket.read(&mut buf).await {
                    Ok(n) if n == 0 => return ,
                    Ok(n) => n,
                    Err(e) => {
                        eprintln!("Error on read: {}", e);
                        return;
                    }
                };

                let message = storage
                    .process_input(
                        String::from_utf8_lossy(&buf[..n]).to_string().as_str()
                    ).unwrap_or_else(|err| {format!("Error: {err}")} );
                

                if let Err(e) = socket.write_all((message + "\r\n").to_string().as_bytes()).await {
                    eprintln!("Error on write to socket: {}", e);
                    return;
                }
            }
        });
    }
}

