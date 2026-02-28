/// Enum representing the state of the circuit breaker pattern.
///
/// The circuit breaker is used to prevent an application from repeatedly
/// trying to execute an operation that is likely to fail.
enum CircuitBreakerState {
  /// The circuit is closed. Requests flow normally.
  /// Transitions to [open] after a threshold of consecutive failures.
  closed,

  /// The circuit is open. Requests are blocked immediately.
  /// Transitions to [halfOpen] after a cooldown period.
  open,

  /// The circuit is half-open. A single trial request is allowed.
  /// If it succeeds, the circuit moves to [closed]. If it fails, it moves back to [open].
  halfOpen,
}

/// Enum representing specific security alerts or policy violations.
///
/// This enum is used by the [NetworkReachability.guard] function to identify
/// why a network-dependent action was blocked.
enum SecurityAlert {
  /// A VPN connection was detected on the active interface.
  vpnDetected,

  /// The resolved IP addresses from the system DNS do not match trusted resolvers.
  dnsHijackDetected,

  /// A system-level proxy server was detected.
  proxyDetected,

  /// The active network interface is not in the allowed list.
  unallowedInterface,
}
