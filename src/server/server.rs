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
            println!("Новое соединение: {}", addr);

            self.connect(socket, addr);
        }
    }

    pub fn connect(&self, mut socket: TcpStream, addr: SocketAddr){
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

                println!("Received {} bytes from {}", n, addr);
                // println!("Received: {}", String::from_utf8_lossy(&buf[..n]));
                let message = storage
                    .process_input(
                        String::from_utf8_lossy(&buf[..n]).to_string().as_str()
                    ).unwrap_or_else(|err| {format!("Unexpected error: {err}")} );


                // let message = match message {
                //     Ok(res) => res,
                //     Err(error) => format!("Unexpected error: {error}\r\n", ).as_str().parse().unwrap(),
                // };

                if let Err(e) = socket.write_all((message + "\r\n").to_string().as_bytes()).await {
                    eprintln!("Ошибка при записи: {}", e);
                    return;
                }
            }
        });
    }
}

