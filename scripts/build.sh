#!/bin/bash

# Script to build and verify the Rust FFI library

set -e

echo "Building Rust library..."
cd rust-lib/dart-ffi

# Clean and build
cargo clean
cargo build --release

echo ""
echo "Checking exported symbols..."

if [ -f "target/release/libdart_ffi.so" ]; then
    echo "Library built successfully at: target/release/libdart_ffi.so"
    echo ""
    echo "Exported symbols:"
    nm -D target/release/libdart_ffi.so | grep -E "(init_dart_ffi|dispatch_event|subscribe_notification|unsubscribe_notification|free_rust_string)"
    echo ""
    echo "Copying to flutter_bittorrent/linux/..."
    mkdir -p ../../flutter_bittorrent/linux
    cp target/release/libdart_ffi.so ../../flutter_bittorrent/linux/
    echo "✓ Library copied successfully"
else
    echo "ERROR: Library not found at target/release/libdart_ffi.so"
    exit 1
fi

echo ""
echo "✅ Build complete! You can now run the Flutter app."
