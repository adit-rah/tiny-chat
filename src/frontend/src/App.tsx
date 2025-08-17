import React from "react";
import { Chat } from "./components/Chat";

function App() {
  return (
    <div>
      <h1>Self-hosted Tiny Chat</h1>
      <Chat serverUrl="ws://localhost:6001" />
    </div>
  );
}

export default App;
