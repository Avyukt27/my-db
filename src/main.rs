use crate::server::Server;

mod server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server = Server::new("127.0.0.1:8080", "db.db").await?;
    server.run().await
}
