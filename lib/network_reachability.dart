/// # Network Reachability
///
/// A high-performance, comprehensive networking library for Dart and Flutter,
/// powered by a multi-threaded Rust engine.
///
/// This library goes beyond simple "is it connected?" checks by providing:
/// * **Deep Quality Analysis:** Detailed latency, jitter, and packet loss metrics.
/// * **Security Probes:** Detection of VPNs, DNS hijacking, and proxies.
/// * **Resilience Patterns:** Built-in circuit breakers and adaptive polling.
/// * **Multi-platform:** Consistent behavior across Android, iOS, Web, and Desktop.
///
/// ## Architecture
/// The library follows Clean Architecture principles:
/// * **Application Layer:** [NetworkReachability] service (main API).
/// * **Domain Layer:** Immutable entities like [NetworkReport] and [NetworkStatus].
/// * **Core:** Enums, custom [NetworkReachabilityException]s, and helper extensions.
///
/// For getting started, see [NetworkReachability.init].
library;

// --- Application Layer ---
export 'src/application/network_reachability_service.dart';

// --- Domain Layer ---
export 'src/domain/entities/entities.dart';

// --- Core ---
export 'src/core/constants/enums.dart';
export 'src/core/exceptions/exceptions.dart';
export 'src/core/extensions/model_extensions.dart';

// --- Internal / Advanced ---
// Export the Rust bridge only if you need low-level access to the generated API.
export 'src/rust/frb_generated.dart';
