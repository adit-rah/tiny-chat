import { useEffect, useState, useRef } from "react";

interface Message {
  sender: string;
  content: string;
}

export function useChat(serverUrl: string, password?: string) {
  const [messages, setMessages] = useState<Message[]>([]);
  const ws = useRef<WebSocket | null>(null);

  useEffect(() => {
    const url = new URL(serverUrl);
    ws.current = new WebSocket(url.toString());

    ws.current.onopen = () => {
      console.log("Connected to server");
      if (password) ws.current?.send(JSON.stringify({ type: "auth", password }));
    };

    ws.current.onmessage = (event) => {
      try {
        const data = JSON.parse(event.data);
        if (data.type === "message") {
          setMessages((prev) => [...prev, { sender: data.sender, content: data.content }]);
        }
      } catch (err) {
        console.error("Invalid message format", err);
      }
    };

    ws.current.onclose = () => {
      console.log("Disconnected from server");
    };

    return () => {
      ws.current?.close();
    };
  }, [serverUrl, password]);

  const sendMessage = (content: string) => {
    ws.current?.send(JSON.stringify({ type: "message", content }));
  };

  return { messages, sendMessage };
}
