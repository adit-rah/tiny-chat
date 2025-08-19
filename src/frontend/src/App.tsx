import React from "react";
import { BrowserRouter as Router, Routes, Route, Link } from "react-router-dom";
import { Chat } from "./pages/Chat";
import { Home } from "./pages/Home";
import { Host } from "./pages/Host";

const App: React.FC = () => {
  return (
    <Router>
      <div style={{ fontFamily: "Arial, sans-serif" }}>
        <nav
          style={{
            display: "flex",
            padding: "12px 24px",
            backgroundColor: "#007bff",
            color: "white",
            justifyContent: "space-between",
          }}
        >
          <div style={{ fontWeight: "bold" }}>Tiny Chat</div>
          <div style={{ display: "flex", gap: "16px" }}>
            <Link to="/" style={{ color: "white", textDecoration: "none" }}>Home</Link>
            <Link to="/chat" style={{ color: "white", textDecoration: "none" }}>Chat</Link>
            <Link to="/host" style={{ color: "white", textDecoration: "none" }}>Host</Link>
          </div>
        </nav>

        <div style={{ padding: "24px" }}>
          <Routes>
            <Route path="/" element={<Home />} />
            <Route path="/chat" element={<Chat />} />
            <Route path="/host" element={<Host />} />
          </Routes>
        </div>
      </div>
    </Router>
  );
};

export default App;
