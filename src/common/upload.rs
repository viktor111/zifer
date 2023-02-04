use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

use crate::common::helpers::{create_stream, socket_address_from_string_ip};
use std::error::Error;

pub async fn create_reader(file_path: &Path) -> Result<BufReader<File>, Box<dyn Error>> {
    if !file_path.exists() {
        return Err("[-] File does not exist".into());
    }

    if !file_path.is_file() {
        return Err("[-] Path is not a file".into());
    }

    let file = File::open(file_path)?;

    let reader = BufReader::new(file);

    Ok(reader)
}

pub async fn create_listener(ip: &str) -> Result<TcpStream, Box<dyn Error>> {
    let listener = create_stream(ip).await?;

    Ok(listener)
}

pub async fn write_file_name(stream: &mut TcpStream, file_path: &Path) -> Result<(), Box<dyn Error>>{
    let file_name = file_path.file_name().unwrap().to_str().unwrap().to_string();

    // write the len of file name
    stream
        .write_all(&(file_name.len() as u32).to_be_bytes())
        .await?;

    // write the file name
    stream.write_all(file_name.as_bytes()).await?;

    Ok(())
}

pub async fn read_chunk_from_file(
    reader: &mut BufReader<File>,
) -> Result<([u8; 1024], u32), Box<dyn Error>> {
    let mut buffer = [0; 1024];
    let read_bytes = reader.read(&mut buffer)?;
    let read_bytes = read_bytes as u32;

    Ok((buffer, read_bytes))
}

pub async fn write_chunk(
    stream: &mut TcpStream,
    read_bytes: u32,
    chunk: &[u8; 1024],
) -> Result<(), Box<dyn Error>> {
    let len_bytes = &1024u32.to_be_bytes();

    if read_bytes < 1024 {
        stream.write_all(&read_bytes.to_be_bytes()).await?;
    } else {
        stream.write_all(len_bytes).await?;
    }
    stream.write_all(chunk).await?;

    Ok(())
}

pub async fn write_eof(stream: &mut TcpStream) -> Result<(), Box<dyn Error>>{
    stream.write_all(&[0x00]).await?;

    Ok(())
}
