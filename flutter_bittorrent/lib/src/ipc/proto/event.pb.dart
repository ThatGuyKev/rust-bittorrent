///
//  Generated code. Do not modify.
//  source: event.proto
///
// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:core' as $core;
import 'package:fixnum/fixnum.dart' as $fixnum;
import 'package:protobuf/protobuf.dart' as $pb;

class Event extends $pb.GeneratedMessage {
  factory Event({
    $core.String? eventType,
    $core.List<$core.int>? payload,
  }) {
    final $result = create();
    if (eventType != null) {
      $result.eventType = eventType;
    }
    if (payload != null) {
      $result.payload = payload;
    }
    return $result;
  }
  Event._() : super();
  factory Event.fromBuffer($core.List<$core.int> i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(i, r);
  factory Event.fromJson($core.String i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'Event',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'event'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'eventType')
    ..a<$core.List<$core.int>>(
        2, _omitFieldNames ? '' : 'payload', $pb.PbFieldType.OY)
    ..hasRequiredFields = false;

  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
      'Will be removed in next major version')
  Event clone() => Event()..mergeFromMessage(this);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
      'Will be removed in next major version')
  Event copyWith(void Function(Event) updates) =>
      super.copyWith((message) => updates(message as Event)) as Event;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static Event create() => Event._();
  Event createEmptyInstance() => create();
  static $pb.PbList<Event> createRepeated() => $pb.PbList<Event>();
  @$core.pragma('dart2js:noInline')
  static Event getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Event>(create);
  static Event? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get eventType => $_getSZ(0);
  @$pb.TagNumber(1)
  set eventType($core.String v) {
    $_setString(0, v);
  }

  @$pb.TagNumber(1)
  $core.bool hasEventType() => $_has(0);
  @$pb.TagNumber(1)
  void clearEventType() => clearField(1);

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

class EventResponse extends $pb.GeneratedMessage {
  factory EventResponse({
    $core.bool? success,
    $core.List<$core.int>? data,
    $core.String? error,
  }) {
    final $result = create();
    if (success != null) {
      $result.success = success;
    }
    if (data != null) {
      $result.data = data;
    }
    if (error != null) {
      $result.error = error;
    }
    return $result;
  }
  EventResponse._() : super();
  factory EventResponse.fromBuffer($core.List<$core.int> i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(i, r);
  factory EventResponse.fromJson($core.String i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'EventResponse',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'event'),
      createEmptyInstance: create)
    ..aOB(1, _omitFieldNames ? '' : 'success')
    ..a<$core.List<$core.int>>(
        2, _omitFieldNames ? '' : 'data', $pb.PbFieldType.OY)
    ..aOS(3, _omitFieldNames ? '' : 'error')
    ..hasRequiredFields = false;

  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
      'Will be removed in next major version')
  EventResponse clone() => EventResponse()..mergeFromMessage(this);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
      'Will be removed in next major version')
  EventResponse copyWith(void Function(EventResponse) updates) =>
      super.copyWith((message) => updates(message as EventResponse))
          as EventResponse;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static EventResponse create() => EventResponse._();
  EventResponse createEmptyInstance() => create();
  static $pb.PbList<EventResponse> createRepeated() =>
      $pb.PbList<EventResponse>();
  @$core.pragma('dart2js:noInline')
  static EventResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<EventResponse>(create);
  static EventResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.bool get success => $_getBF(0);
  @$pb.TagNumber(1)
  set success($core.bool v) {
    $_setBool(0, v);
  }

  @$pb.TagNumber(1)
  $core.bool hasSuccess() => $_has(0);
  @$pb.TagNumber(1)
  void clearSuccess() => clearField(1);

  @$pb.TagNumber(2)
  $core.List<$core.int> get data => $_getN(1);
  @$pb.TagNumber(2)
  set data($core.List<$core.int> v) {
    $_setBytes(1, v);
  }

  @$pb.TagNumber(2)
  $core.bool hasData() => $_has(1);
  @$pb.TagNumber(2)
  void clearData() => clearField(2);

  @$pb.TagNumber(3)
  $core.String get error => $_getSZ(2);
  @$pb.TagNumber(3)
  set error($core.String v) {
    $_setString(2, v);
  }

  @$pb.TagNumber(3)
  $core.bool hasError() => $_has(2);
  @$pb.TagNumber(3)
  void clearError() => clearField(3);
}

class StartTorrentRequest extends $pb.GeneratedMessage {
  factory StartTorrentRequest({
    $core.String? torrentPath,
    $core.String? downloadDir,
  }) {
    final $result = create();
    if (torrentPath != null) {
      $result.torrentPath = torrentPath;
    }
    if (downloadDir != null) {
      $result.downloadDir = downloadDir;
    }
    return $result;
  }
  StartTorrentRequest._() : super();
  factory StartTorrentRequest.fromBuffer($core.List<$core.int> i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(i, r);
  factory StartTorrentRequest.fromJson($core.String i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'StartTorrentRequest',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'event'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'torrentPath')
    ..aOS(2, _omitFieldNames ? '' : 'downloadDir')
    ..hasRequiredFields = false;

  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
      'Will be removed in next major version')
  StartTorrentRequest clone() => StartTorrentRequest()..mergeFromMessage(this);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
      'Will be removed in next major version')
  StartTorrentRequest copyWith(void Function(StartTorrentRequest) updates) =>
      super.copyWith((message) => updates(message as StartTorrentRequest))
          as StartTorrentRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static StartTorrentRequest create() => StartTorrentRequest._();
  StartTorrentRequest createEmptyInstance() => create();
  static $pb.PbList<StartTorrentRequest> createRepeated() =>
      $pb.PbList<StartTorrentRequest>();
  @$core.pragma('dart2js:noInline')
  static StartTorrentRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<StartTorrentRequest>(create);
  static StartTorrentRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get torrentPath => $_getSZ(0);
  @$pb.TagNumber(1)
  set torrentPath($core.String v) {
    $_setString(0, v);
  }

  @$pb.TagNumber(1)
  $core.bool hasTorrentPath() => $_has(0);
  @$pb.TagNumber(1)
  void clearTorrentPath() => clearField(1);

  @$pb.TagNumber(2)
  $core.String get downloadDir => $_getSZ(1);
  @$pb.TagNumber(2)
  set downloadDir($core.String v) {
    $_setString(1, v);
  }

  @$pb.TagNumber(2)
  $core.bool hasDownloadDir() => $_has(1);
  @$pb.TagNumber(2)
  void clearDownloadDir() => clearField(2);
}

class StartTorrentResponse extends $pb.GeneratedMessage {
  factory StartTorrentResponse({
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
  StartTorrentResponse._() : super();
  factory StartTorrentResponse.fromBuffer($core.List<$core.int> i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(i, r);
  factory StartTorrentResponse.fromJson($core.String i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'StartTorrentResponse',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'event'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'torrentId')
    ..aOS(2, _omitFieldNames ? '' : 'name')
    ..aInt64(3, _omitFieldNames ? '' : 'totalSize')
    ..hasRequiredFields = false;

  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
      'Will be removed in next major version')
  StartTorrentResponse clone() =>
      StartTorrentResponse()..mergeFromMessage(this);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
      'Will be removed in next major version')
  StartTorrentResponse copyWith(void Function(StartTorrentResponse) updates) =>
      super.copyWith((message) => updates(message as StartTorrentResponse))
          as StartTorrentResponse;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static StartTorrentResponse create() => StartTorrentResponse._();
  StartTorrentResponse createEmptyInstance() => create();
  static $pb.PbList<StartTorrentResponse> createRepeated() =>
      $pb.PbList<StartTorrentResponse>();
  @$core.pragma('dart2js:noInline')
  static StartTorrentResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<StartTorrentResponse>(create);
  static StartTorrentResponse? _defaultInstance;

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

class GetTorrentStatusRequest extends $pb.GeneratedMessage {
  factory GetTorrentStatusRequest({
    $core.String? torrentId,
  }) {
    final $result = create();
    if (torrentId != null) {
      $result.torrentId = torrentId;
    }
    return $result;
  }
  GetTorrentStatusRequest._() : super();
  factory GetTorrentStatusRequest.fromBuffer($core.List<$core.int> i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(i, r);
  factory GetTorrentStatusRequest.fromJson($core.String i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GetTorrentStatusRequest',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'event'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'torrentId')
    ..hasRequiredFields = false;

  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
      'Will be removed in next major version')
  GetTorrentStatusRequest clone() =>
      GetTorrentStatusRequest()..mergeFromMessage(this);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
      'Will be removed in next major version')
  GetTorrentStatusRequest copyWith(
          void Function(GetTorrentStatusRequest) updates) =>
      super.copyWith((message) => updates(message as GetTorrentStatusRequest))
          as GetTorrentStatusRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static GetTorrentStatusRequest create() => GetTorrentStatusRequest._();
  GetTorrentStatusRequest createEmptyInstance() => create();
  static $pb.PbList<GetTorrentStatusRequest> createRepeated() =>
      $pb.PbList<GetTorrentStatusRequest>();
  @$core.pragma('dart2js:noInline')
  static GetTorrentStatusRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<GetTorrentStatusRequest>(create);
  static GetTorrentStatusRequest? _defaultInstance;

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
}

class GetTorrentStatusResponse extends $pb.GeneratedMessage {
  factory GetTorrentStatusResponse({
    $core.String? torrentId,
    $core.String? status,
    $fixnum.Int64? downloaded,
    $fixnum.Int64? totalSize,
    $core.double? progress,
  }) {
    final $result = create();
    if (torrentId != null) {
      $result.torrentId = torrentId;
    }
    if (status != null) {
      $result.status = status;
    }
    if (downloaded != null) {
      $result.downloaded = downloaded;
    }
    if (totalSize != null) {
      $result.totalSize = totalSize;
    }
    if (progress != null) {
      $result.progress = progress;
    }
    return $result;
  }
  GetTorrentStatusResponse._() : super();
  factory GetTorrentStatusResponse.fromBuffer($core.List<$core.int> i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(i, r);
  factory GetTorrentStatusResponse.fromJson($core.String i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GetTorrentStatusResponse',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'event'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'torrentId')
    ..aOS(2, _omitFieldNames ? '' : 'status')
    ..aInt64(3, _omitFieldNames ? '' : 'downloaded')
    ..aInt64(4, _omitFieldNames ? '' : 'totalSize')
    ..a<$core.double>(5, _omitFieldNames ? '' : 'progress', $pb.PbFieldType.OD)
    ..hasRequiredFields = false;

  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
      'Will be removed in next major version')
  GetTorrentStatusResponse clone() =>
      GetTorrentStatusResponse()..mergeFromMessage(this);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
      'Will be removed in next major version')
  GetTorrentStatusResponse copyWith(
          void Function(GetTorrentStatusResponse) updates) =>
      super.copyWith((message) => updates(message as GetTorrentStatusResponse))
          as GetTorrentStatusResponse;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static GetTorrentStatusResponse create() => GetTorrentStatusResponse._();
  GetTorrentStatusResponse createEmptyInstance() => create();
  static $pb.PbList<GetTorrentStatusResponse> createRepeated() =>
      $pb.PbList<GetTorrentStatusResponse>();
  @$core.pragma('dart2js:noInline')
  static GetTorrentStatusResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<GetTorrentStatusResponse>(create);
  static GetTorrentStatusResponse? _defaultInstance;

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
  $core.String get status => $_getSZ(1);
  @$pb.TagNumber(2)
  set status($core.String v) {
    $_setString(1, v);
  }

  @$pb.TagNumber(2)
  $core.bool hasStatus() => $_has(1);
  @$pb.TagNumber(2)
  void clearStatus() => clearField(2);

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
  $core.double get progress => $_getN(4);
  @$pb.TagNumber(5)
  set progress($core.double v) {
    $_setDouble(4, v);
  }

  @$pb.TagNumber(5)
  $core.bool hasProgress() => $_has(4);
  @$pb.TagNumber(5)
  void clearProgress() => clearField(5);
}

const _omitFieldNames = $core.bool.fromEnvironment('protobuf.omit_field_names');
const _omitMessageNames =
    $core.bool.fromEnvironment('protobuf.omit_message_names');
