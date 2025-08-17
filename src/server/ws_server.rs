use crate::server::chat::{Clients, broadcast};
use tokio::net::TcpListener;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::accept_async;
use std::sync::Arc;
use tokio::sync::Mutex;
use futures::{SinkExt, StreamExt};
use std::collections::HashMap;

pub async fn start_ws_server(port: u16, password: Option<String>) {
    let addr = format!("0.0.0.0:{}", port);
    let listener = TcpListener::bind(&addr).await.expect("Failed to bind");

    println!("WebSocket server listening on ws://{}", addr);

    let clients: Clients = Arc::new(Mutex::new(HashMap::new()));

    while let Ok((stream, _)) = listener.accept().await {
        let clients = Arc::clone(&clients);
        let password = password.clone();

        tokio::spawn(async move {
            if let Err(e) = handle_connection(stream, clients, password).await {
                eprintln!("Connection error: {}", e);
            }
        });
    }
}

async fn handle_connection(stream: tokio::net::TcpStream, clients: Clients, password: Option<String>) -> anyhow::Result<()> {
    let ws_stream = accept_async(stream).await?;
    let ws = Arc::new(Mutex::new(ws_stream));

    // Prompt for nickname
    let nickname = {
        let mut ws_guard = ws.lock().await;
        ws_guard.send(Message::Text("Enter your nickname:".to_string())).await?;

        let msg = ws_guard.next().await
            .ok_or_else(|| anyhow::anyhow!("Client disconnected"))??;

        match msg {
            Message::Text(txt) => txt.trim().to_string(),
            _ => "Anonymous".to_string(),
        }
    };

    // Optional password auth
    if let Some(ref pw) = password {
        let mut ws_guard = ws.lock().await;
        ws_guard.send(Message::Text("Enter server password:".to_string())).await?;

        let auth_msg = ws_guard.next().await
            .ok_or_else(|| anyhow::anyhow!("Client disconnected"))??;

        let client_pw = match auth_msg {
            Message::Text(txt) => txt,
            _ => "".to_string(),
        };

        if client_pw != *pw {
            ws_guard.send(Message::Text("Invalid password. Disconnecting.".to_string())).await?;
            return Ok(());
        }
    }

    // Register client
    clients.lock().await.insert(nickname.clone(), Arc::clone(&ws));

    let join_msg = format!("{} has joined!", nickname);
    println!("{}", join_msg);
    broadcast(&clients, &join_msg, Some(&nickname)).await;

    // Read messages
    let clients_clone = Arc::clone(&clients);
    let nickname_clone = nickname.clone();
    loop {
        let msg = {
            let mut ws_guard = ws.lock().await;
            ws_guard.next().await
        };

        let msg = match msg {
            Some(Ok(Message::Text(txt))) => txt,
            Some(Ok(Message::Close(_))) | None => break,
            _ => continue,
        };

        if msg.starts_with('/') {
            // Only /list for now
            if msg.trim() == "/list" {
                let clients_guard = clients.lock().await;
                let names: Vec<&String> = clients_guard.keys().collect();
                let response = format!("Connected users: {:?}", names);
                let mut ws_guard = ws.lock().await;
                let _ = ws_guard.send(Message::Text(response)).await;
            }
        } else {
            let formatted = format!("{}: {}", nickname_clone, msg);
            println!("{}", formatted);
            broadcast(&clients_clone, &formatted, Some(&nickname_clone)).await;
        }
    }

    clients.lock().await.remove(&nickname_clone);
    let leave_msg = format!("{} has left!", nickname_clone);
    println!("{}", leave_msg);
    broadcast(&clients, &leave_msg, None).await;

    Ok(())
}
