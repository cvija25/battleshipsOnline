import React from 'react';
import { AuthProvider } from '@/components/AuthContext';

export default function ProfileLayout({ children }: { children: React.ReactNode }) {
  return (
    <AuthProvider>
      <div>
        {children}
      </div>
    </AuthProvider>
  );
}
