use std::{error::Error};
use tokio::{net::{TcpStream}, io::{AsyncWriteExt, AsyncReadExt}};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = TcpStream::connect("127.0.0.1:7667").await?;

    // Send data to the server
    let set_msg = "SEwT name John";
    let get_msg = "GET name";

    send_message(&mut stream, set_msg).await?;
    let response = read_message(&mut stream).await?;
    println!("Response: {}", response);
    send_message(&mut stream, get_msg).await?;
    let response = read_message(&mut stream).await?;
    println!("Response: {}", response);

    let close_msg = "CLOSE";
    send_message(&mut stream, close_msg).await?;

    // Wait for response
    let response = read_message(&mut stream).await?;
    println!("Response: {}", response);

    // Close connection
    stream.shutdown().await?;

    Ok(())
}

pub async fn read_message(stream: &mut TcpStream) -> Result<String, Box<dyn Error>> {
    let mut len_bytes = [0; 4];
    let bytes_read = stream.read(&mut len_bytes).await?;
    if bytes_read < 4 {
        return Err("Failed to read the length of the message".into());
    }
    let len = u32::from_be_bytes(len_bytes);
    let mut buffer = vec![0; len as usize];
    stream.read_exact(&mut buffer).await?;
    let message = String::from_utf8_lossy(&buffer);
    Ok(message.to_string())
}

pub async fn send_message(stream: &mut TcpStream, message: &str) -> Result<(), Box<dyn Error>> {
    let len = message.len() as u32;
    let len_bytes = len.to_be_bytes();
    stream.write_all(&len_bytes).await?;
    stream.write_all(message.as_bytes()).await?;
    Ok(())
}