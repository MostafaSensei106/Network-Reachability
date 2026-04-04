import '../entities/net_info.dart';
import '../entities/report.dart';
import '../entities/target.dart';

/// Contract for performing low-level network reachability probes.
///
/// This interface abstracts the platform-specific logic required to inspect
/// network interfaces, check for captive portals, and validate DNS integrity.
///
/// Implementations of this repository are provided for Native (Rust FFI)
/// and Web (Rust WASM) environments.
abstract interface class NetworkProbesRepository {
  /// Probes the network to see if it's behind a "Captive Portal" (login page).
  ///
  /// This typically involves making an unencrypted HTTP request to a known
  /// endpoint (like Google's connectivity check) and checking for redirects.
  ///
  /// * [timeoutMs]: Maximum duration for the probe to complete.
  Future<CaptivePortalStatus> checkForCaptivePortal({
    required final BigInt timeoutMs,
  });

  /// Validates if DNS queries are being intercepted or tampered with.
  ///
  /// * [domain]: The hostname to resolve and compare against trusted results.
  ///
  /// Returns true if a mismatch is detected, suggesting hijacking.
  Future<bool> detectDnsHijacking({required final String domain});

  /// Scans the system's network interfaces.
  ///
  /// Identifies the [ConnectionType] (WiFi, Cellular, etc.) and populates
  /// security flags such as VPN or Proxy detection.
  Future<(SecurityFlagsResult, ConnectionType)> detectSecurityAndNetworkType();

  /// Executes a single, low-level reachability probe against a specific target.
  ///
  /// * [target]: The endpoint configuration (host, port, protocol).
  ///
  /// Returns a [TargetReport] with latency and success status.
  Future<TargetReport> checkTarget({required final NetworkTarget target});
}
