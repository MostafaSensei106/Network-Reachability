import '../../../../rust/api/models/net_info.dart';
import '../../../../rust/api/models/report.dart';
import '../../../../rust/api/models/target.dart';
import '../../../../rust/api/probes/captive_portal.dart' as rust_captive_portal;
import '../../../../rust/api/probes/dns.dart' as rust_dns;
import '../../../../rust/api/probes/interface.dart' as rust_interface;
import '../../../../rust/api/probes/target.dart' as rust_target;

/// Web-specific data source for network probes.
///
/// This class handles interaction with browser-native APIs (Fetch, Navigator)
/// via Rust WASM glue code.
final class NetworkProbesSource {
  /// Checks for the presence of a captive portal using the web Fetch-based probe.
  ///
  /// [timeoutMs] The timeout for the probe.
  ///
  /// Returns a [Future] resolving to [CaptivePortalStatus].
  static Future<CaptivePortalStatus> checkForCaptivePortal(
          {required BigInt timeoutMs}) =>
      rust_captive_portal.checkForCaptivePortalWeb(timeoutMs: timeoutMs);

  /// Detects potential DNS hijacking in a web environment (best effort).
  ///
  /// [domain] The domain to test.
  ///
  /// Returns a [Future] resolving to `false` (currently unsupported on web).
  static Future<bool> detectDnsHijacking({required String domain}) =>
      rust_dns.detectDnsHijackingWeb(domain: domain);

  /// Inspects browser network interfaces using the Navigator.connection API.
  ///
  /// Returns a [Future] resolving to a tuple of [SecurityFlagsResult] and [ConnectionType].
  static Future<(SecurityFlagsResult, ConnectionType)>
      detectSecurityAndNetworkType() =>
          rust_interface.detectSecurityAndNetworkTypeWeb();

  /// Performs a reachability check using the browser Fetch API.
  ///
  /// [target] The target to probe.
  ///
  /// Returns a [Future] resolving to [TargetReport].
  static Future<TargetReport> checkTarget({required NetworkTarget target}) =>
      rust_target.checkTarget(target: target);
}
