mod proto;
mod notification;
mod handlers;
mod torrent_service;

use once_cell::sync::Lazy;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use proto::EventDispatcher;
use notification::NotificationBroadcaster;
use torrent_service::TorrentService;

// Global dispatcher, broadcaster, and torrent service
static TORRENT_SERVICE: Lazy<TorrentService> = Lazy::new(TorrentService::new);

static DISPATCHER: Lazy<EventDispatcher> = Lazy::new(|| {
    let dispatcher = EventDispatcher::new();
    handlers::register_handlers(&dispatcher);
    dispatcher
});

static BROADCASTER: Lazy<NotificationBroadcaster> = Lazy::new(NotificationBroadcaster::new);

/// Initialize the FFI system
#[no_mangle]
pub unsafe extern "C" fn init_dart_ffi() {
    // Force initialization of lazy statics
    let _ = &*DISPATCHER;
    let _ = &*BROADCASTER;
}

/// Dispatch an event (Flutter -> Rust)
#[no_mangle]
pub unsafe extern "C" fn dispatch_event(event_bytes_ptr: *const u8, event_len: usize) -> *mut c_char {
    let event_bytes = std::slice::from_raw_parts(event_bytes_ptr, event_len).to_vec();

    let response_bytes = DISPATCHER.dispatch(event_bytes);
    
    // Convert response to base64 for easier FFI transfer
    let base64_response = base64_encode(&response_bytes);
    
    CString::new(base64_response)
        .unwrap()
        .into_raw()
}

/// Subscribe to notifications (Flutter registers interest)
#[no_mangle]
pub unsafe extern "C" fn subscribe_notification(
    notification_type_ptr: *const c_char,
    dart_port: i64,
) {
    let notification_type = CStr::from_ptr(notification_type_ptr)
        .to_string_lossy()
        .into_owned();

    BROADCASTER.subscribe(notification_type, dart_port);
}

/// Unsubscribe from notifications
#[no_mangle]
pub unsafe extern "C" fn unsubscribe_notification(dart_port: i64) {
    BROADCASTER.unsubscribe(dart_port);
}

/// Free a string allocated by Rust
#[no_mangle]
pub unsafe extern "C" fn free_rust_string(ptr: *mut c_char) {
    if !ptr.is_null() {
        let _ = CString::from_raw(ptr);
    }
}

// Helper to get broadcaster for handlers
pub fn get_broadcaster() -> &'static NotificationBroadcaster {
    &BROADCASTER
}

// Helper to get torrent service for handlers
pub fn get_torrent_service() -> &'static TorrentService {
    &TORRENT_SERVICE
}

// Simple base64 encoding without external dependency
fn base64_encode(bytes: &[u8]) -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut result = String::new();
    
    for chunk in bytes.chunks(3) {
        let mut buf = [0u8; 3];
        for (i, &byte) in chunk.iter().enumerate() {
            buf[i] = byte;
        }
        
        result.push(CHARSET[(buf[0] >> 2) as usize] as char);
        result.push(CHARSET[(((buf[0] & 0x03) << 4) | (buf[1] >> 4)) as usize] as char);
        
        if chunk.len() > 1 {
            result.push(CHARSET[(((buf[1] & 0x0F) << 2) | (buf[2] >> 6)) as usize] as char);
        } else {
            result.push('=');
        }
        
        if chunk.len() > 2 {
            result.push(CHARSET[(buf[2] & 0x3F) as usize] as char);
        } else {
            result.push('=');
        }
    }
    
    result
}
