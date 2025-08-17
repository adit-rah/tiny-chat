import React from "react";
import { Link } from "react-router-dom";

export const Home: React.FC = () => {
  return (
    <div style={{ textAlign: "center", marginTop: "50px" }}>
      <h1>Welcome to Tiny Chat!</h1>
      <p>A simple chat app to connect with friends.</p>
      <Link
        to="/chat"
        style={{
          display: "inline-block",
          marginTop: 20,
          padding: "12px 24px",
          backgroundColor: "#007bff",
          color: "white",
          borderRadius: 6,
          textDecoration: "none",
        }}
      >
        Go to Chat
      </Link>
    </div>
  );
};
