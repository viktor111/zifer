use std::path::Path;
use std::error::Error;

use tokio::io::AsyncWriteExt;
use tracing::{info, error};

use crate::common::{upload, helpers, download};

pub async fn init_client_upload(ip: &str, file_path: &str) -> Result<(), Box<dyn Error>> {
    let file_path = Path::new(file_path);

    let mut reader = upload::create_reader(file_path).await?;

    let mut stream = upload::create_listener(ip).await?;

    info!("[+] Connection established");
   
    helpers::write_message(&mut stream, "upload").await?;

    upload::write_file_name(&mut stream, &file_path).await.unwrap();

    info!("[+] File name sent to server starting file transfer");

    loop {
        let (chunk, read_bytes) = upload::read_chunk_from_file(&mut reader).await?;

        if read_bytes == 0 {
            upload::write_eof(&mut stream).await.unwrap();

            return Ok(());
        }

        upload::write_chunk(&mut stream, read_bytes, &chunk).await.unwrap();
    }
}

pub async fn init_client_download(ip: &str, file_name: &str) -> Result<(), Box<dyn Error>> {

    let mut stream = upload::create_listener(ip).await?;

    info!("[+] Connection established");

    helpers::write_message(&mut stream, "download").await?;

    helpers::write_message(&mut stream, file_name).await?;

    let server_response_file = helpers::read_message(&mut stream).await.unwrap();

    if server_response_file == "error" {
        error!("Server responded with error check the file name or server logs");
        stream.shutdown().await.unwrap();
        return Ok(());
    }

    let path = Path::new(file_name);
    let file_name = path.file_name().unwrap().to_str().unwrap().to_string();
    
    let mut file = download::create_file(&file_name).unwrap();
    info!("Created file with name {}", file_name);
    info!("Starting to read file...");

    loop {
        let chunk = download::read_chunk(&mut stream).await.unwrap();
        match chunk {
            Some(chunk) => download::write_chunk_to_file(&mut file, &chunk).await,
            None => return Ok(()),
        }
    }
}