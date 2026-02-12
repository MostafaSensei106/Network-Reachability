/// A comprehensive networking library for Dart and Flutter, built on a powerful Rust core.
///
/// This library provides advanced network monitoring, reachability checks, and security features
/// to ensure your application's network communication is reliable and secure.
///
/// It provides a high-level Dart API over a powerful Rust engine, allowing you to easily integrate
/// network monitoring into your Dart and Flutter applications.
///
/// The library is designed to be highly customizable, with support for custom security policies,
/// quality thresholds, and performance tuning.
///
/// The library also provides a set of custom exceptions that can be used to handle errors in a more
/// fine-grained way.
///
/// The library is organized into several sub-libraries, each of which provides a specific set of functionality.
///
/// * The `core/logic` sub-library contains the core logic of the library, including the main entry points
///   for performing network checks.
/// * The `core/err` sub-library contains custom exceptions that can be used to handle errors in a more
///   fine-grained way.
/// * The `core/rust/api` sub-library contains the data models that are used to interact with the library.
/// * The `core/rust/frb_generated` sub-library contains the auto-generated code for interacting with the Rust core.

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
    show ConnectionType, CaptivePortalStatus, SecurityFlags, TraceHop;
export 'core/rust/api/models/report.dart'
    show NetworkReport, NetworkStatus, TargetReport;
export 'core/rust/api/models/target.dart' show NetworkTarget, TargetProtocol;

export 'core/rust/frb_generated.dart' show RustLib;
