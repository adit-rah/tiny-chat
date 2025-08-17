mod server;

use server::server::run_server;

#[tokio::main]
async fn main() {
    // Start the server
    run_server().await;
}
