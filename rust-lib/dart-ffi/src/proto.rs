// Generated protobuf modules
pub mod event {
    include!(concat!(env!("OUT_DIR"), "/event.rs"));
}

pub mod notification {
    include!(concat!(env!("OUT_DIR"), "/notification.rs"));
}

use std::collections::HashMap;
use std::sync::Arc;
use parking_lot::RwLock;
use prost::Message;

pub type EventHandler = Box<dyn Fn(Vec<u8>) -> Result<Vec<u8>, String> + Send + Sync>;

/// Event dispatcher - handles incoming events from Flutter
pub struct EventDispatcher {
    handlers: Arc<RwLock<HashMap<String, EventHandler>>>,
}

impl EventDispatcher {
    pub fn new() -> Self {
        Self {
            handlers: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Register an event handler
    pub fn register<F>(&self, event_type: &str, handler: F)
    where
        F: Fn(Vec<u8>) -> Result<Vec<u8>, String> + Send + Sync + 'static,
    {
        self.handlers.write().insert(event_type.to_string(), Box::new(handler));
    }

    /// Dispatch an event
    pub fn dispatch(&self, event_bytes: Vec<u8>) -> Vec<u8> {
        let event = match event::Event::decode(&event_bytes[..]) {
            Ok(e) => e,
            Err(e) => {
                return self.create_error_response(&format!("Failed to decode event: {}", e));
            }
        };

        let handlers = self.handlers.read();
        match handlers.get(&event.event_type) {
            Some(handler) => match handler(event.payload) {
                Ok(data) => {
                    let response = event::EventResponse {
                        success: true,
                        data,
                        error: String::new(),
                    };
                    response.encode_to_vec()
                }
                Err(error) => self.create_error_response(&error),
            },
            None => self.create_error_response(&format!("No handler for event: {}", event.event_type)),
        }
    }

    fn create_error_response(&self, error: &str) -> Vec<u8> {
        let response = event::EventResponse {
            success: false,
            data: vec![],
            error: error.to_string(),
        };
        response.encode_to_vec()
    }
}

impl Default for EventDispatcher {
    fn default() -> Self {
        Self::new()
    }
}
