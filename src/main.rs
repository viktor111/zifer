use clap::Parser;

mod server;
mod client;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   /// Name of the person to greet
   #[arg(short, long, default_value_t)]
   server: String,

   #[arg(short, long, default_value_t)]
    client: bool
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    if args.server != ""  {
        println!("Server running in {} directory...", args.server);
    } else if args.client {
        println!("Client running...");
    }
}
