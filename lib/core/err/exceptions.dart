import 'package:network_reachability/core/constants/enums.dart';

/// Base class for all network reachability exceptions.
abstract class NetworkReachabilityException implements Exception {
  final String message;
  NetworkReachabilityException(this.message);

  @override
  String toString() => '$runtimeType: $message';
}

/// Thrown by the `guard` method when the connection quality is below the required minimum.
class PoorConnectionException extends NetworkReachabilityException {
  PoorConnectionException(super.message);
}

/// Thrown by the `guard` method when a security requirement is not met (e.g., VPN detected).
class SecurityException extends NetworkReachabilityException {
  final SecurityAlert reason;
  SecurityException(this.reason, String message) : super(message);
}

/// Thrown by the `guard` method when the circuit breaker is open due to repeated backend failures.
class CircuitBreakerOpenException extends NetworkReachabilityException {
  CircuitBreakerOpenException(super.message);
}
