use std::env;

use server::Server;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

async fn spawn_test_server() -> TcpStream {
    let port = rand::random::<u16>();
    let mut path = env::temp_dir();
    path.push(format!("net_test_{}_{}.db", std::process::id(), port));
    let _ = std::fs::remove_file(&path);
    let server = Server::new(format!("127.0.0.1:{}", port), path)
        .await
        .unwrap();
    let addr = server.addr();

    tokio::spawn(async move {
        let _ = server.run().await;
    });

    TcpStream::connect(addr).await.unwrap()
}

#[tokio::test]
async fn test_tcp_set_get() {
    let mut stream = spawn_test_server().await;

    stream.write_all(b"SET username TEST\n").await.unwrap();
    let mut buf = [0; 1024];
    let n = stream.read(&mut buf).await.unwrap();
    let response = String::from_utf8_lossy(&buf[..n]);
    assert!(response.contains("Ok"));

    stream.write_all(b"GET username\n").await.unwrap();
    let n = stream.read(&mut buf).await.unwrap();
    let response = String::from_utf8_lossy(&buf[..n]);
    assert!(response.contains("TEST"));
}

#[tokio::test]
async fn test_tcp_del() {
    let mut stream = spawn_test_server().await;

    stream.write_all(b"SET username TEST\n").await.unwrap();
    let mut buf = [0; 1024];
    let n = stream.read(&mut buf).await.unwrap();
    let response = String::from_utf8_lossy(&buf[..n]);
    assert!(response.contains("Ok"));

    stream.write_all(b"DEL username\n").await.unwrap();
    let n = stream.read(&mut buf).await.unwrap();
    let response = String::from_utf8_lossy(&buf[..n]);
    assert!(response.contains("TEST"));

    stream.write_all(b"DEL username\n").await.unwrap();
    let n = stream.read(&mut buf).await.unwrap();
    let response = String::from_utf8_lossy(&buf[..n]);
    assert!(response.contains("ERROR"));
}
