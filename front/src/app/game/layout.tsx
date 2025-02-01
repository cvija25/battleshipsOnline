import React from 'react';
import { WebSocketProvider } from '@/components/SocketContext';

export default function GameLayout({ children }: { children: React.ReactNode }) {
  return (
    <WebSocketProvider>
      <div>
        {children}
      </div>
    </WebSocketProvider>
  );
}
