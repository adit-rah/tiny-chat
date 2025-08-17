import { useEffect, useRef, useState } from "react";

export function useChat(url: string) {
  const ws = useRef<WebSocket | null>(null);
  const [messages, setMessages] = useState<string[]>([]);
  const [connected, setConnected] = useState(false);

  const sendMessage = (msg: string) => {
    if (ws.current && ws.current.readyState === WebSocket.OPEN) {
      ws.current.send(msg);
    }
  };

  const connect = () => {
    ws.current = new WebSocket(url);

    ws.current.onopen = () => {
      setConnected(true);
      console.log("Connected to chat server");
    };

    ws.current.onmessage = (event) => {
      setMessages((prev) => [...prev, event.data]);
    };

    ws.current.onclose = () => {
      setConnected(false);
      console.log("Disconnected from chat server");
    };

    ws.current.onerror = (err) => {
      console.error("WebSocket error:", err);
    };
  };

  return { messages, connected, connect, sendMessage };
}
