use std::{error::Error, sync::Arc};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
    sync::Mutex,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stream = TcpStream::connect("127.0.0.1:7667")
        .await
        .expect("[-] Failed to connect to server check if server is running on port 7667");
    let stream = Arc::new(Mutex::new(stream));
    let stream_clone = Arc::clone(&stream);
    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.unwrap();
        disconnect(&stream_clone).await.unwrap();
        std::process::exit(0);
    });
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;

        if input.trim() == "CLOSE" {
            disconnect(&stream).await?;
            std::process::exit(0);
        }

        send_message(&stream, &input.trim()).await?;

        let response = read_message(&stream).await?;
        println!("{}", response);
    }
}

pub async fn disconnect(stream: &Arc<Mutex<TcpStream>>) -> Result<(), Box<dyn Error>> {
    let close_msg = "CLOSE";
    send_message(stream, close_msg).await?;
    let response = read_message(stream).await?;
    println!("{}", response);
    let mut stream = stream.lock().await;
    stream.shutdown().await?;
    Ok(())
}

pub async fn read_message(stream: &Arc<Mutex<TcpStream>>) -> Result<String, Box<dyn Error>> {
    let mut stream = stream.lock().await;
    let mut len_bytes = [0; 4];
    let bytes_read = stream.read(&mut len_bytes).await?;
    if bytes_read < 4 {
        return Err("[-] Failed to read the length of the message".into());
    }
    let len = u32::from_be_bytes(len_bytes);
    let mut buffer = vec![0; len as usize];
    stream.read_exact(&mut buffer).await?;
    let message = String::from_utf8_lossy(&buffer);
    Ok(message.to_string())
}

pub async fn send_message(
    stream: &Arc<Mutex<TcpStream>>,
    message: &str,
) -> Result<(), Box<dyn Error>> {
    let mut stream = stream.lock().await;
    let len = message.len() as u32;
    let len_bytes = len.to_be_bytes();
    stream.write_all(&len_bytes).await?;
    stream.write_all(message.as_bytes()).await?;
    Ok(())
}
