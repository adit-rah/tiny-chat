use clap::Parser;

mod server;

#[derive(Parser)]
struct Args {
    #[arg(short, long, default_value_t = 6001)]
    port: u16,

    #[arg(short, long)]
    password: Option<String>,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    println!("Starting chat server...");
    server::ws_server::start_ws_server(args.port, args.password).await;
}
