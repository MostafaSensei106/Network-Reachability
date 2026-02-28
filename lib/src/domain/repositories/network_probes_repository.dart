import '../entities/net_info.dart';
import '../entities/report.dart';
import '../entities/target.dart';

/// Interface for network reachability probes.
///
/// This repository defines the contract for platform-specific network probes
/// such as captive portal detection, DNS hijacking detection, and interface inspection.
abstract interface class NetworkProbesRepository {
  /// Checks for the presence of a captive portal.
  ///
  /// [timeoutMs] The maximum time allowed for the probe.
  ///
  /// Returns a [Future] resolving to [CaptivePortalStatus].
  Future<CaptivePortalStatus> checkForCaptivePortal({required BigInt timeoutMs});

  /// Detects potential DNS hijacking for a given domain.
  ///
  /// [domain] The hostname to resolve and validate.
  ///
  /// Returns a [Future] resolving to `true` if hijacking is detected.
  Future<bool> detectDnsHijacking({required String domain});

  /// Inspects system network interfaces to determine connection type and security flags.
  ///
  /// Returns a [Future] resolving to a tuple of [SecurityFlagsResult] and [ConnectionType].
  Future<(SecurityFlagsResult, ConnectionType)> detectSecurityAndNetworkType();

  /// Performs a low-level reachability check against a single network target.
  ///
  /// [target] The specific endpoint configuration to probe.
  ///
  /// Returns a [Future] resolving to a [TargetReport].
  Future<TargetReport> checkTarget({required NetworkTarget target});
}
