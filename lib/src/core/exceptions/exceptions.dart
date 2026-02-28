import '../constants/enums.dart';

/// Base class for all network reachability exceptions.
///
/// This class is abstract and cannot be directly instantiated.
abstract base class NetworkReachabilityException implements Exception {
  /// The message associated with this exception.
  final String message;

  /// Creates a new [NetworkReachabilityException] with the given [message].
  NetworkReachabilityException(this.message);

  @override
  String toString() => '$runtimeType: $message';
}

/// Thrown by the `guard` method when the connection quality is below the required minimum.
final class PoorConnectionException extends NetworkReachabilityException {
  /// Creates a new [PoorConnectionException] with the given [message].
  PoorConnectionException(super.message);
}

/// Thrown by the `guard` method when a security requirement is not met (e.g., VPN detected).
final class SecurityException extends NetworkReachabilityException {
  /// The reason this exception was thrown.
  final SecurityAlert reason;

  /// Creates a new [SecurityException] with the given [reason] and [message].
  SecurityException(this.reason, String message) : super(message);
}

/// Thrown by the `guard` method when the circuit breaker is open due to repeated backend failures.
final class CircuitBreakerOpenException extends NetworkReachabilityException {
  /// The duration to wait before attempting another request.
  final Duration? retryAfter;

  /// Creates a new [CircuitBreakerOpenException] with the given [message] and [retryAfter].
  CircuitBreakerOpenException(super.message, {this.retryAfter});
}

/// Thrown when a network check operation times out.
final class NetworkTimeoutException extends NetworkReachabilityException {
  /// Creates a new [NetworkTimeoutException] with the given [message].
  NetworkTimeoutException(super.message);
}
