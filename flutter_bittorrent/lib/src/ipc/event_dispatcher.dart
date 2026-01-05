import 'dart:async';
import 'package:protobuf/protobuf.dart';
import 'proto/event.pb.dart' as event_pb;
import 'ffi_bridge.dart';

/// Redux-saga style effect types
abstract class Effect<T> {
  const Effect();
}

/// Call effect - dispatches an event and waits for response
class CallEffect<TRequest extends GeneratedMessage,
    TResponse extends GeneratedMessage> extends Effect<TResponse> {
  final String eventType;
  final TRequest request;
  final TResponse Function(List<int>) decoder;

  const CallEffect(this.eventType, this.request, this.decoder);
}

/// Put effect - for side effects (like logging, state updates)
class PutEffect extends Effect<void> {
  final dynamic action;

  const PutEffect(this.action);
}

/// Saga - a generator-like function that yields effects
typedef Saga<T> = Stream<Effect> Function();

/// Event dispatcher with saga-style API
class EventDispatcher {
  static EventDispatcher? _instance;
  final FFIBridge _bridge;

  EventDispatcher._() : _bridge = FFIBridge.instance;

  static EventDispatcher get instance {
    _instance ??= EventDispatcher._();
    return _instance!;
  }

  /// Dispatch a single event (direct call)
  Future<TResponse> call<TRequest extends GeneratedMessage,
      TResponse extends GeneratedMessage>(
    String eventType,
    TRequest request,
    TResponse Function(List<int>) decoder,
  ) async {
    // Create event wrapper
    final event = event_pb.Event()
      ..eventType = eventType
      ..payload = request.writeToBuffer();

    // Send to Rust
    final responseBytes = _bridge.dispatchEvent(event.writeToBuffer());

    // Decode response wrapper
    final response = event_pb.EventResponse.fromBuffer(responseBytes);

    if (!response.success) {
      throw Exception('Event failed: ${response.error}');
    }

    // Decode the actual response data
    return decoder(response.data);
  }

  /// Run a saga - processes effects in sequence
  Future<T?> runSaga<T>(Saga saga) async {
    T? result;

    await for (final effect in saga()) {
      if (effect is CallEffect) {
        // Execute call effect
        result = await call(
          effect.eventType,
          effect.request,
          effect.decoder,
        ) as T?;
      } else if (effect is PutEffect) {
        // Execute put effect (side effect)
        // In a real app, this would dispatch to a state management system
        print('Put effect: ${effect.action}');
      }
    }

    return result;
  }

  /// Helper method to create call effects
  CallEffect<TRequest, TResponse> callEffect<TRequest extends GeneratedMessage,
      TResponse extends GeneratedMessage>(
    String eventType,
    TRequest request,
    TResponse Function(List<int>) decoder,
  ) {
    return CallEffect(eventType, request, decoder);
  }

  /// Helper method to create put effects
  PutEffect putEffect(dynamic action) {
    return PutEffect(action);
  }
}

/// Convenience functions for saga-style code
CallEffect<TRequest, TResponse>
    call<TRequest extends GeneratedMessage, TResponse extends GeneratedMessage>(
  String eventType,
  TRequest request,
  TResponse Function(List<int>) decoder,
) {
  return CallEffect(eventType, request, decoder);
}

PutEffect put(dynamic action) {
  return PutEffect(action);
}
