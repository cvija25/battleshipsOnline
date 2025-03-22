"use client"

import React, { createContext, useContext, useEffect, useState } from 'react';

// Type for the WebSocket connection (you can extend this type based on your use case)
type WebSocketContextType = WebSocket | null;

const WebSocketContext = createContext<WebSocketContextType>(null);

interface WebSocketProviderProps {
  children: React.ReactNode;
}

export const WebSocketProvider: React.FC<WebSocketProviderProps> = ({ children }) => {
  const [ws, setWs] = useState<WebSocketContextType>(null);

  useEffect(() => {
    // Prevent creating a new WebSocket if one already exists
    if (ws) return;

    // Initialize WebSocket connection only once
    const socket = new WebSocket('ws://localhost:8000/ws');
    
    socket.onopen = () => {
      console.log('WebSocket connection established');
    };

    socket.onclose = () => {
      console.log('WebSocket connection closed');
    };

    // Set the WebSocket instance in state
    setWs(socket);

    // Clean up the WebSocket connection on component unmount
    return () => {
      if (socket) {
        socket.close();
      }
    };
  }, []);

  return (
    <WebSocketContext.Provider value={ws}>
      {children}
    </WebSocketContext.Provider>
  );
};

export const useWebSocket = (): WebSocketContextType => {
  const context = useContext(WebSocketContext);
  return context;
};
