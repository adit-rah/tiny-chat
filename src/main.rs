mod server;

#[tokio::main]
async fn main() {
    println!("Starting chat servers...");

    // Start WebSocket server
    tokio::spawn(server::ws_server::start_ws_server());

    // Start TCP server (CLI clients)
    server::server::start_tcp_server().await;
}
