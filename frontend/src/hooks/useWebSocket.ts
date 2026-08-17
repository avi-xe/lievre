import { useEffect, useRef, useCallback, useState } from "react";

interface Notification {
  id: string;
  type: "follow" | "like" | "comment" | "repost";
  actor: string;
  content: string;
  entity_type?: string;
  entity_id?: string;
  created_at: string;
}

interface WebSocketMessage {
  type: "connected" | "notification" | "error";
  data?: Notification;
  message?: string;
}

interface UseWebSocketOptions {
  token: string | null;
  onNotification?: (notification: Notification) => void;
  onConnect?: () => void;
  onDisconnect?: () => void;
}

interface UseWebSocketReturn {
  isConnected: boolean;
  lastNotification: Notification | null;
  notifications: Notification[];
  clearNotifications: () => void;
}

export function useWebSocket({
  token,
  onNotification,
  onConnect,
  onDisconnect,
}: UseWebSocketOptions): UseWebSocketReturn {
  const wsRef = useRef<WebSocket | null>(null);
  const [isConnected, setIsConnected] = useState(false);
  const [lastNotification, setLastNotification] = useState<Notification | null>(null);
  const [notifications, setNotifications] = useState<Notification[]>([]);
  const reconnectTimeoutRef = useRef<NodeJS.Timeout>();
  const reconnectAttemptsRef = useRef(0);

  const connect = useCallback(() => {
    if (!token || wsRef.current?.readyState === WebSocket.OPEN) {
      return;
    }

    // Determine WebSocket URL based on current protocol
    const protocol = window.location.protocol === "https:" ? "wss:" : "ws:";
    const wsUrl = `${protocol}//${window.location.host}/ws?token=${token}`;

    try {
      const ws = new WebSocket(wsUrl);
      wsRef.current = ws;

      ws.onopen = () => {
        console.log("[WebSocket] Connected");
        setIsConnected(true);
        reconnectAttemptsRef.current = 0;
        onConnect?.();
      };

      ws.onmessage = (event) => {
        try {
          const msg: WebSocketMessage = JSON.parse(event.data);

          switch (msg.type) {
            case "connected":
              console.log("[WebSocket] Server confirmed connection");
              break;

            case "notification":
              if (msg.data) {
                console.log("[WebSocket] Notification received:", msg.data);
                setLastNotification(msg.data);
                setNotifications((prev) => [msg.data!, ...prev]);
                onNotification?.(msg.data);
              }
              break;

            case "error":
              console.error("[WebSocket] Server error:", msg.message);
              break;
          }
        } catch (err) {
          console.error("[WebSocket] Failed to parse message:", err);
        }
      };

      ws.onclose = (event) => {
        console.log("[WebSocket] Disconnected:", event.code, event.reason);
        setIsConnected(false);
        wsRef.current = null;
        onDisconnect?.();

        // Reconnect with exponential backoff
        if (reconnectAttemptsRef.current < 5) {
          const delay = Math.min(1000 * Math.pow(2, reconnectAttemptsRef.current), 30000);
          console.log(`[WebSocket] Reconnecting in ${delay}ms (attempt ${reconnectAttemptsRef.current + 1})`);
          reconnectTimeoutRef.current = setTimeout(() => {
            reconnectAttemptsRef.current++;
            connect();
          }, delay);
        }
      };

      ws.onerror = (error) => {
        console.error("[WebSocket] Error:", error);
      };
    } catch (err) {
      console.error("[WebSocket] Failed to connect:", err);
    }
  }, [token, onNotification, onConnect, onDisconnect]);

  const disconnect = useCallback(() => {
    if (reconnectTimeoutRef.current) {
      clearTimeout(reconnectTimeoutRef.current);
    }
    if (wsRef.current) {
      wsRef.current.close();
      wsRef.current = null;
    }
    setIsConnected(false);
  }, []);

  const clearNotifications = useCallback(() => {
    setNotifications([]);
    setLastNotification(null);
  }, []);

  // Connect on mount, disconnect on unmount
  useEffect(() => {
    if (token) {
      connect();
    }

    return () => {
      disconnect();
    };
  }, [token, connect, disconnect]);

  // Send ping every 30 seconds to keep connection alive
  useEffect(() => {
    if (!isConnected || !wsRef.current) return;

    const interval = setInterval(() => {
      if (wsRef.current?.readyState === WebSocket.OPEN) {
        wsRef.current.send("ping");
      }
    }, 30000);

    return () => clearInterval(interval);
  }, [isConnected]);

  return {
    isConnected,
    lastNotification,
    notifications,
    clearNotifications,
  };
}
