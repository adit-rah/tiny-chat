use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::WebSocketStream;
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use futures::stream::SplitSink;
use std::sync::Arc;
use std::collections::HashMap;
use serde_json::json;
use futures::SinkExt;

// Clients now store the sender half of the split WebSocket
pub type Clients = Arc<Mutex<HashMap<String, Arc<Mutex<SplitSink<WebSocketStream<TcpStream>, Message>>>>>>;

/// Broadcast a message to all connected clients
pub async fn broadcast(clients: &Clients, sender: &str, message: &str) {
    let clients_guard = clients.lock().await;
    let msg = json!({
        "type": "message",
        "sender": sender,
        "content": message
    });

    for (_, client_ws) in clients_guard.iter() {
        let mut ws_guard = client_ws.lock().await;
        let _ = ws_guard.send(Message::Text(msg.to_string())).await;
    }
}
