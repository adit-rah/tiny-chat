import React, { useState } from "react";
import { useChat } from "../hooks/useChat";

interface ChatProps {
  serverUrl: string;
  password?: string;
}

export const Chat: React.FC<ChatProps> = ({ serverUrl, password }) => {
  const { messages, sendMessage } = useChat(serverUrl, password);
  const [input, setInput] = useState("");

  const handleSend = () => {
    if (!input.trim()) return;
    sendMessage(input.trim());
    setInput("");
  };

  return (
    <div>
      <div style={{ height: "300px", overflowY: "scroll", border: "1px solid gray" }}>
        {messages.map((m, idx) => (
          <div key={idx}>
            <strong>{m.sender}:</strong> {m.content}
          </div>
        ))}
      </div>
      <input
        type="text"
        value={input}
        onChange={(e) => setInput(e.target.value)}
        onKeyDown={(e) => e.key === "Enter" && handleSend()}
      />
      <button onClick={handleSend}>Send</button>
    </div>
  );
};
