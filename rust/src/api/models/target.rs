#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TargetProtocol {
    Tcp, // Connect (Reliable)
    Udp, // Send/Receive (Fast but tricky)
}

#[derive(Debug, Clone)]
pub struct NetworkTarget {
    pub label: String,
    pub host: String,
    pub port: u16,
    pub protocol: TargetProtocol,
    pub timeout_ms: u64,
    pub priority: u8,       // 1 = High, 2 = Low
    pub is_essential: bool, // true = Essential, false = Optional
}
