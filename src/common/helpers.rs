use std::{
    error::Error,
    net::{IpAddr, Ipv4Addr, SocketAddr},
};

use tokio::{net::{TcpListener, TcpStream}};

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