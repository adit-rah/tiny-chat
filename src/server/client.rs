use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;
use std::sync::Arc;
use tokio::sync::Mutex;
use colored::*;

use crate::server::broadcast::broadcast;
use crate::server::server::Clients;

pub async fn handle_client(
    stream: TcpStream,
    clients: Clients,
    log_file: Arc<Mutex<tokio::fs::File>>,
) {
    // split stream into read and write halves
    let (reader_half, writer_half) = stream.into_split();
    let writer_half = Arc::new(Mutex::new(writer_half));
    let mut reader = BufReader::new(reader_half);
    let mut line = String::new();

    // helper to write to client
    async fn write_to_client(client: Arc<Mutex<tokio::net::tcp::OwnedWriteHalf>>, msg: &str) {
        let mut locked = client.lock().await;
        let _ = locked.write_all(msg.as_bytes()).await;
        let _ = locked.write_all(b"\n").await;
    }

    // ask for nickname
    {
        let mut writer = writer_half.lock().await;
        writer.write_all(b"Enter your nickname:\n").await.unwrap();
    }

    reader.read_line(&mut line).await.unwrap();
    let nickname = line.trim().to_string();
    line.clear();

    // register client
    {
        let mut clients_guard = clients.lock().await;
        clients_guard.insert(nickname.clone(), writer_half.clone());
    }

    let join_msg = format!("{} has joined!", nickname);
    println!("{}", join_msg.green());
    broadcast(&clients, &log_file, &join_msg, Some(&nickname)).await;

    // main message loop
    loop {
        line.clear();
        let bytes_read = reader.read_line(&mut line).await;

        match bytes_read {
            Ok(0) => break, // connection closed
            Ok(_) => {
                let message = match std::str::from_utf8(line.as_bytes()) {
                    Ok(m) => m.trim(),
                    Err(_) => {
                        eprintln!("Received invalid UTF-8 from {}", nickname);
                        continue;
                    }
                };

                if message.starts_with('/') {
                    // Handle commands
                    match message {
                        "/list" => {
                            let clients_guard = clients.lock().await;
                            if let Some(client) = clients_guard.get(&nickname) {
                                let names: Vec<&String> = clients_guard.keys().collect();
                                write_to_client(client.clone(), &format!("Connected users: {:?}\n", names)).await;
                            }
                        }
                        _ => {
                            let clients_guard = clients.lock().await;
                            if let Some(client) = clients_guard.get(&nickname) {
                                write_to_client(client.clone(), "Unknown command").await;
                            }
                        }
                    }
                } else {
                    // Broadcast normal message
                    let formatted = format!("{}: {}", nickname.blue(), message);
                    println!("{}", formatted);
                    broadcast(&clients, &log_file, &formatted, Some(&nickname)).await;
                }
            }
            Err(e) => {
                eprintln!("Error reading from {}: {}", nickname, e);
                break;
            }
        }
    }

    // client disconnected
    let leave_msg = format!("{} has left!", nickname);
    println!("{}", leave_msg.red());
    broadcast(&clients, &log_file, &leave_msg, Some(&nickname)).await;
    clients.lock().await.remove(&nickname);
}
