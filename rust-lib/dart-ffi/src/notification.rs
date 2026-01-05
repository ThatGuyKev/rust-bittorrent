use std::collections::HashMap;
use std::sync::Arc;
use parking_lot::RwLock;
use prost::Message;
use allo_isolate::{IntoDart, Isolate};

use crate::proto::notification;

pub type NotificationCallback = Box<dyn Fn(notification::Notification) + Send + Sync>;

/// Notification broadcaster - sends notifications from Rust to Flutter
pub struct NotificationBroadcaster {
    listeners: Arc<RwLock<HashMap<String, Vec<i64>>>>, // event_type -> port list
    dart_isolates: Arc<RwLock<HashMap<i64, Isolate>>>,
}

impl NotificationBroadcaster {
    pub fn new() -> Self {
        Self {
            listeners: Arc::new(RwLock::new(HashMap::new())),
            dart_isolates: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Subscribe a Dart isolate port to a specific notification type
    pub fn subscribe(&self, notification_type: String, port: i64) {
        let isolate = Isolate::new(port);
        self.dart_isolates.write().insert(port, isolate);
        
        let mut listeners = self.listeners.write();
        listeners
            .entry(notification_type)
            .or_insert_with(Vec::new)
            .push(port);
    }

    /// Unsubscribe a Dart isolate port
    pub fn unsubscribe(&self, port: i64) {
        self.dart_isolates.write().remove(&port);
        
        let mut listeners = self.listeners.write();
        for ports in listeners.values_mut() {
            ports.retain(|p| *p != port);
        }
    }

    /// Broadcast a notification to all subscribers
    pub fn broadcast(&self, notification_type: &str, payload: Vec<u8>) {
        let notification = notification::Notification {
            notification_type: notification_type.to_string(),
            payload,
        };

        let notification_bytes = notification.encode_to_vec();
        
        let listeners = self.listeners.read();
        if let Some(ports) = listeners.get(notification_type) {
            let isolates = self.dart_isolates.read();
            for port in ports {
                if let Some(isolate) = isolates.get(port) {
                    // Send notification bytes to Dart
                    let data: Vec<u8> = notification_bytes.clone();
                    let _ = isolate.post(data.into_dart());
                }
            }
        }
    }
}

impl Default for NotificationBroadcaster {
    fn default() -> Self {
        Self::new()
    }
}
