use std::env;
use std::process;

mod server;
use server::ws_server::start_server;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    // Usage: ./server [host] [port]
    // Example: ./server 0.0.0.0 8080
    let (host, port) = match args.len() {
        1 => ("127.0.0.1".to_string(), "8080".to_string()),  // default
        2 => ("127.0.0.1".to_string(), args[1].clone()),     // only port given
        3 => (args[1].clone(), args[2].clone()),             // host + port
        _ => {
            eprintln!("Usage: {} [host] [port]", args[0]);
            process::exit(1);
        }
    };

    let addr = format!("{}:{}", host, port);

    println!("Starting chat server on {}...", addr);
    start_server(&addr).await;
}
