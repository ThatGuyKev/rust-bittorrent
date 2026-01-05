import 'dart:async';
import 'dart:isolate';
import 'package:protobuf/protobuf.dart';
import 'proto/notification.pb.dart' as notification_pb;
import 'ffi_bridge.dart';

/// Callback type for notification handlers
typedef NotificationHandler<T extends GeneratedMessage> = void Function(
    T notification);

/// Notification listener - receives notifications from Rust backend
class NotificationListener {
  final FFIBridge _bridge;
  final Map<String, List<_Subscription>> _subscriptions = {};
  final Map<String, ReceivePort> _receivePorts = {};

  NotificationListener._() : _bridge = FFIBridge.instance;

  static NotificationListener? _instance;

  static NotificationListener get instance {
    _instance ??= NotificationListener._();
    return _instance!;
  }

  /// Subscribe to a notification type
  StreamSubscription<T> on<T extends GeneratedMessage>(
    String notificationType,
    T Function(List<int>) decoder,
    NotificationHandler<T> handler,
  ) {
    // Create receive port if not exists
    if (!_receivePorts.containsKey(notificationType)) {
      final receivePort = ReceivePort();
      _receivePorts[notificationType] = receivePort;

      // Listen to messages from Rust
      receivePort.listen((dynamic message) {
        if (message is List<int>) {
          _handleNotification(notificationType, message);
        }
      });

      // Subscribe in Rust - using the SendPort hashCode as a unique identifier
      final portId = receivePort.sendPort.hashCode;
      _bridge.subscribeNotification(notificationType, portId);
    }

    // Create subscription
    final controller = StreamController<T>.broadcast();
    final subscription = _Subscription<T>(
      notificationType,
      decoder,
      handler,
      controller,
    );

    _subscriptions.putIfAbsent(notificationType, () => []).add(subscription);

    // Return stream subscription
    return controller.stream.listen(handler);
  }

  /// Handle incoming notification
  void _handleNotification(
      String notificationType, List<int> notificationBytes) {
    try {
      // Decode notification wrapper
      final notification =
          notification_pb.Notification.fromBuffer(notificationBytes);

      // Find and notify subscribers
      final subs = _subscriptions[notificationType];
      if (subs != null) {
        for (final sub in subs) {
          try {
            final decoded = sub.decoder(notification.payload);
            sub.controller.add(decoded);
          } catch (e) {
            print('Error decoding notification: $e');
          }
        }
      }
    } catch (e) {
      print('Error handling notification: $e');
    }
  }

  /// Unsubscribe from a notification type
  void unsubscribe(String notificationType) {
    final receivePort = _receivePorts[notificationType];
    if (receivePort != null) {
      final portId = receivePort.sendPort.hashCode;
      _bridge.unsubscribeNotification(portId);
      receivePort.close();
      _receivePorts.remove(notificationType);
    }

    // Close all controllers
    final subs = _subscriptions[notificationType];
    if (subs != null) {
      for (final sub in subs) {
        sub.controller.close();
      }
      _subscriptions.remove(notificationType);
    }
  }

  /// Dispose all subscriptions
  void dispose() {
    for (final notificationType in _receivePorts.keys.toList()) {
      unsubscribe(notificationType);
    }
  }
}

class _Subscription<T extends GeneratedMessage> {
  final String notificationType;
  final T Function(List<int>) decoder;
  final NotificationHandler<T> handler;
  final StreamController<T> controller;

  _Subscription(
    this.notificationType,
    this.decoder,
    this.handler,
    this.controller,
  );
}
