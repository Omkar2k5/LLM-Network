use tokio::net::{TcpStream, TcpListener};
use tokio::io::{AsyncWriteExt, AsyncReadExt};
use tokio::sync::Mutex;
use tokio::time::sleep;
use std::sync::Arc;
use std::time::Duration;
use std::path::PathBuf;
use std::collections::HashSet;

const PORT: i32 = 7878;

#[derive(Debug)]
enum Message {
    FileInfo {
        path: PathBuf,
        size: u64,
    },
    FileData(Vec<u8>),
}

impl Message {
    async fn send(&self, stream: &mut TcpStream) -> std::io::Result<()> {
        match self {
            Message::FileInfo { path, size } => {
                stream.write_all(b"INFO:").await?;
                let data = format!("{}{}", path.to_string_lossy(), size);
                let len = data.len() as u64;
                stream.write_all(&len.to_le_bytes()).await?;
                stream.write_all(data.as_bytes()).await?;
            }
            Message::FileData(data) => {
                stream.write_all(b"DATA:").await?;
                let len = data.len() as u64;
                stream.write_all(&len.to_le_bytes()).await?;
                stream.write_all(data).await?;
            }
        }
        stream.flush().await?;
        Ok(())
    }

    async fn receive(stream: &mut TcpStream) -> std::io::Result<Option<Message>> {
        let mut marker = [0u8; 5];
        if let Ok(0) = stream.read_exact(&mut marker).await {
            return Ok(None);
        }

        let mut len_bytes = [0u8; 8];
        stream.read_exact(&mut len_bytes).await?;
        let len = u64::from_le_bytes(len_bytes) as usize;

        let mut data = vec![0u8; len];
        stream.read_exact(&mut data).await?;

        match &marker {
            b"INFO:" => {
                let content = String::from_utf8_lossy(&data);
                let path_str = &content[..content.len()-20]; // Assuming size is at most 20 digits
                let size_str = &content[content.len()-20..];
                Ok(Some(Message::FileInfo {
                    path: PathBuf::from(path_str),
                    size: size_str.trim().parse().unwrap_or(0),
                }))
            }
            b"DATA:" => Ok(Some(Message::FileData(data))),
            _ => Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Unknown message type")),
        }
    }
}

pub async fn listen_for_connections() -> std::io::Result<()> {
    let listener = TcpListener::bind(format!("0.0.0.0:{}", PORT)).await?;
    println!("Listening for incoming connections on port {}", PORT);

    loop {
        let (stream, addr) = listener.accept().await?;
        println!("New incoming connection from {}", addr);
        tokio::spawn(async move {
            if let Err(e) = handle_connection(stream).await {
                eprintln!("Connection error: {}", e);
            }
        });
    }
}

async fn handle_connection(_stream: TcpStream) -> std::io::Result<()> {
    // Keep connection alive
    loop {
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}

pub async fn connect_to_peers(received_ips: Arc<Mutex<HashSet<String>>>) {
    loop {
        let mut ips = received_ips.lock().await;
        for ip in ips.drain() {
            let addr = format!("{}{}", ip, PORT);
            match TcpStream::connect(&addr).await {
                Ok(_) => println!("Connected to peer: {}", addr),
                Err(e) => eprintln!("Failed to connect to {}: {}", addr, e),
            }
        }
        drop(ips);
        sleep(Duration::from_secs(5)).await;
    }
}
