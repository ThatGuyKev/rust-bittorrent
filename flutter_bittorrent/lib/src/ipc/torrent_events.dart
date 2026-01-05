import 'package:flutter_bittorrent/src/ipc/event_dispatcher.dart';
import 'package:flutter_bittorrent/src/ipc/proto/event.pb.dart' as event_pb;

/// Torrent Events - Redux-saga style API
class TorrentEvents {
  static final EventDispatcher _dispatcher = EventDispatcher.instance;

  /// Start a torrent download
  static Future<event_pb.StartTorrentResponse> startTorrent({
    required String torrentPath,
    required String downloadDir,
  }) async {
    final request = event_pb.StartTorrentRequest()
      ..torrentPath = torrentPath
      ..downloadDir = downloadDir;

    return await _dispatcher.call(
      'StartTorrent',
      request,
      event_pb.StartTorrentResponse.fromBuffer,
    );
  }

  /// Get torrent status
  static Future<event_pb.GetTorrentStatusResponse> getTorrentStatus({
    required String torrentId,
  }) async {
    final request = event_pb.GetTorrentStatusRequest()..torrentId = torrentId;

    return await _dispatcher.call(
      'GetTorrentStatus',
      request,
      event_pb.GetTorrentStatusResponse.fromBuffer,
    );
  }

  /// Saga example: Start torrent and monitor progress
  static Stream<Effect> startTorrentSaga({
    required String torrentPath,
    required String downloadDir,
  }) async* {
    // Dispatch start event
    yield call(
      'StartTorrent',
      event_pb.StartTorrentRequest()
        ..torrentPath = torrentPath
        ..downloadDir = downloadDir,
      event_pb.StartTorrentResponse.fromBuffer,
    );

    // Log action (put effect)
    yield put({'type': 'TORRENT_STARTED', 'path': torrentPath});
  }

  /// Saga example: Poll torrent status
  static Stream<Effect> getTorrentStatusSaga({
    required String torrentId,
  }) async* {
    // Dispatch status event
    yield call(
      'GetTorrentStatus',
      event_pb.GetTorrentStatusRequest()..torrentId = torrentId,
      event_pb.GetTorrentStatusResponse.fromBuffer,
    );

    // Log action
    yield put({'type': 'STATUS_FETCHED', 'torrentId': torrentId});
  }
}
