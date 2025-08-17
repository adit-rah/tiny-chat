import React from "react";
import { Chat } from "./components/Chat";

function App() {
  return (
    <div>
      <h1>Self-hosted Tiny Chat</h1>
      <Chat serverUrl="wss://localhost:6001" password="optional-password" />
    </div>
  );
}

export default App;
