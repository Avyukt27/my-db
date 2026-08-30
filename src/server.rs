use std::{
    path::Path,
    sync::{Arc, Mutex},
};

use my_db::db::DataBase;
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::{TcpListener, TcpStream, ToSocketAddrs},
};

pub struct Server {
    listener: TcpListener,
    db: Arc<Mutex<DataBase>>,
}

impl Server {
    pub async fn new<A: ToSocketAddrs, P: AsRef<Path>>(
        addr: A,
        path: P,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let listener = TcpListener::bind(addr).await?;
        let db = DataBase::new(path)?;

        Ok(Self {
            listener,
            db: Arc::new(Mutex::new(db)),
        })
    }

    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Server is running");

        loop {
            let (socket, _addr) = self.listener.accept().await?;
            let db = Arc::clone(&self.db);

            tokio::spawn(async move {
                println!("New client connected!");
                Self::handle_client(socket)
                    .await
                    .unwrap_or_else(|e| eprintln!("Error handling client: {}", e));
            });
        }
    }

    async fn handle_client(socket: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
        let (reader, mut writer) = socket.into_split();
        let mut reader = BufReader::new(reader);
        let mut line = String::new();

        loop {
            line.clear();
            let bytes_read = reader.read_line(&mut line).await?;
            if bytes_read == 0 {
                println!("Client disconnected");
                break;
            }

            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }

            println!("Received: {}", trimmed);
            writer.write_all(b"OK\n").await?;
        }

        Ok(())
    }
}
