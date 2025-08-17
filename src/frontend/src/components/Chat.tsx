import React, { useState, useEffect } from "react";
import { useChat, ChatMessage } from "../hooks/useChat";

interface ChatProps {
  serverUrl: string;
}

export const Chat: React.FC<ChatProps> = ({ serverUrl }) => {
  const { messages, sendMessage, sendNickname } = useChat(serverUrl);
  const [input, setInput] = useState("");
  const [nickname, setNickname] = useState<string | null>(null);

  // Automatically send nickname once
  useEffect(() => {
    if (nickname) {
      sendNickname(nickname);
    }
  }, [nickname, sendNickname]);

  const handleSend = () => {
    if (!input.trim()) return;
    sendMessage(input.trim());
    setInput("");
  };

  return (
    <div style={{ maxWidth: 400, margin: "0 auto" }}>
      {!nickname && (
        <div>
          <input
            type="text"
            placeholder="Enter your nickname"
            value={input}
            onChange={(e) => setInput(e.target.value)}
            onKeyDown={(e) => e.key === "Enter" && setNickname(input.trim())}
          />
          <button onClick={() => setNickname(input.trim())}>Set Nickname</button>
        </div>
      )}

      {nickname && (
        <div>
          <div
            style={{
              height: "300px",
              overflowY: "scroll",
              border: "1px solid gray",
              padding: "8px",
            }}
          >
            {messages.map((m: ChatMessage, idx) => (
            <div key={idx}>
                {m.type === "message" ? (
                <>
                    {m.sender && <strong>{m.sender}: </strong>}
                    {m.content}
                </>
                ) : (
                <em>{m.content}</em>
                )}
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
      )}
    </div>
  );
};
