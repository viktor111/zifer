use std::fs::File;
use std::{error::Error, path::Path};

use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;
use tracing::info;

use crate::common::download;
use crate::common::helpers;

pub async fn init_server(ip: &str, dir: &str) -> Result<(), Box<dyn Error>> {
    let path = Path::new(dir);

    helpers::validate_path(path)?;

    let socket_addr =
        helpers::socket_address_from_string_ip(ip.to_string()).expect("Invalid IP address");
    let listener = helpers::create_listener(socket_addr).await.unwrap();

    info!("[.] Server started on port 7677");

    loop {
        let (stream, _) = helpers::listener_accept_conn(&listener).await.unwrap();
        info!("[.] New connection established");
        handle_connection(stream, dir).await;
    }
}

async fn handle_connection(mut stream: TcpStream, dir: &str) {
    tokio::spawn(async move {
        // TODO: Refactor this code
        let mut command_len_bytes = [0; 4];
        stream.read_exact(&mut command_len_bytes).await.unwrap();
        let command_len = u32::from_be_bytes(command_len_bytes);

        let mut command = vec![0; command_len as usize];
        stream.read_exact(&mut command).await.unwrap();
        let command = String::from_utf8_lossy(&command);
        info!("Recieved command for {}", command);
        let file_name = download::read_file_name(&mut stream).await.unwrap();
        info!("[+] Recieved file name {}", file_name);

        let mut file = download::create_file(&file_name).unwrap();
        info!("[+] Starting to read file...");

        loop {
            if &command == "download" {
                client_is_downloading(&mut stream).await;
            } else if &command == "upload" {
                let upload = client_is_uploading(&mut stream, &mut file).await.unwrap();
                if upload.is_none(){
                    return;
                }
            }
        }
    });
}

async fn client_is_uploading(stream: &mut TcpStream, file: &mut File) -> Result<Option<bool>, Box<dyn Error>>{
    loop {
        let chunk = download::read_chunk(stream).await.unwrap();

        match chunk {
            Some(chunk) => download::write_chunk_to_file(file, &chunk).await,
            None => {
                info!("Exit stream chunk is none");
                return Ok(None);
            },
        }
    }
}

async fn client_is_downloading(stream: &mut TcpStream) {}
