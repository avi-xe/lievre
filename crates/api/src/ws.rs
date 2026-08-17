use std::collections::HashMap;
use std::sync::Arc;

use axum::extract::ws::{Message, WebSocket};
use futures_util::{SinkExt, StreamExt};
use serde_json::json;
use tokio::sync::{mpsc, RwLock};

/// Thread-safe connection manager for WebSocket clients
#[derive(Clone)]
pub struct ConnectionManager {
    /// user_id -> sender channel
    connections: Arc<RwLock<HashMap<String, mpsc::UnboundedSender<String>>>>,
}

impl ConnectionManager {
    pub fn new() -> Self {
        Self {
            connections: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Register a new WebSocket connection for a user
    pub async fn connect(&self, user_id: String, sender: mpsc::UnboundedSender<String>) {
        let mut conns = self.connections.write().await;
        // If user already connected, close old connection by dropping sender
        conns.insert(user_id.clone(), sender);
        tracing::debug!("WebSocket connected: {}", user_id);
    }

    /// Remove a user's WebSocket connection
    pub async fn disconnect(&self, user_id: &str) {
        let mut conns = self.connections.write().await;
        conns.remove(user_id);
        tracing::debug!("WebSocket disconnected: {}", user_id);
    }

    /// Send a notification to a specific user
    pub async fn send_to_user(&self, user_id: &str, notification: &serde_json::Value) {
        let conns = self.connections.read().await;
        if let Some(sender) = conns.get(user_id) {
            let msg = serde_json::to_string(notification).unwrap_or_default();
            if sender.send(msg).is_err() {
                tracing::warn!("Failed to send to user {}: channel closed", user_id);
            }
        }
    }

    /// Broadcast a notification to multiple users
    pub async fn broadcast(&self, user_ids: &[String], notification: &serde_json::Value) {
        for user_id in user_ids {
            self.send_to_user(user_id, notification).await;
        }
    }

    /// Get count of connected users
    pub async fn connected_count(&self) -> usize {
        self.connections.read().await.len()
    }
}

/// Handle a single WebSocket connection
pub async fn handle_socket(socket: WebSocket, user_id: String, manager: ConnectionManager) {
    let (mut sender, mut receiver) = socket.split();

    // Create channel for this connection
    let (tx, mut rx) = mpsc::unbounded_channel::<String>();

    // Register connection
    manager.connect(user_id.clone(), tx).await;

    // Send welcome message
    let welcome = json!({
        "type": "connected",
        "message": "WebSocket connected"
    });
    let _ = sender.send(Message::Text(welcome.to_string().into())).await;

    // Spawn task to forward messages from channel to WebSocket
    let mut send_task = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if sender.send(Message::Text(msg.into())).await.is_err() {
                break;
            }
        }
    });

    // Spawn task to handle incoming messages (ping/pong, close)
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            match msg {
                Message::Close(_) => break,
                Message::Text(text) => {
                    // Client can send ping messages
                    if text == "ping" {
                        // pong is handled by tungstenite automatically
                    }
                }
                _ => {}
            }
        }
    });

    // Wait for either task to finish
    tokio::select! {
        _ = &mut send_task => recv_task.abort(),
        _ = &mut recv_task => send_task.abort(),
    }

    // Cleanup
    manager.disconnect(&user_id).await;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_connection_manager() {
        let manager = ConnectionManager::new();
        let (tx, _rx) = mpsc::unbounded_channel();

        manager.connect("user1".to_string(), tx).await;
        assert_eq!(manager.connected_count().await, 1);

        manager.disconnect("user1").await;
        assert_eq!(manager.connected_count().await, 0);
    }
}
