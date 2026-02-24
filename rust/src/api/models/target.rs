//! Data structures for defining a network check target.

/// Supported protocols for network reachability checks.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TargetProtocol {
    /// Transmission Control Protocol.
    ///
    /// Performs a full 3-way handshake to ensure the port is open and accepting connections.
    Tcp,
    /// Internet Control Message Protocol.
    ///
    /// Sends an Echo Request (ping). Requires appropriate OS permissions.
    Icmp,
    /// Hypertext Transfer Protocol.
    ///
    /// Performs a GET or HEAD request and expects a successful status code (2xx/3xx).
    Http,
    /// Secure Hypertext Transfer Protocol.
    ///
    /// Performs an encrypted request over TLS. Validates the certificate chain by default.
    Https,
}

/// Configuration for a specific network endpoint to be monitored.
#[derive(Debug, Clone)]
pub struct NetworkTarget {
    /// A human-readable identifier for the target (e.g., "Primary DNS", "API Gateway").
    pub label: String,
    /// The remote address to check. Can be a domain name (google.com) or an IP address (8.8.8.8).
    pub host: String,
    /// The destination port for the check (e.g., 80 for HTTP, 443 for HTTPS, 53 for DNS).
    pub port: u16,
    /// The network protocol to use for the probe.
    pub protocol: TargetProtocol,
    /// The maximum time in milliseconds to wait for a response before timing out.
    pub timeout_ms: u64,
    /// The relative priority of this target (lower numbers = higher priority).
    ///
    /// While the current engine treats all targets equally, this field is useful
    /// for UI sorting or selective checking in future versions.
    pub priority: u8,
    /// If true, a failure of this specific target is considered a critical event.
    ///
    /// Essential targets trigger the circuit breaker and can disqualify
    /// the network from being considered "connected" depending on the strategy.
    pub is_essential: bool,
}
