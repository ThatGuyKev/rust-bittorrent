///
//  Generated code. Do not modify.
//  source: notification.proto
///
// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:core' as $core;
import 'package:fixnum/fixnum.dart' as $fixnum;
import 'package:protobuf/protobuf.dart' as $pb;

class Notification extends $pb.GeneratedMessage {
  factory Notification({
    $core.String? notificationType,
    $core.List<$core.int>? payload,
  }) {
    final $result = create();
    if (notificationType != null) {
      $result.notificationType = notificationType;
    }
    if (payload != null) {
      $result.payload = payload;
    }
    return $result;
  }
  Notification._() : super();
  factory Notification.fromBuffer($core.List<$core.int> i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(i, r);
  factory Notification.fromJson($core.String i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'Notification',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'notification'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'notificationType')
    ..a<$core.List<$core.int>>(
        2, _omitFieldNames ? '' : 'payload', $pb.PbFieldType.OY)
    ..hasRequiredFields = false;

  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
      'Will be removed in next major version')
  Notification clone() => Notification()..mergeFromMessage(this);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
      'Will be removed in next major version')
  Notification copyWith(void Function(Notification) updates) =>
      super.copyWith((message) => updates(message as Notification))
          as Notification;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static Notification create() => Notification._();
  Notification createEmptyInstance() => create();
  static $pb.PbList<Notification> createRepeated() =>
      $pb.PbList<Notification>();
  @$core.pragma('dart2js:noInline')
  static Notification getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<Notification>(create);
  static Notification? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get notificationType => $_getSZ(0);
  @$pb.TagNumber(1)
  set notificationType($core.String v) {
    $_setString(0, v);
  }

  @$pb.TagNumber(1)
  $core.bool hasNotificationType() => $_has(0);
  @$pb.TagNumber(1)
  void clearNotificationType() => clearField(1);

  @$pb.TagNumber(2)
  $core.List<$core.int> get payload => $_getN(1);
  @$pb.TagNumber(2)
  set payload($core.List<$core.int> v) {
    $_setBytes(1, v);
  }

  @$pb.TagNumber(2)
  $core.bool hasPayload() => $_has(1);
  @$pb.TagNumber(2)
  void clearPayload() => clearField(2);
}

class TorrentProgressNotification extends $pb.GeneratedMessage {
  factory TorrentProgressNotification({
    $core.String? torrentId,
    $core.double? progress,
    $fixnum.Int64? downloaded,
    $fixnum.Int64? totalSize,
    $fixnum.Int64? downloadSpeed,
  }) {
    final $result = create();
    if (torrentId != null) {
      $result.torrentId = torrentId;
    }
    if (progress != null) {
      $result.progress = progress;
    }
    if (downloaded != null) {
      $result.downloaded = downloaded;
    }
    if (totalSize != null) {
      $result.totalSize = totalSize;
    }
    if (downloadSpeed != null) {
      $result.downloadSpeed = downloadSpeed;
    }
    return $result;
  }
  TorrentProgressNotification._() : super();
  factory TorrentProgressNotification.fromBuffer($core.List<$core.int> i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(i, r);
  factory TorrentProgressNotification.fromJson($core.String i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'TorrentProgressNotification',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'notification'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'torrentId')
    ..a<$core.double>(2, _omitFieldNames ? '' : 'progress', $pb.PbFieldType.OD)
    ..aInt64(3, _omitFieldNames ? '' : 'downloaded')
    ..aInt64(4, _omitFieldNames ? '' : 'totalSize')
    ..aInt64(5, _omitFieldNames ? '' : 'downloadSpeed')
    ..hasRequiredFields = false;

  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
      'Will be removed in next major version')
  TorrentProgressNotification clone() =>
      TorrentProgressNotification()..mergeFromMessage(this);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
      'Will be removed in next major version')
  TorrentProgressNotification copyWith(
          void Function(TorrentProgressNotification) updates) =>
      super.copyWith(
              (message) => updates(message as TorrentProgressNotification))
          as TorrentProgressNotification;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static TorrentProgressNotification create() =>
      TorrentProgressNotification._();
  TorrentProgressNotification createEmptyInstance() => create();
  static $pb.PbList<TorrentProgressNotification> createRepeated() =>
      $pb.PbList<TorrentProgressNotification>();
  @$core.pragma('dart2js:noInline')
  static TorrentProgressNotification getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<TorrentProgressNotification>(create);
  static TorrentProgressNotification? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get torrentId => $_getSZ(0);
  @$pb.TagNumber(1)
  set torrentId($core.String v) {
    $_setString(0, v);
  }

  @$pb.TagNumber(1)
  $core.bool hasTorrentId() => $_has(0);
  @$pb.TagNumber(1)
  void clearTorrentId() => clearField(1);

  @$pb.TagNumber(2)
  $core.double get progress => $_getN(1);
  @$pb.TagNumber(2)
  set progress($core.double v) {
    $_setDouble(1, v);
  }

  @$pb.TagNumber(2)
  $core.bool hasProgress() => $_has(1);
  @$pb.TagNumber(2)
  void clearProgress() => clearField(2);

  @$pb.TagNumber(3)
  $fixnum.Int64 get downloaded => $_getI64(2);
  @$pb.TagNumber(3)
  set downloaded($fixnum.Int64 v) {
    $_setInt64(2, v);
  }

  @$pb.TagNumber(3)
  $core.bool hasDownloaded() => $_has(2);
  @$pb.TagNumber(3)
  void clearDownloaded() => clearField(3);

  @$pb.TagNumber(4)
  $fixnum.Int64 get totalSize => $_getI64(3);
  @$pb.TagNumber(4)
  set totalSize($fixnum.Int64 v) {
    $_setInt64(3, v);
  }

  @$pb.TagNumber(4)
  $core.bool hasTotalSize() => $_has(3);
  @$pb.TagNumber(4)
  void clearTotalSize() => clearField(4);

  @$pb.TagNumber(5)
  $fixnum.Int64 get downloadSpeed => $_getI64(4);
  @$pb.TagNumber(5)
  set downloadSpeed($fixnum.Int64 v) {
    $_setInt64(4, v);
  }

  @$pb.TagNumber(5)
  $core.bool hasDownloadSpeed() => $_has(4);
  @$pb.TagNumber(5)
  void clearDownloadSpeed() => clearField(5);
}

class TorrentCompletedNotification extends $pb.GeneratedMessage {
  factory TorrentCompletedNotification({
    $core.String? torrentId,
    $core.String? name,
    $fixnum.Int64? totalSize,
  }) {
    final $result = create();
    if (torrentId != null) {
      $result.torrentId = torrentId;
    }
    if (name != null) {
      $result.name = name;
    }
    if (totalSize != null) {
      $result.totalSize = totalSize;
    }
    return $result;
  }
  TorrentCompletedNotification._() : super();
  factory TorrentCompletedNotification.fromBuffer($core.List<$core.int> i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(i, r);
  factory TorrentCompletedNotification.fromJson($core.String i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'TorrentCompletedNotification',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'notification'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'torrentId')
    ..aOS(2, _omitFieldNames ? '' : 'name')
    ..aInt64(3, _omitFieldNames ? '' : 'totalSize')
    ..hasRequiredFields = false;

  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
      'Will be removed in next major version')
  TorrentCompletedNotification clone() =>
      TorrentCompletedNotification()..mergeFromMessage(this);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
      'Will be removed in next major version')
  TorrentCompletedNotification copyWith(
          void Function(TorrentCompletedNotification) updates) =>
      super.copyWith(
              (message) => updates(message as TorrentCompletedNotification))
          as TorrentCompletedNotification;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static TorrentCompletedNotification create() =>
      TorrentCompletedNotification._();
  TorrentCompletedNotification createEmptyInstance() => create();
  static $pb.PbList<TorrentCompletedNotification> createRepeated() =>
      $pb.PbList<TorrentCompletedNotification>();
  @$core.pragma('dart2js:noInline')
  static TorrentCompletedNotification getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<TorrentCompletedNotification>(create);
  static TorrentCompletedNotification? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get torrentId => $_getSZ(0);
  @$pb.TagNumber(1)
  set torrentId($core.String v) {
    $_setString(0, v);
  }

  @$pb.TagNumber(1)
  $core.bool hasTorrentId() => $_has(0);
  @$pb.TagNumber(1)
  void clearTorrentId() => clearField(1);

  @$pb.TagNumber(2)
  $core.String get name => $_getSZ(1);
  @$pb.TagNumber(2)
  set name($core.String v) {
    $_setString(1, v);
  }

  @$pb.TagNumber(2)
  $core.bool hasName() => $_has(1);
  @$pb.TagNumber(2)
  void clearName() => clearField(2);

  @$pb.TagNumber(3)
  $fixnum.Int64 get totalSize => $_getI64(2);
  @$pb.TagNumber(3)
  set totalSize($fixnum.Int64 v) {
    $_setInt64(2, v);
  }

  @$pb.TagNumber(3)
  $core.bool hasTotalSize() => $_has(2);
  @$pb.TagNumber(3)
  void clearTotalSize() => clearField(3);
}

class TorrentErrorNotification extends $pb.GeneratedMessage {
  factory TorrentErrorNotification({
    $core.String? torrentId,
    $core.String? errorMessage,
  }) {
    final $result = create();
    if (torrentId != null) {
      $result.torrentId = torrentId;
    }
    if (errorMessage != null) {
      $result.errorMessage = errorMessage;
    }
    return $result;
  }
  TorrentErrorNotification._() : super();
  factory TorrentErrorNotification.fromBuffer($core.List<$core.int> i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(i, r);
  factory TorrentErrorNotification.fromJson($core.String i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'TorrentErrorNotification',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'notification'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'torrentId')
    ..aOS(2, _omitFieldNames ? '' : 'errorMessage')
    ..hasRequiredFields = false;

  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
      'Will be removed in next major version')
  TorrentErrorNotification clone() =>
      TorrentErrorNotification()..mergeFromMessage(this);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
      'Will be removed in next major version')
  TorrentErrorNotification copyWith(
          void Function(TorrentErrorNotification) updates) =>
      super.copyWith((message) => updates(message as TorrentErrorNotification))
          as TorrentErrorNotification;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static TorrentErrorNotification create() => TorrentErrorNotification._();
  TorrentErrorNotification createEmptyInstance() => create();
  static $pb.PbList<TorrentErrorNotification> createRepeated() =>
      $pb.PbList<TorrentErrorNotification>();
  @$core.pragma('dart2js:noInline')
  static TorrentErrorNotification getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<TorrentErrorNotification>(create);
  static TorrentErrorNotification? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get torrentId => $_getSZ(0);
  @$pb.TagNumber(1)
  set torrentId($core.String v) {
    $_setString(0, v);
  }

  @$pb.TagNumber(1)
  $core.bool hasTorrentId() => $_has(0);
  @$pb.TagNumber(1)
  void clearTorrentId() => clearField(1);

  @$pb.TagNumber(2)
  $core.String get errorMessage => $_getSZ(1);
  @$pb.TagNumber(2)
  set errorMessage($core.String v) {
    $_setString(1, v);
  }

  @$pb.TagNumber(2)
  $core.bool hasErrorMessage() => $_has(1);
  @$pb.TagNumber(2)
  void clearErrorMessage() => clearField(2);
}

const _omitFieldNames = $core.bool.fromEnvironment('protobuf.omit_field_names');
const _omitMessageNames =
    $core.bool.fromEnvironment('protobuf.omit_message_names');
