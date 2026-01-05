import 'dart:ffi';
import 'dart:io';
import 'dart:convert';

import 'package:ffi/ffi.dart';

// FFI type definitions
typedef InitDartFFINative = Void Function();
typedef InitDartFFIDart = void Function();

typedef DispatchEventNative = Pointer<Utf8> Function(Pointer<Uint8>, IntPtr);
typedef DispatchEventDart = Pointer<Utf8> Function(Pointer<Uint8>, int);

typedef SubscribeNotificationNative = Void Function(Pointer<Utf8>, Int64);
typedef SubscribeNotificationDart = void Function(Pointer<Utf8>, int);

typedef UnsubscribeNotificationNative = Void Function(Int64);
typedef UnsubscribeNotificationDart = void Function(int);

typedef FreeRustStringNative = Void Function(Pointer<Utf8>);
typedef FreeRustStringDart = void Function(Pointer<Utf8>);

/// FFI bridge to Rust backend
class FFIBridge {
  static FFIBridge? _instance;
  late final DynamicLibrary _dylib;
  late final InitDartFFIDart _initDartFFI;
  late final DispatchEventDart _dispatchEvent;
  late final SubscribeNotificationDart _subscribeNotification;
  late final UnsubscribeNotificationDart _unsubscribeNotification;
  late final FreeRustStringDart _freeRustString;

  FFIBridge._() {
    // Load the dynamic library
    _dylib = _loadLibrary();

    // Bind functions
    _initDartFFI = _dylib
        .lookup<NativeFunction<InitDartFFINative>>('init_dart_ffi')
        .asFunction();

    _dispatchEvent = _dylib
        .lookup<NativeFunction<DispatchEventNative>>('dispatch_event')
        .asFunction();

    _subscribeNotification = _dylib
        .lookup<NativeFunction<SubscribeNotificationNative>>(
            'subscribe_notification')
        .asFunction();

    _unsubscribeNotification = _dylib
        .lookup<NativeFunction<UnsubscribeNotificationNative>>(
            'unsubscribe_notification')
        .asFunction();

    _freeRustString = _dylib
        .lookup<NativeFunction<FreeRustStringNative>>('free_rust_string')
        .asFunction();

    // Initialize
    _initDartFFI();
  }

  static FFIBridge get instance {
    _instance ??= FFIBridge._();
    return _instance!;
  }

  DynamicLibrary _loadLibrary() {
    if (Platform.isLinux) {
      return DynamicLibrary.open('linux/libdart_ffi.so');
    } else if (Platform.isAndroid) {
      return DynamicLibrary.open('libdart_ffi.so');
    } else if (Platform.isMacOS) {
      return DynamicLibrary.open('macos/libdart_ffi.dylib');
    } else if (Platform.isWindows) {
      return DynamicLibrary.open('windows/dart_ffi.dll');
    } else {
      throw UnsupportedError('Unsupported platform');
    }
  }

  /// Dispatch an event to Rust
  List<int> dispatchEvent(List<int> eventBytes) {
    // Convert Dart bytes to native pointer
    final ptr = malloc<Uint8>(eventBytes.length);
    final list = ptr.asTypedList(eventBytes.length);
    list.setAll(0, eventBytes);

    // Call Rust function
    final resultPtr = _dispatchEvent(ptr, eventBytes.length);

    // Free the input pointer
    malloc.free(ptr);

    // Get the response string (base64 encoded)
    final resultString = resultPtr.toDartString();

    // Free the Rust-allocated string
    _freeRustString(resultPtr);

    // Decode base64
    return base64.decode(resultString);
  }

  /// Subscribe to notifications
  void subscribeNotification(String notificationType, int dartPort) {
    final typePtr = notificationType.toNativeUtf8();
    _subscribeNotification(typePtr, dartPort);
    malloc.free(typePtr);
  }

  /// Unsubscribe from notifications
  void unsubscribeNotification(int dartPort) {
    _unsubscribeNotification(dartPort);
  }
}
