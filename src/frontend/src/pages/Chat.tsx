import React, { useState, useEffect, useRef } from "react";
import { useChat, ChatMessage } from "../hooks/useChat";

export const Chat: React.FC = () => {
  const [serverIp, setServerIp] = useState("");
  const [serverPort, setServerPort] = useState("");
  const [serverUrl, setServerUrl] = useState<string | null>(null);

  const { messages, sendMessage, sendNickname } = useChat(serverUrl || "");
  const [input, setInput] = useState("");
  const [nickname, setNickname] = useState<string | null>(null);
  const messagesEndRef = useRef<HTMLDivElement | null>(null);

  // Automatically send nickname once
  useEffect(() => {
    if (nickname) sendNickname(nickname);
  }, [nickname, sendNickname]);

  // Auto scroll to bottom when messages update
  useEffect(() => {
    messagesEndRef.current?.scrollIntoView({ behavior: "smooth" });
  }, [messages]);

  const handleSend = () => {
    if (!input.trim()) return;
    sendMessage(input.trim());
    setInput("");
  };

  const renderMessage = (m: ChatMessage) => {
    const isUser = m.sender === nickname;
    const bubbleStyle: React.CSSProperties = {
      backgroundColor: isUser ? "#007bff" : "#e5e5ea",
      color: isUser ? "white" : "black",
      padding: "8px 12px",
      borderRadius: 16,
      marginBottom: 6,
      maxWidth: "75%",
      alignSelf: isUser ? "flex-end" : "flex-start",
      wordBreak: "break-word",
    };

    return (
      <div style={{ display: "flex", flexDirection: "column" }} key={Math.random()}>
        {m.type === "system" ? (
          <em style={{ textAlign: "center", color: "#666", margin: "4px 0" }}>
            {m.content}
          </em>
        ) : (
          <div style={bubbleStyle}>
            {!isUser && m.sender && <strong>{m.sender}: </strong>}
            {m.content}
          </div>
        )}
      </div>
    );
  };

  if (!serverUrl) {
    return (
      <div style={{ padding: 16, display: "flex", flexDirection: "column", gap: 8, maxWidth: 300, margin: "0 auto" }}>
        <input
          type="text"
          placeholder="Server IP (e.g. 192.168.1.10)"
          value={serverIp}
          onChange={(e) => setServerIp(e.target.value)}
          style={{ padding: 8, borderRadius: 4, border: "1px solid #ccc" }}
        />
        <input
          type="text"
          placeholder="Port (e.g. 8080)"
          value={serverPort}
          onChange={(e) => setServerPort(e.target.value)}
          style={{ padding: 8, borderRadius: 4, border: "1px solid #ccc" }}
        />
        <button
          onClick={() => {
            if (serverIp && serverPort) {
              setServerUrl(`ws://${serverIp}:${serverPort}`);
            }
          }}
          style={{ padding: 8, borderRadius: 4, backgroundColor: "#007bff", color: "white", border: "none" }}
        >
          Connect
        </button>
      </div>
    );
  }

  return (
    <div
      style={{
        width: "100%",
        maxWidth: 400,
        display: "flex",
        flexDirection: "column",
        margin: "0 auto",
        border: "1px solid #ccc",
        borderRadius: 8,
        overflow: "hidden",
      }}
    >
      {!nickname ? (
        <div style={{ padding: 16, display: "flex", flexDirection: "column", gap: 8 }}>
          <input
            type="text"
            placeholder="Enter your nickname"
            value={input}
            onChange={(e) => setInput(e.target.value)}
            onKeyDown={(e) => e.key === "Enter" && setNickname(input.trim())}
            style={{ padding: 8, borderRadius: 4, border: "1px solid #ccc" }}
          />
          <button
            onClick={() => setNickname(input.trim())}
            style={{ padding: 8, borderRadius: 4, backgroundColor: "#007bff", color: "white", border: "none" }}
          >
            Set Nickname
          </button>
        </div>
      ) : (
        <>
          <div
            style={{
              flex: 1,
              display: "flex",
              flexDirection: "column",
              padding: 12,
              gap: 4,
              height: 300,
              overflowY: "auto",
              backgroundColor: "#f7f7f7",
            }}
          >
            {messages.map(renderMessage)}
            <div ref={messagesEndRef} />
          </div>
          <div style={{ display: "flex", padding: 8, borderTop: "1px solid #ccc", gap: 8 }}>
            <input
              type="text"
              value={input}
              onChange={(e) => setInput(e.target.value)}
              onKeyDown={(e) => e.key === "Enter" && handleSend()}
              style={{ flex: 1, padding: 8, borderRadius: 4, border: "1px solid #ccc" }}
            />
            <button
              onClick={handleSend}
              style={{ padding: "8px 12px", borderRadius: 4, backgroundColor: "#007bff", color: "white", border: "none" }}
            >
              Send
            </button>
          </div>
        </>
      )}
    </div>
  );
};
