mod server;
mod frontend;

use server::server::run_server;

#[tokio::main]
async fn main() {
    // Start the server
    println!("{}", frontend::display::info("Starting chat server..."));
    run_server().await;
}
