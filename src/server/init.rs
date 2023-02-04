use std::fs::File;
use std::io::BufReader;
use std::{error::Error, path::Path};

use tokio::net::TcpStream;
use tracing::{error, info};

use crate::common::helpers;
use crate::common::{download, upload};

pub async fn init_server(ip: &str, dir: &str) -> Result<(), Box<dyn Error>> {
    let path = Path::new(dir);

    helpers::validate_path(path)?;

    let socket_addr =
        helpers::socket_address_from_string_ip(ip.to_string()).expect("Invalid IP address");

    let listener = helpers::create_listener(socket_addr).await.unwrap();
    info!("Server started on port 7677");

    loop {
        let (stream, _) = helpers::listener_accept_conn(&listener).await.unwrap();
        info!("New connection established");
        handle_connection(stream).await;
    }
}

async fn handle_connection(mut stream: TcpStream) {
    tokio::spawn(async move {
        let command = helpers::read_message(&mut stream).await.unwrap();
        info!("Recieved command for {}", command);

        if command == "upload" {
            let file_name = download::read_file_name(&mut stream).await.unwrap();
            info!("Recieved file name {}", file_name);

            let mut file = download::create_file(&file_name).unwrap();
            info!("Starting to read file...");

            loop {
                let upload = client_is_uploading(&mut stream, &mut file).await.unwrap();
                if upload.is_none() {
                    return;
                }
            }
        } else if command == "download" {
            let file_name = helpers::read_message(&mut stream).await.unwrap();

            let file_path = Path::new(&file_name);

            if !file_path.is_file() {
                error!("File does not exist");
                helpers::write_message(&mut stream, "error").await.unwrap();
                return;
            }

            helpers::write_message(&mut stream, "ok").await.unwrap();

            let mut reader = upload::create_reader(file_path).await.unwrap();

            loop {
                let download = client_is_downloading(&mut stream, &mut reader)
                    .await
                    .unwrap();
                if download.is_none() {
                    return;
                }
            }
        }
    });
}

async fn client_is_uploading(
    stream: &mut TcpStream,
    file: &mut File,
) -> Result<Option<bool>, Box<dyn Error>> {
    loop {
        let chunk = download::read_chunk(stream).await.unwrap();

        match chunk {
            Some(chunk) => download::write_chunk_to_file(file, &chunk).await,
            None => return Ok(None),
        }
    }
}

async fn client_is_downloading(
    stream: &mut TcpStream,
    reader: &mut BufReader<File>,
) -> Result<Option<bool>, Box<dyn Error>> {
    loop {
        let (chunk, read_bytes) = upload::read_chunk_from_file(reader).await?;

        if read_bytes == 0 {
            upload::write_eof(stream).await;

            return Ok(None);
        }

        upload::write_chunk(stream, read_bytes, &chunk).await;
    }
}
