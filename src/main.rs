use clap::Parser;
use tracing::{error, info, Level};

mod client;
mod common;
mod server;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t)]
    server: String,

    #[arg(short, long, default_value_t)]
    client: String,

    #[arg(short, long, default_value_t)]
    file: String,

    #[arg(short, long, default_value_t)]
    download: bool,

    #[arg(short, long, default_value_t)]
    upload: bool,
}

#[tokio::main]
async fn main() {
    tracing::subscriber::set_global_default(
        tracing_subscriber::FmtSubscriber::builder()
            .with_max_level(Level::INFO)
            .finish(),
    )
    .unwrap();

    let args = Args::parse();
    if args.server != "" {
        
        info!("Server starting....");
        server::init::init_server("127.0.0.1:7677", &args.server)
            .await
            .unwrap();
    } 
    else if args.client != "" {
        if args.upload {
            let ip = format!("{}:{}", args.client, 7677);
            info!("Client connecting to {}.... for upload", ip);
            client::init::init_client_upload(&ip, &args.file)
                .await
                .unwrap();
        }
        else if args.download {
            let ip = format!("{}:{}", args.client, 7677);
            info!("Client connecting to {}.... for download", ip);
            client::init::init_client_download(&ip, &args.file)
                .await
                .unwrap();
        }
        else{
            error!("Need to specify --upload/-u or --download/-d")
        }
    }
}
