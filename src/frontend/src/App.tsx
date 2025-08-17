import React, { useState } from "react";
import { useChat } from "./hooks/useChat";

const CHAT_SERVER_URL = "ws://localhost:6001"; // update if server runs elsewhere

function App() {
  const [nickname, setNickname] = useState("");
  const [input, setInput] = useState("");
  const { messages, connected, connect, sendMessage } = useChat(CHAT_SERVER_URL);

  const handleConnect = () => {
    connect();
    if (nickname) sendMessage(nickname); // send nickname as first message
  };

  const handleSend = () => {
    if (input.trim()) {
      sendMessage(input);
      setInput("");
    }
  };

  if (!connected) {
    return (
      <div style={{ padding: "2rem" }}>
        <h1>Tiny Chat</h1>
        <input
          type="text"
          placeholder="Enter nickname"
          value={nickname}
          onChange={(e) => setNickname(e.target.value)}
        />
        <button onClick={handleConnect} disabled={!nickname}>
          Connect
        </button>
      </div>
    );
  }

  return (
    <div style={{ padding: "2rem" }}>
      <h1>Tiny Chat</h1>
      <p>Connected as: {nickname}</p>
      <div style={{ border: "1px solid #ccc", padding: "1rem", minHeight: "200px", marginBottom: "1rem" }}>
        {messages.map((msg, idx) => (
          <div key={idx}>{msg}</div>
        ))}
      </div>
      <input
        type="text"
        value={input}
        onChange={(e) => setInput(e.target.value)}
        onKeyDown={(e) => e.key === "Enter" && handleSend()}
        placeholder="Type a message..."
      />
      <button onClick={handleSend}>Send</button>
    </div>
  );
}

export default App;
