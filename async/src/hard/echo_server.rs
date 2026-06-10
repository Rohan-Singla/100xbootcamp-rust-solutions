/*
  Problem 99: Async TCP Echo Server (Simplified)

  Write an async function that starts a mock TCP echo server using
  tokio::net::TcpListener on a given port. It should accept one connection,
  read exactly 5 bytes, and write them back. Return the bytes read.

  Run the tests for this problem with:
    cargo test --test echo_server_test
*/

use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub async fn run_echo_server(port: u16) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let addr = format!("127.0.0.1:{}", port);
    let listener = TcpListener::bind(addr).await?;
    let (mut socket, _) = listener.accept().await?;

    let mut buf = Vec::new();
    let mut read_buf = [0u8; 1024];

    loop {
        match socket.read(&mut read_buf).await? {
            0 => break, // connection closed
            n => buf.extend_from_slice(&read_buf[..n]),
        }
    }
    if !buf.is_empty() {
        socket.write_all(&buf).await?;
    }
    Ok(buf)
}
