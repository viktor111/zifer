use clap::Parser;
use tracing::info;

mod server;
mod client;
mod common;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   /// Name of the person to greet
   #[arg(short, long, default_value_t)]
   server: String,

   #[arg(short, long, default_value_t)]
    client: String,

    #[arg(short, long, default_value_t)]
    file: String
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    if args.server != ""  {
        info!("[.] Server starting....");
        server::init::init_server("127.0.0.1:7677", &args.server).await.unwrap();
    } 
    else if args.client != "" {
        info!("Client connecting to {}....", args.client);
        client::init::init_client("127.0.0.1:7677", &args.file).await.unwrap();
    }
}
