use std::{error::Error, fs::File, path::Path, io::Write};

use tokio::{io::AsyncReadExt, net::TcpStream};

use crate::common::helpers;

pub async fn init_server(ip: &str, dir: &str) -> Result<(), Box<dyn Error>> {
    let path = Path::new(dir);

    validate_path(path)?;

    let socket_addr = helpers::socket_address_from_string_ip(ip.to_string()).expect("Invalid IP address");
    let listener = helpers::create_listener(socket_addr).await.unwrap();
    
    
    loop{
        let (stream, _) = helpers::listener_accept_conn(&listener).await.unwrap();
        
        handle_connection(stream, dir).await;
    }
}

async fn handle_connection(mut stream: TcpStream, dir: &str) {
    tokio::spawn(async move {
        loop {
            let file_name = read_file_name(&mut stream).await.unwrap();

            let mut file = create_file(&file_name).unwrap();

            loop{
                let chunk = read_chunk(&mut stream).await.unwrap();

                match chunk {
                    Some(chunk) => write_chunk_to_file(&mut file, &chunk).await,
                    None => return,
                }
            }
        }
    });
}

fn validate_path(path: &Path) -> Result<(), Box<dyn Error>> {
    if !path.is_dir() {
        return Err("[-] Not a directory".into());
    }

    Ok(())
}


async fn read_file_name(stream: &mut TcpStream) -> Result<String, Box<dyn Error>> {
    let mut file_name_len_bytes = [0; 4];
    stream.read_exact(&mut file_name_len_bytes).await.unwrap();

    let file_name_len: u32 = u32::from_be_bytes(file_name_len_bytes);

    let mut file_name_bytes = vec![0; file_name_len as usize];
    stream.read_exact(&mut file_name_bytes).await.unwrap();

    let file_name = String::from_utf8(file_name_bytes.to_vec()).unwrap();

    Ok(file_name)
}

fn create_file(file_name: &str) -> Result<File, Box<dyn Error>> {
    let file = std::fs::File::create(&file_name).unwrap();
    Ok(file)
}

async fn read_chunk(stream: &mut TcpStream) -> Result<Option<Vec<u8>>, Box<dyn Error>> {
    let mut chunk_len_bytes = [0; 4];
    stream.read_exact(&mut chunk_len_bytes).await.unwrap();
    let chunk_len = u32::from_be_bytes(chunk_len_bytes);

    if chunk_len == 0 {
        return Ok(None)
    }

    let mut frame_data = vec![0; chunk_len as usize];
    stream.read_exact(&mut frame_data).await.unwrap();

    Ok(Some(frame_data))
}

async fn write_chunk_to_file(file: &mut File, chunk: &Vec<u8>){
    file.write_all(chunk).unwrap();
}
