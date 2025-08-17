mod server;
use server::ws_server::start_server;

#[tokio::main]
async fn main() {
    println!("Starting chat server...");
    start_server("0.0.0.0:6001").await;
}
