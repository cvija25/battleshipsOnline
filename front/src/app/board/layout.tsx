import React from 'react';
import { WebSocketProvider } from '@/components/SocketContext';

export default function BoradLayout({ children }: { children: React.ReactNode }) {
  return (
    <WebSocketProvider>
      <div>
        {children}
      </div>
    </WebSocketProvider>
  );
}