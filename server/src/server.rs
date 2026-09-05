use std::{
    path::Path,
    sync::{Arc, Mutex},
};

use db::{Database, parse_command};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::{TcpListener, TcpStream, ToSocketAddrs},
};

pub struct Server {
    listener: TcpListener,
    db: Arc<Mutex<Database>>,
}

impl Server {
    pub async fn new<A: ToSocketAddrs, P: AsRef<Path>>(
        addr: A,
        path: P,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let listener = TcpListener::bind(addr).await?;
        let db = Database::new(path)?;

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
                handle_client(socket, db)
                    .await
                    .unwrap_or_else(|e| eprintln!("Error handling client: {}", e));
            });
        }
    }
}

async fn handle_client(
    socket: TcpStream,
    db: Arc<Mutex<Database>>,
) -> Result<(), Box<dyn std::error::Error>> {
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

        let db_clone = Arc::clone(&db);
        let command = trimmed.to_owned();

        let result = tokio::task::spawn_blocking(move || {
            let mut guard = db_clone.lock().unwrap();
            parse_command(&mut guard, &command)
        })
        .await?;

        match result {
            Ok(s) => {
                println!("Processed: {}", trimmed);
                writer.write_all(format!("{}\n", s).as_bytes()).await?;
            }
            Err(e) => {
                eprintln!("ERROR: {}", e);
                writer
                    .write_all(format!("ERROR: {}\n", e).as_bytes())
                    .await?;
            }
        }
    }

    Ok(())
}
