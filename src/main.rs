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
            info!("Client connecting to {}.... for upload", args.client);
            let ip = format!("{}:7677", args.client);
            client::init::init_client_upload(&ip, &args.file)
                .await
                .unwrap();
        }
        else if args.download {
            // TODO: call init_client_download
        }
        else{
            error!("Need to specify --upload/-u or --download/-d")
        }
    }
}
