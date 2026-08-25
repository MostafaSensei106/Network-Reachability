/// # Network Reachability
///
/// A high-performance, comprehensive networking library for Dart and Flutter,
/// powered by a multi-threaded Rust engine.
///
/// This library goes beyond simple "is it connected?" checks by providing:
/// * **Deep Quality Analysis:** Detailed latency, jitter, and packet loss metrics.
/// * **Security Probes:** Detection of VPNs, DNS hijacking, and proxies.
/// * **Resilience Patterns:** Built-in circuit breakers and adaptive polling.
/// * **Workload Presets:** Pre-configured profiles (Gaming, Streaming, VoIP, IoT, Enterprise).
/// * **Multi-platform:** Consistent behavior across Android, iOS, Web, and Desktop.
///
/// ## Architecture
/// The library follows Clean Architecture principles:
/// * **Application Layer:** [NetworkReachability] service (main API) and [Connectivity] facade.
/// * **Domain Layer:** Immutable entities like [NetworkReport], [NetworkStatus], and [ConfigPreset].
/// * **Core:** Enums, custom [NetworkReachabilityException]s, and helper extensions.
///
/// For getting started, see [NetworkReachability.init].
library;

// --- Application Layer ---
export 'src/application/connectivity.dart';
export 'src/application/network_reachability_service.dart';

// --- Core ---
export 'src/core/constants/enums.dart';
export 'src/core/exceptions/exceptions.dart';
export 'src/core/extensions/model_extensions.dart';

// --- Data Layer (for custom probe mocking/testing) ---
export 'src/data/repositories/network_probes_repository_impl.dart';

// --- Domain Layer ---
export 'src/domain/entities/entities.dart';
export 'src/domain/repositories/network_probes_repository.dart';
