//! Target and protocol definitions for network probes.
//!
//! This module defines the [`NetworkTarget`] structure, which is the 
//! fundamental unit of configuration for the reachability engine. 
//! Every endpoint the engine checks must be defined as a target.

/// Supported network protocols for performing reachability probes.
///
/// Each protocol has different performance characteristics and requires 
/// different system permissions.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TargetProtocol {
    /// Transmission Control Protocol (TCP).
    ///
    /// # Behavior
    /// Performs a full TCP 3-way handshake (`SYN` -> `SYN-ACK` -> `ACK`). 
    /// If the connection is established, the target is considered "up".
    ///
    /// # Pros/Cons
    /// + Reliable; works through most firewalls on standard ports (80/443).
    /// - More overhead than ICMP (Ping) as it requires a stateful connection.
    Tcp,

    /// Internet Control Message Protocol (ICMP).
    ///
    /// # Behavior
    /// Sends an "Echo Request" (Ping) and waits for an "Echo Reply".
    ///
    /// # Pros/Cons
    /// + Extremely low overhead; the "purest" measure of network latency.
    /// - Often blocked by public firewalls or corporate proxies.
    /// - May require `root` or special capabilities on some Linux systems.
    Icmp,

    /// Hypertext Transfer Protocol (HTTP/1.1 or HTTP/2).
    ///
    /// # Behavior
    /// Performs a full HTTP/1.1 `GET` or `HEAD` request to the target URL. 
    /// The target is considered "up" if it returns a 2xx (Success) or 
    /// 3xx (Redirection) status code.
    ///
    /// # Use Case
    /// Best for validating that a specific web service or API is actually 
    /// functional, not just that its port is open.
    Http,

    /// Secure Hypertext Transfer Protocol (HTTPS).
    ///
    /// # Behavior
    /// Performs an encrypted request over TLS/SSL. The engine validates 
    /// the certificate chain and expiration by default.
    ///
    /// # Pros/Cons
    /// + High confidence; validates that the end-to-end encrypted path is open.
    /// - Highest overhead; requires a full TLS handshake.
    Https,
}

/// Configuration for a specific network endpoint to be monitored.
///
/// A `NetworkTarget` combines an address, a port, and a protocol to 
/// define a unique "probe point".
#[derive(Debug, Clone)]
pub struct NetworkTarget {
    /// A human-readable identifier (e.g., "Google Cloud DNS", "Internal API").
    /// This label is used in [`TargetReport`](super::report::TargetReport) results.
    pub label: String,

    /// The remote address to check.
    ///
    /// Can be a fully qualified domain name (e.g., `google.com`) or 
    /// a raw IP address (e.g., `1.1.1.1`).
    pub host: String,

    /// The destination port for the check.
    ///
    /// Common ports:
    /// - 80: Default for HTTP
    /// - 443: Default for HTTPS
    /// - 53: Default for DNS (TCP)
    pub port: u16,

    /// The network protocol to use for this specific probe.
    pub protocol: TargetProtocol,

    /// The maximum duration (in milliseconds) the engine should wait for a response.
    ///
    /// If no response is received within this window, the target is 
    /// marked with a [`TimeoutError`](super::error::NetworkError::TimeoutError).
    pub timeout_ms: u64,

    /// The relative priority of this target (lower numbers = higher priority).
    ///
    /// Currently used for UI sorting and as a hint for the [`Race`](super::config::CheckStrategy::Race) 
    /// strategy to decide which targets to fire first.
    pub priority: u8,

    /// If true, a failure of this target is treated as a severe network event.
    ///
    /// Essential targets are the primary triggers for the engine's 
    /// "Circuit Breaker" mechanism. Failure of an essential target can 
    /// disqualify the entire network status regardless of other successes.
    pub is_essential: bool,
}
