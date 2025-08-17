use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::io::AsyncWriteExt;
use crate::server::server::Clients;

pub async fn broadcast(
    clients: &Clients,
    log_file: &Arc<Mutex<tokio::fs::File>>,
    message: &str,
    sender: Option<&str>
) {
    // Log message
    {
        let mut file = log_file.lock().await;
        file.write_all(message.as_bytes()).await.unwrap();
        file.write_all(b"\n").await.unwrap();
    }

    let clients_guard = clients.lock().await;
    for (name, client) in clients_guard.iter() {
        if Some(name.as_str()) != sender {
            let client = client.clone();
            let mut client = client.lock().await;
            client.write_all(message.as_bytes()).await.unwrap();
            client.write_all(b"\n").await.unwrap();
        }
    }
}
