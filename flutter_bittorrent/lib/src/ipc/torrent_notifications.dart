import 'dart:async';
import 'package:flutter_bittorrent/src/ipc/notification_listener.dart';
import 'package:flutter_bittorrent/src/ipc/proto/notification.pb.dart'
    as notification_pb;

/// Torrent Notifications - Subscribe to backend events
class TorrentNotifications {
  static final NotificationListener _listener = NotificationListener.instance;

  /// Listen to torrent progress updates
  static StreamSubscription<notification_pb.TorrentProgressNotification>
      onProgress(
    void Function(notification_pb.TorrentProgressNotification) handler,
  ) {
    return _listener.on(
      'TorrentProgress',
      notification_pb.TorrentProgressNotification.fromBuffer,
      handler,
    );
  }

  /// Listen to torrent completion
  static StreamSubscription<notification_pb.TorrentCompletedNotification>
      onCompleted(
    void Function(notification_pb.TorrentCompletedNotification) handler,
  ) {
    return _listener.on(
      'TorrentCompleted',
      notification_pb.TorrentCompletedNotification.fromBuffer,
      handler,
    );
  }

  /// Listen to torrent errors
  static StreamSubscription<notification_pb.TorrentErrorNotification> onError(
    void Function(notification_pb.TorrentErrorNotification) handler,
  ) {
    return _listener.on(
      'TorrentError',
      notification_pb.TorrentErrorNotification.fromBuffer,
      handler,
    );
  }
}
