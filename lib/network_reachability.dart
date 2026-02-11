/// A comprehensive networking library for Dart and Flutter, built on a powerful Rust core.
///
/// This library provides advanced network monitoring, reachability checks, and security features
/// to ensure your application's network communication is reliable and secure.
library;

// --- Core Logic ---
export 'core/logic/network_reachability_logic.dart';

// --- Custom Exceptions ---
export 'core/err/exceptions.dart';

// --- Data Models ---
// Export all generated models that the user will need to interact with.
export 'core/rust/api/models/config.dart'
    show
        NetworkConfiguration,
        ResilienceConfig,
        SecurityConfig,
        QualityThresholds,
        CheckStrategy,
        ConnectionQuality;
export 'core/rust/api/models/net_info.dart'
    show
        ConnectionType,
        CaptivePortalStatus,
        SecurityFlags,
        LocalDevice,
        TraceHop;
export 'core/rust/api/models/report.dart'
    show NetworkReport, NetworkStatus, TargetReport;
export 'core/rust/api/models/target.dart' show NetworkTarget, TargetProtocol;

// --- Rust Bridge Initialization ---
// The user might need this for advanced setup.
export 'core/rust/frb_generated.dart' show RustLib;
