import '../constants/enums.dart';

/// Base class for all exceptions thrown by the Network Reachability library.
///
/// Catch this type if you want to handle any connectivity-related error
/// in a unified way.
abstract base class NetworkReachabilityException implements Exception {
  /// Creates a new [NetworkReachabilityException] with the given [message].
  NetworkReachabilityException(this.message);

  /// A human-readable description of the error.
  final String message;

  @override
  String toString() => '$runtimeType: $message';
}

/// Exception thrown when the network quality is too low for a requested operation.
///
/// This is typically thrown by [NetworkReachability.guard] when the detected
/// [ConnectionQuality] is worse than the required `minQuality`.
///
/// **Handling:** Suggest the user move to a better location or switch to a
/// different network (e.g., from Cellular to WiFi).
final class PoorConnectionException extends NetworkReachabilityException {
  /// Creates a new [PoorConnectionException] with the given [message].
  PoorConnectionException(super.message);
}

/// Exception thrown when a security policy is violated.
///
/// This occurs if a VPN is detected (and `blockVpn` is true) or if
/// DNS hijacking is suspected.
///
/// **Handling:** Warn the user that their connection might be insecure or
/// that they need to disable their VPN to continue.
final class SecurityException extends NetworkReachabilityException {
  /// Creates a new [SecurityException] with the given [reason] and [message].
  SecurityException(this.reason, final String message) : super(message);

  /// The specific security reason why this exception was thrown.
  final SecurityAlert reason;
}

/// Exception thrown when the Circuit Breaker is active.
///
/// This indicates that the network has been consistently failing, and the
/// library is intentionally blocking requests to save resources.
///
/// **Handling:** You should respect the [retryAfter] duration and avoid
/// attempting new network requests until that time has passed.
final class CircuitBreakerOpenException extends NetworkReachabilityException {
  /// Creates a new [CircuitBreakerOpenException] with the given [message] and [retryAfter].
  CircuitBreakerOpenException(super.message, {this.retryAfter});

  /// The suggested duration to wait before trying again.
  final Duration? retryAfter;
}

/// Exception thrown when a network probe or check exceeds its allocated time.
final class NetworkTimeoutException extends NetworkReachabilityException {
  /// Creates a new [NetworkTimeoutException] with the given [message].
  NetworkTimeoutException(super.message);
}
