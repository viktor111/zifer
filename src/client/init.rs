use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

use crate::common::helpers::{create_stream, socket_address_from_string_ip};
use std::error::Error;

pub async fn init_client(ip: &str, file_path: &str) -> Result<(), Box<dyn Error>> {
    let file_path = Path::new(file_path);

    let mut reader = create_reader(file_path).await?;

    let mut stream = create_listener(ip).await?;

    write_file_name(&mut stream, &file_path).await;

    loop {
        let (chunk, read_bytes) = read_chunk_from_file(&mut stream, &mut reader).await?;

        if read_bytes == 0 {
            write_EOF(&mut stream).await;

            return Ok(());
        }

        write_chunk(&mut stream, read_bytes, &chunk).await;
    }
}

async fn create_reader(file_path: &Path) -> Result<BufReader<File>, Box<dyn Error>> {
    if !file_path.exists() {
        return Err("File does not exist".into());
    }

    if !file_path.is_file() {
        return Err("Path is not a file".into());
    }

    let file = File::open(file_path)?;

    let reader = BufReader::new(file);

    Ok(reader)
}

async fn create_listener(ip: &str) -> Result<TcpStream, Box<dyn Error>> {
    let addr = socket_address_from_string_ip(ip.to_string())?;
    let listener = create_stream(addr).await?;

    Ok(listener)
}

async fn write_file_name(stream: &mut TcpStream, file_path: &Path) {
    let file_name = file_path.file_name().unwrap().to_str().unwrap().to_string();

    // write the len of file name
    stream
        .write_all(&(file_name.len() as u32).to_be_bytes())
        .await
        .unwrap();

    // write the file name
    stream.write_all(file_name.as_bytes()).await.unwrap();
}

async fn read_chunk_from_file(
    stream: &mut TcpStream,
    reader: &mut BufReader<File>,
) -> Result<([u8; 1024], u32), Box<dyn Error>> {
    let mut buffer = [0; 1024];
    let read_bytes = reader.read(&mut buffer).unwrap();
    let read_bytes = read_bytes as u32;

    Ok((buffer, read_bytes))
}

async fn write_chunk(stream: &mut TcpStream, read_bytes: u32, chunk: &[u8; 1024]) {
    let len_bytes = &1024u32.to_be_bytes();

    if read_bytes < 1024 {
        stream.write_all(&read_bytes.to_be_bytes()).await.unwrap();
    } else {
        stream.write_all(len_bytes).await.unwrap();
    }
    stream.write_all(chunk).await.unwrap();
}

async fn write_EOF(stream: &mut TcpStream) {
    stream.write_all(&[0x00]).await.unwrap();
}
