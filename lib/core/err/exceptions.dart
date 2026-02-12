import 'package:network_reachability/core/constants/enums.dart';

/// Base class for all network reachability exceptions.
///
/// This class is abstract and cannot be directly instantiated.
///
/// It provides a common interface for all network reachability exceptions.
///
/// See also [NetworkReachabilityException.toString] for more information.
abstract class NetworkReachabilityException implements Exception {
  /// The message associated with this exception.
  ///
  /// This string is human-readable and should provide enough information for a
  /// user to understand the error and potentially take corrective action.
  final String message;

  /// Creates a new [NetworkReachabilityException] with the given [message].
  NetworkReachabilityException(this.message);

  /// Converts this object to a human-readable string representation.
  ///
  /// This string will contain the runtime type and the message associated with
  /// this exception.
  @override
  String toString() => '$runtimeType: $message';
}

/// Thrown by the `guard` method when the connection quality is below the required minimum.
class PoorConnectionException extends NetworkReachabilityException {
  /// Creates a new [PoorConnectionException] with the given [message].
  PoorConnectionException(super.message);
}

/// Thrown by the `guard` method when a security requirement is not met (e.g., VPN detected).
class SecurityException extends NetworkReachabilityException {
  /// The reason this exception was thrown.
  ///
  /// This is a human-readable string that provides additional information about the
  /// security requirement that was not met.
  final SecurityAlert reason;

  /// Creates a new [SecurityException] with the given [reason] and [message].
  SecurityException(this.reason, String message) : super(message);
}

/// Thrown by the `guard` method when the circuit breaker is open due to repeated backend failures.
class CircuitBreakerOpenException extends NetworkReachabilityException {
  /// Creates a new [CircuitBreakerOpenException] with the given [message].
  CircuitBreakerOpenException(super.message);
}
