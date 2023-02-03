use std::{error::Error, path::Path};

use tokio::{net::TcpStream};
use tracing::info;

use crate::common::helpers;
use crate::common::download;

pub async fn init_server(ip: &str, dir: &str) -> Result<(), Box<dyn Error>> {
    let path = Path::new(dir);

    helpers::validate_path(path)?;

    let socket_addr = helpers::socket_address_from_string_ip(ip.to_string()).expect("Invalid IP address");
    let listener = helpers::create_listener(socket_addr).await.unwrap();
    
    info!("[.] Server started on port 7677");
    
    loop{
        let (stream, _) = helpers::listener_accept_conn(&listener).await.unwrap();
        info!("[.] New connection established");
        handle_connection(stream, dir).await;
    }
}

async fn handle_connection(mut stream: TcpStream, dir: &str) {
    tokio::spawn(async move {
        loop {
            let file_name = download::read_file_name(&mut stream).await.unwrap();
            info!("[+] Recieved file name {}", file_name);

            let mut file = download::create_file(&file_name).unwrap();
            info!("[+] Starting to read file...");

            loop{
                let chunk = download::read_chunk(&mut stream).await.unwrap();

                match chunk {
                    Some(chunk) => download::write_chunk_to_file(&mut file, &chunk).await,
                    None => return,
                }
            }
        }
    });
}

