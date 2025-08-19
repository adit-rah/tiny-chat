import { useEffect, useState, useRef, useCallback } from "react";

export interface ChatMessage {
  type: "message" | "system";
  sender?: string;
  content: string;
}

export function useChat(serverUrl: string | null) {
  const [messages, setMessages] = useState<ChatMessage[]>([]);
  const ws = useRef<WebSocket | null>(null);

  // Establish WebSocket connection
  useEffect(() => {
    if (!serverUrl) return; // 👈 skip if no serverUrl yet

    ws.current = new WebSocket(serverUrl);

    ws.current.onopen = () => {
      console.log("Connected to server:", serverUrl);
    };

    ws.current.onmessage = (event) => {
      try {
        const data = JSON.parse(event.data);

        // Only add recognized types
        if (data.type === "message" || data.type === "system") {
          setMessages((prev) => [...prev, data]);
        } else {
          console.warn("Unknown message type:", data);
        }
      } catch (err) {
        console.error("Invalid message format", err, "Raw data:", event.data);
      }
    };

    ws.current.onclose = () => {
      console.log("Disconnected from server:", serverUrl);
    };

    return () => ws.current?.close();
  }, [serverUrl]);

  // Send chat message
  const sendMessage = useCallback((content: string) => {
    if (!ws.current || ws.current.readyState !== WebSocket.OPEN) return;
    ws.current.send(JSON.stringify({ type: "message", content }));
  }, []);

  // Send nickname once
  const sendNickname = useCallback((nickname: string) => {
    if (!ws.current || ws.current.readyState !== WebSocket.OPEN) return;
    ws.current.send(JSON.stringify({ type: "nickname", name: nickname }));
  }, []);

  return { messages, sendMessage, sendNickname };
}
