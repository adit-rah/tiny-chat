use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_tungstenite::tungstenite::Message;
use tokio::net::TcpStream;
use tokio_tungstenite::WebSocketStream;
use futures::SinkExt;

/// Type alias for shared chat clients
pub type Clients = Arc<Mutex<HashMap<String, Arc<Mutex<WebSocketStream<TcpStream>>>>>>;

/// Broadcast a message to all clients except optionally the sender
pub async fn broadcast(clients: &Clients, message: &str, sender: Option<&str>) {
    let clients_guard = clients.lock().await;

    for (nickname, ws) in clients_guard.iter() {
        if let Some(s) = sender {
            if s == nickname {
                continue;
            }
        }
        let mut ws_guard = ws.lock().await;
        let _ = ws_guard.send(Message::Text(message.to_string())).await;
    }
}
