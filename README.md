# Tiny Chat

Tiny Chat is a lightweight real-time chat application built with **Rust** for the backend and **WebSockets**, with a frontend interface to communicate with the server. It supports multiple clients, stress testing, and latency measurement.

---

## Features

* Real-time messaging using WebSockets
* Multiple client support
* Stress testing with configurable clients and message intervals
* Message latency measurement
* Simple, lightweight architecture

---

## Backend

The backend is written in **Rust** and handles WebSocket connections for chat clients.

### Requirements

* Rust 1.72+
* Cargo

### Running the Server

1. Clone the repository:

```bash
git clone https://github.com/adit-rah/tiny-chat.git
cd tiny-chat
```

2. Build and run the server:

```bash
cargo run --release --bin server
```

The server will start listening on `ws://127.0.0.1:8080/ws`.

---

## Stress Tester

You can test the server's capacity using the included stress tester.

### Running the Stress Tester

```bash
cargo run --release --bin stress_tester -- \
  --url ws://127.0.0.1:8080/ws \
  --clients 200 \
  --messages 50 \
  --interval 5
```

* `--clients`: Number of simulated clients
* `--messages`: Number of messages each client sends
* `--interval`: Interval (in seconds) between messages

### Latency Testing

The stress tester can also measure latency per message. Modify the stress tester to record timestamps when messages are sent and received.

---

## Frontend

The frontend is a simple web interface that connects to the WebSocket server.

### Requirements

* Node.js 20+
* npm or yarn

### Setup

```bash
cd frontend
npm install
```

### Running the Frontend

```bash
npm run start
```

The frontend will start at `http://localhost:5173` (or the port displayed in the terminal). Open it in your browser, enter a nickname, and start chatting.

### Features

* Connects to the backend WebSocket server
* Send and receive messages in real-time
* Simple UI for multiple users

---

## Contributing

Contributions are welcome! Please open issues or submit pull requests for improvements or bug fixes.

---

## License

MIT License
