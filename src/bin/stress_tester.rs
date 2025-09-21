use futures_util::{SinkExt, StreamExt};
use rand::Rng;
use std::sync::Arc;
use tokio::sync::Semaphore;
use tokio_tungstenite::connect_async;
use url::Url;

const CLIENTS: usize = 200;
const MESSAGES: usize = 50;
const INTERVAL_SECS: u64 = 5;

#[tokio::main]
async fn main() {
    let url = Url::parse("ws://127.0.0.1:8080/ws").unwrap();
    let semaphore = Arc::new(Semaphore::new(50)); // limit concurrency if needed

    let mut handles = vec![];

    for i in 0..CLIENTS {
        let url = url.clone();
        let permit = semaphore.clone().acquire_owned().await.unwrap();

        let handle = tokio::spawn(async move {
            let nickname = format!("User{}", rand::thread_rng().gen::<u16>());
            let (mut ws_stream, _) = connect_async(&url)
                .await
                .expect("Failed to connect");

            // Send nickname immediately
            ws_stream
                .send(tungstenite::Message::Text(nickname.clone()))
                .await
                .unwrap();
            println!("Client-{} sent nickname: {}", i, nickname);

            // Send messages in a loop
            for j in 0..MESSAGES {
                let msg = format!("Message {} from {}", j + 1, nickname);
                if ws_stream.send(tungstenite::Message::Text(msg)).await.is_err() {
                    println!("Client-{} disconnected", i);
                    break;
                }
                tokio::time::sleep(tokio::time::Duration::from_secs(INTERVAL_SECS)).await;
            }

            drop(permit);
        });

        handles.push(handle);
    }

    // Wait for all clients to finish
    for handle in handles {
        let _ = handle.await;
    }

    println!("Stress test completed!");
}
