# zifer
Transfer files over network using cli

## Download
**Curently only supported method is cargo**
- Cargo 
  - `cargo install zifer`
  
## How to download from server

1. Start the server using `zifer -s .`
    - Where `-s .` says to start in server mode in the current dircetory
2. Initialize download `zifer -c 127.0.0.1 -d -f /work/some-file.txt`
    - Where `-c` specifies its in client mode and as argument it expects an ip address
    - Where `-d` specifes that we are about to download
    - And `-f /work/some-file.txt` specifies the file name
    
## How to upload to server

1. Start the server using `zifer -s .`
    - Where `-s .` says to start in server mode in the current dircetory
2. Initialize upload to server `zifer -c 127.0.0.1 -u -f /work/some-file.txt`
    - Where `-c` specifies its in client mode and as argument it expects an ip address
    - Where `-u` specifes that we are about to download
    - And `-f /work/some-file.txt` specifies the file name

## Options
- `-s, --server <SERVER>`
- `-c, --client <CLIENT>` 
- `-f, --file <FILE>` 
- `-d, --download`         
- `-u, --upload`           
- `-h, --help`
- `-V, --version`
