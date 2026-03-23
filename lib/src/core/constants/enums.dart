/// Represents the operational state of the Circuit Breaker pattern.
///
/// The circuit breaker is a resilience pattern that prevents an application
/// from repeatedly attempting an operation that is likely to fail, such as
/// making network requests when the internet is completely down.
enum CircuitBreakerState {
  /// The circuit is healthy and closed.
  ///
  /// Requests are allowed to pass through to the network. If a certain number
  /// of consecutive failures occur (defined by `circuitBreakerThreshold`),
  /// the circuit transitions to [open].
  closed,

  /// The circuit is broken and open.
  ///
  /// Requests are blocked immediately without even attempting a network probe.
  /// This state persists for a cooldown period (`circuitBreakerCooldownMs`),
  /// after which it transitions to [halfOpen].
  open,

  /// The circuit is in a trial state.
  ///
  /// A single request is allowed to pass through. If it succeeds, the circuit
  /// returns to [closed]. If it fails, it returns to [open] and the cooldown
  /// period restarts.
  halfOpen,
}

/// Identifies specific security violations or network policy alerts.
///
/// These alerts are primarily used by the [NetworkReachability.guard] method
/// to explain why a sensitive operation was blocked.
enum SecurityAlert {
  /// A VPN or tunnel interface (e.g., WireGuard, OpenVPN) was detected.
  ///
  /// Often used to enforce region-locking or prevent bypass of security filters.
  vpnDetected,

  /// The system's DNS resolution results do not match trusted resolvers.
  ///
  /// Indicates that the network might be intercepting or tampering with
  /// DNS traffic (Man-in-the-Middle attack).
  dnsHijackDetected,

  /// An HTTP or SOCKS proxy was detected in the system settings.
  ///
  /// This can be an indicator of traffic monitoring or unauthorized interception.
  proxyDetected,

  /// The active network interface is not permitted by the application policy.
  unallowedInterface,
}
