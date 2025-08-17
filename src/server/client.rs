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
    log_file: Arc<Mutex<tokio::fs::File>>
) {
    let (reader_half, writer_half) = stream.into_split();
    let writer_half = Arc::new(Mutex::new(writer_half)); // store write half
    let mut reader = BufReader::new(reader_half);
    let mut line = String::new();

    // Ask for nickname
    {
        let mut writer = writer_half.lock().await;
        writer.write_all(b"Enter your nickname:\n").await.unwrap();
    }

    reader.read_line(&mut line).await.unwrap();
    let nickname = line.trim().to_string();
    line.clear();

    {
        let mut clients_guard = clients.lock().await;
        clients_guard.insert(nickname.clone(), writer_half.clone());
    }

    let join_msg = format!("{} has joined!", nickname);
    println!("{}", join_msg.green());
    broadcast(&clients, &log_file, &join_msg, Some(&nickname)).await;

    loop {
        line.clear();
        
        // byte error handling (skips invalid utf)
        let bytes_read = reader.read_line(&mut line).await;

        match bytes_read {
            Ok(0) => break, // connection closed
            Ok(_) => {
                // skip invalid UTF-8
                if let Ok(message) = std::str::from_utf8(line.as_bytes()) {
                    let message = message.trim();

                    if message.starts_with('/') {
                        match message {
                            "/list" => {
                                let clients_guard = clients.lock().await;
                                let client = clients_guard.get(&nickname).unwrap().clone();
                                let mut client = client.lock().await;
                                let names: Vec<&String> = clients_guard.keys().collect();
                                client.write_all(format!("Connected users: {:?}\n", names).as_bytes()).await.unwrap();
                            }
                            _ => {
                                let clients_guard = clients.lock().await;
                                let client = clients_guard.get(&nickname).unwrap().clone();
                                let mut client = client.lock().await;
                                client.write_all(b"Unknown command\n").await.unwrap();
                            }
                        }
                    } else {
                        let formatted = format!("{}: {}", nickname.blue(), message);
                        println!("{}", formatted);
                        broadcast(&clients, &log_file, &formatted, Some(&nickname)).await;
                    }
                } else {
                    eprintln!("Received invalid UTF-8 from {}", nickname);
                    continue;
                }
            }
            Err(e) => {
                eprintln!("Error reading from {}: {}", nickname, e);
                break;
            }
        }
    }

    let leave_msg = format!("{} has left!", nickname);
    println!("{}", leave_msg.red());
    broadcast(&clients, &log_file, &leave_msg, Some(&nickname)).await;
    clients.lock().await.remove(&nickname);
}
