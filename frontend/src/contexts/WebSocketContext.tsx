import { createContext, useContext, useState, useCallback, type ReactNode } from "react";
import { useWebSocket } from "@/hooks/useWebSocket";
import { useAuth } from "./useAuth";
import { ToastContainer } from "@/components/Toast";

interface Notification {
  id: string;
  type: "follow" | "like" | "comment" | "repost";
  actor: string;
  content: string;
  entity_type?: string;
  entity_id?: string;
  created_at: string;
}

interface WebSocketContextValue {
  isConnected: boolean;
  lastNotification: Notification | null;
  notifications: Notification[];
  unreadCount: number;
  clearNotifications: () => void;
  markAllRead: () => void;
}

const WebSocketContext = createContext<WebSocketContextValue | null>(null);

export function useWebSocketContext() {
  const context = useContext(WebSocketContext);
  if (!context) {
    throw new Error("useWebSocketContext must be used within WebSocketProvider");
  }
  return context;
}

interface WebSocketProviderProps {
  children: ReactNode;
}

export function WebSocketProvider({ children }: WebSocketProviderProps) {
  const { token } = useAuth();
  const [unreadCount, setUnreadCount] = useState(0);
  const [toastQueue, setToastQueue] = useState<Array<{
    id: string;
    type: "follow" | "like" | "comment" | "repost";
    actor: string;
    content: string;
  }>>([]);

  const handleNotification = useCallback((notification: Notification) => {
    setUnreadCount((prev) => prev + 1);

    // Add to toast queue
    setToastQueue((prev) => [
      ...prev.slice(-2), // Keep max 3 toasts
      {
        id: notification.id,
        type: notification.type,
        actor: notification.actor,
        content: notification.content,
      },
    ]);
  }, []);

  const handleConnect = useCallback(() => {
    console.log("[WebSocketProvider] Connected");
  }, []);

  const handleDisconnect = useCallback(() => {
    console.log("[WebSocketProvider] Disconnected");
  }, []);

  const { isConnected, lastNotification, notifications, clearNotifications } = useWebSocket({
    token,
    onNotification: handleNotification,
    onConnect: handleConnect,
    onDisconnect: handleDisconnect,
  });

  const markAllRead = useCallback(() => {
    setUnreadCount(0);
  }, []);

  const dismissToast = useCallback((id: string) => {
    setToastQueue((prev) => prev.filter((t) => t.id !== id));
  }, []);

  const value: WebSocketContextValue = {
    isConnected,
    lastNotification,
    notifications,
    unreadCount,
    clearNotifications,
    markAllRead,
  };

  return (
    <WebSocketContext.Provider value={value}>
      {children}
      <ToastContainer toasts={toastQueue} onDismiss={dismissToast} />
    </WebSocketContext.Provider>
  );
}
