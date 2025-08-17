use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex;
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::Message;
use futures::{StreamExt, SinkExt};
use crate::server::chat::{Clients, broadcast};
use std::collections::HashMap;
use serde_json::json;
use serde_json::Value;

pub async fn start_server(addr: &str) {
    let listener = TcpListener::bind(addr).await.unwrap();
    println!("WebSocket server listening on ws://{}", addr);

    let clients: Clients = Arc::new(Mutex::new(HashMap::new()));

    while let Ok((stream, _)) = listener.accept().await {
        let clients_clone = clients.clone();
        tokio::spawn(async move {
            if let Err(e) = handle_connection(stream, clients_clone).await {
                eprintln!("Connection error: {}", e);
            }
        });
    }
}

async fn handle_connection(stream: TcpStream, clients: Clients) -> anyhow::Result<()> {
    let peer = stream.peer_addr()?;
    println!("Incoming TCP connection from {}", peer);

    let ws_stream = match accept_async(stream).await {
        Ok(ws) => {
            println!("WebSocket handshake succeeded with {}", peer);
            ws
        }
        Err(err) => {
            eprintln!("WebSocket handshake failed with {}: {}", peer, err);
            return Err(err.into());
        }
    };

    // Split WebSocket into sender and receiver
    let (ws_sender, mut ws_receiver) = ws_stream.split();
    let ws_sender = Arc::new(Mutex::new(ws_sender));

    // Send system message for nickname prompt
    {
        let msg = json!({
            "type": "system",
            "content": "Enter your nickname:"
        });
        ws_sender.lock().await.send(Message::Text(msg.to_string())).await?;
    }

    // Read nickname
    let mut nickname = String::new();
    if let Some(msg_result) = ws_receiver.next().await {
        match msg_result {
            Ok(Message::Text(txt)) => {
                // parse JSON
                let data: serde_json::Value = serde_json::from_str(&txt)?;
                if data["type"] == "nickname" {
                    nickname = data["name"].as_str().unwrap_or("Unknown").to_string();
                } else {
                    eprintln!("Expected nickname message from {}", peer);
                    return Err(anyhow::anyhow!("Expected nickname message"));
                }
            }
            _ => return Err(anyhow::anyhow!("Failed to read nickname")),
        }
    }

    println!("{} connected as '{}'", peer, nickname);

    // Add client for broadcasting
    {
        let mut clients_guard = clients.lock().await;
        clients_guard.insert(nickname.clone(), ws_sender.clone());
    }

    broadcast(&clients, "Server", &format!("{} has joined!", nickname)).await;

    // Listen for messages from this client
    while let Some(msg) = ws_receiver.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                // Parse incoming JSON
                if let Ok(data) = serde_json::from_str::<Value>(&text) {
                    if data["type"] == "message" {
                        let content = data["content"].as_str().unwrap_or("");
                        println!("Received from {}: {}", nickname, content);
                        broadcast(&clients, &nickname, content).await;
                    } else {
                        println!("Unknown message type from {}: {:?}", nickname, data);
                    }
                } else {
                    println!("Invalid JSON from {}: {}", nickname, text);
                }
            }
            Ok(Message::Close(_)) => {
                println!("{} disconnected", nickname);
                break;
            }
            Ok(msg) => {
                println!("Received non-text message from {}: {:?}", nickname, msg);
            }
            Err(err) => {
                eprintln!("Error receiving message from {}: {}", nickname, err);
                break;
            }
        }
    }

    // Remove client and broadcast leaving
    {
        let mut clients_guard = clients.lock().await;
        clients_guard.remove(&nickname);
    }
    broadcast(&clients, "Server", &format!("{} has left!", nickname)).await;

    Ok(())
}
