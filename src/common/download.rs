use std::{error::Error, fs::File, io::Write};

use tokio::{net::TcpStream, io::{AsyncReadExt, AsyncWriteExt}};
use tracing::info;

pub async fn read_file_name(stream: &mut TcpStream) -> Result<String, Box<dyn Error>> {
    let mut file_name_len_bytes = [0; 4];
    stream.read_exact(&mut file_name_len_bytes).await.unwrap();

    let file_name_len: u32 = u32::from_be_bytes(file_name_len_bytes);

    let mut file_name_bytes = vec![0; file_name_len as usize];
    stream.read_exact(&mut file_name_bytes).await.unwrap();

    let file_name = String::from_utf8(file_name_bytes.to_vec()).unwrap();

    Ok(file_name)
}

pub fn create_file(file_name: &str) -> Result<File, Box<dyn Error>> {
    let file = std::fs::File::create(&file_name).unwrap();
    Ok(file)
}

pub async fn read_chunk(stream: &mut TcpStream) -> Result<Option<Vec<u8>>, Box<dyn Error>> {
    let mut chunk_len_bytes = [0; 4];
    stream.read_exact(&mut chunk_len_bytes).await.unwrap();
    let chunk_len = u32::from_be_bytes(chunk_len_bytes);

    if chunk_len == 0 {
        info!("[+] File read to the end");
        return Ok(None)
    }

    let mut frame_data = vec![0; chunk_len as usize];
    stream.read_exact(&mut frame_data).await.unwrap();

    Ok(Some(frame_data))
}

pub async fn write_chunk_to_file(file: &mut File, chunk: &Vec<u8>){
    file.write_all(chunk).unwrap();
}