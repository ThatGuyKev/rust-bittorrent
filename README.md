# BitTorrent Flutter-Rust

A cross-platform BitTorrent client built with Flutter for the UI and Rust for the backend BitTorrent protocol implementation. This project demonstrates efficient interoperability between Flutter and Rust using FFI (Foreign Function Interface).

### Build System
- **Cargo** - Rust package manager and build tool
- **CMake** - Native build configuration (Linux/Windows)
- **Protocol Buffers** - Message schema compilation

## 📋 Features

- BitTorrent file download support
- Magnet link support
- TODO: Real-time download progress tracking
- TODO: Download speed monitoring
- TODO: Cross-platform support (Linux, Windows, macOS, Android, iOS)
- TODO: File picker integration
- TODO: Modern Material Design UI

## 🛠️ Prerequisites

Before building the project, ensure you have the following installed:

- **Flutter** (3.6.2 or higher)
- **Rust** (rustc 1.70.0 or higher)
- **Protocol Buffers compiler** (`protoc`)

### Platform-Specific Requirements

#### Linux
```bash
sudo apt-get install cmake ninja-build clang pkg-config libgtk-3-dev
```

#### macOS
```bash
brew install cmake protobuf
```

#### Windows
- Not supported yet

## 🚀 How to Run

### 1. Clone the Repository
```bash
git clone <repository-url>
cd bittorrent-f-r
```

### 2. Build the Project

The project includes a build script that handles both Rust and Flutter compilation:

```bash
chmod +x build.sh
./build.sh
```

This script will:
1. Build the Rust FFI library in release mode
2. Install Flutter dependencies
3. Run Flutter code analysis

### 3. Run the Application

After building, navigate to the Flutter directory and run:

```bash
cd flutter_bittorrent
flutter run
```

For specific platforms:

```bash
# Linux
flutter run -d linux

# Windows
flutter run -d windows

# macOS
flutter run -d macos

# Android (with device connected)
flutter run -d android

# iOS (macOS only, with device/simulator)
flutter run -d ios
```

## 🤝 Contributing

Contributions are welcome! Please feel free to submit pull requests or open issues.

## 📄 License

This project is licensed under the MIT License 