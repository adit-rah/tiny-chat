use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use futures::{SinkExt, StreamExt};
use tokio_tungstenite::tungstenite::Message;

pub async fn start_ws_server() {
    let listener = TcpListener::bind("127.0.0.1:6001").await.unwrap();
    println!("WebSocket server running on ws://127.0.0.1:6001");

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(handle_connection(stream));
    }
}

async fn handle_connection(stream: tokio::net::TcpStream) {
    let ws_stream = accept_async(stream).await.unwrap();
    println!("New WebSocket connection!");

    let (mut write, mut read) = ws_stream.split();

    while let Some(msg) = read.next().await {
        let msg = msg.unwrap();
        if msg.is_text() {
            let text = msg.to_text().unwrap();
            println!("Received: {}", text);

            // Echo it back for now
            write.send(Message::Text(text.into())).await.unwrap();
        }
    }
}
