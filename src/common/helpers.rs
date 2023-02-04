use std::{
    error::Error,
    net::{IpAddr, Ipv4Addr, SocketAddr}, path::Path,
};
use tracing::{error};

use tokio::{net::{TcpListener, TcpStream}, io::{AsyncWriteExt, AsyncReadExt}};

pub fn validate_path(path: &Path) -> Result<(), Box<dyn Error>> {
    if !path.is_dir() {
        return Err("[-] Not a directory".into());
    }

    Ok(())
}

pub async fn create_stream(addr: SocketAddr) -> Result<TcpStream, Box<dyn Error>> {
    let stream = TcpStream::connect(addr).await;

    match stream {
        Ok(stream) => Ok(stream),
        Err(e) => Err(e.into()),
    }
}

pub async fn create_listener(addr: SocketAddr) -> Result<TcpListener, Box<dyn Error>> {
    let listener = TcpListener::bind(addr).await;

    match listener {
        Ok(listener) => Ok(listener),
        Err(e) => Err(e.into()),
    }
}

pub async fn listener_accept_conn(
    listener: &TcpListener,
) -> Result<(TcpStream, SocketAddr), Box<dyn Error>> {
    let accepted = listener.accept().await;

    match accepted {
        Ok((stream, addr)) => Ok((stream, addr)),
        Err(e) => Err(e.into()),
    }
}

pub async fn write_message(stream: &mut TcpStream, message: &str) -> Result<(), Box<dyn Error>> {
    let len = message.len() as u32;
    let len_bytes = len.to_be_bytes();
    stream.write_all(&len_bytes).await?;

    stream.write_all(message.as_bytes()).await?;
    Ok(())
}

pub async fn recieve_command(stream: &mut TcpStream) -> Result<String, Box<dyn Error>>{
    let mut command_len_bytes = [0; 4];
    stream.read_exact(&mut command_len_bytes).await.unwrap();
    let command_len = u32::from_be_bytes(command_len_bytes);

    let mut command = vec![0; command_len as usize];
    stream.read_exact(&mut command).await.unwrap();
    let command = String::from_utf8_lossy(&command);

    Ok(command.to_string())
}

pub fn socket_address_from_string_ip(ip: String) -> Result<SocketAddr, Box<dyn Error>> {
    const INVALID_IP_ERROR: &str = "Invalid IP address - should be in format: 127.0.0.1:8080";

    let ip = ip.split(":").collect::<Vec<&str>>();
    let port = ip[1].parse::<u16>().expect(INVALID_IP_ERROR);

    let ip_parts = ip[0].split(".").collect::<Vec<&str>>();

    if ip_parts.len() != 4 {
        return Err(INVALID_IP_ERROR.into());
    }

    let mut ip_parts_u8 = Vec::new();
    for part in ip_parts {
        let part_u8 = part.parse::<u8>();
        if part_u8.is_err() {
            return Err(INVALID_IP_ERROR.into());
        }
        ip_parts_u8.push(part_u8.unwrap());
    }

    let ip_addr = IpAddr::V4(Ipv4Addr::new(
        ip_parts_u8[0],
        ip_parts_u8[1],
        ip_parts_u8[2],
        ip_parts_u8[3],
    ));

    let socket_addr = SocketAddr::new(ip_addr, port);

    return Ok(socket_addr);
}