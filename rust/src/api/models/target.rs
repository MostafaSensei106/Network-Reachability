//! Data structures for defining a network check target.

/// The network protocol to use for a check.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TargetProtocol {
    /// Use the Transmission Control Protocol (TCP). This is a reliable, connection-oriented protocol.
    Tcp,
    /// Use the User Datagram Protocol (UDP). This is a fast, connectionless protocol.
    Udp,
}

/// Defines a single network endpoint to be checked.
#[derive(Debug, Clone)]
pub struct NetworkTarget {
    /// A unique, human-readable label for this target (e.g., "Google DNS").
    pub label: String,
    /// The hostname or IP address of the target.
    pub host: String,
    /// The port number to connect to.
    pub port: u16,
    /// The protocol (TCP or UDP) to use for the check.
    pub protocol: TargetProtocol,
    /// The timeout in milliseconds for this specific target check.
    pub timeout_ms: u64,
    /// The priority of the target. While not used in the current engine logic,
    /// it can be used by the caller for sorting or selection. (e.g., 1=High).
    pub priority: u8,
    /// If true, a failure to connect to this target will be considered a critical
    /// failure, affecting the circuit breaker and potentially the overall check status.
    pub is_essential: bool,
}
