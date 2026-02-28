import 'package:network_reachability/rust/api/models/net_info.dart';
import 'package:network_reachability/rust/api/models/report.dart';
import 'package:network_reachability/rust/api/models/target.dart';
import 'package:network_reachability/rust/api/probes/captive_portal.dart'
    as rust_captive_portal;
import 'package:network_reachability/rust/api/probes/dns.dart' as rust_dns;
import 'package:network_reachability/rust/api/probes/interface.dart'
    as rust_interface;
import 'package:network_reachability/rust/api/probes/target.dart'
    as rust_target;

/// Native-specific data source for network probes.
///
/// This class handles direct interaction with the Rust backend via FFI
/// for native platforms (Android, iOS, Desktop).
final class NetworkProbesSource {
  /// Checks for the presence of a captive portal using the native implementation.
  ///
  /// [timeoutMs] The timeout for the probe.
  ///
  /// Returns a [Future] resolving to [CaptivePortalStatus].
  static Future<CaptivePortalStatus> checkForCaptivePortal(
          {required BigInt timeoutMs}) =>
      rust_captive_portal.checkForCaptivePortal(timeoutMs: timeoutMs);

  /// Detects potential DNS hijacking using the native implementation.
  ///
  /// [domain] The domain to test.
  ///
  /// Returns a [Future] resolving to `true` if hijacking is detected.
  static Future<bool> detectDnsHijacking({required String domain}) =>
      rust_dns.detectDnsHijacking(domain: domain);

  /// Inspects system network interfaces to determine connection type and security flags using native APIs.
  ///
  /// Returns a [Future] resolving to a tuple of [SecurityFlagsResult] and [ConnectionType].
  static Future<(SecurityFlagsResult, ConnectionType)>
      detectSecurityAndNetworkType() =>
          rust_interface.detectSecurityAndNetworkType();

  /// Performs a low-level reachability check against a single target.
  ///
  /// [target] The target to probe.
  ///
  /// Returns a [Future] resolving to [TargetReport].
  static Future<TargetReport> checkTarget({required NetworkTarget target}) =>
      rust_target.checkTarget(target: target);
}
