use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::net::tcp::OwnedWriteHalf;

pub type Clients = Arc<Mutex<HashMap<String, Arc<Mutex<OwnedWriteHalf>>>>>;

pub async fn run_server() {
    let listener = TcpListener::bind("127.0.0.1:6000").await.unwrap();
    println!("Server running on 127.0.0.1:6000");

    let clients: Clients = Arc::new(Mutex::new(HashMap::new()));

    // Initialize async log file
    let log_file = init_log_file("chat_log.txt").await;
    let log_file = Arc::new(Mutex::new(log_file));

    loop {
        let (stream, _) = listener.accept().await.unwrap();
        let clients_clone = Arc::clone(&clients);
        let log_file_clone = Arc::clone(&log_file);

        tokio::spawn(async move {
            handle_client(stream, clients_clone, log_file_clone).await;
        });
    }
}
