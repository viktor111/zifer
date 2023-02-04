use std::path::Path;
use std::error::Error;

use tracing::info;

use crate::common::{upload, helpers};

pub async fn init_client_upload(ip: &str, file_path: &str) -> Result<(), Box<dyn Error>> {
    let file_path = Path::new(file_path);

    let mut reader = upload::create_reader(file_path).await?;

    let mut stream = upload::create_listener(ip).await?;

    info!("[+] Connection established");
   
    helpers::write_message(&mut stream, "upload").await?;

    upload::write_file_name(&mut stream, &file_path).await;

    info!("[+] File name sent to server starting file transfer");

    loop {
        let (chunk, read_bytes) = upload::read_chunk_from_file(&mut reader).await?;

        if read_bytes == 0 {
            upload::write_eof(&mut stream).await;

            return Ok(());
        }

        upload::write_chunk(&mut stream, read_bytes, &chunk).await;
    }
}

pub async fn init_client_download(ip: &str, file_name: &str) -> Result<(), Box<dyn Error>> {


    Ok(())
}