import '../../../rust/api/models/net_info.dart';
import '../../../rust/api/models/report.dart';
import '../../../rust/api/models/target.dart';
import '../../../rust/api/probes/captive_portal.dart' as rust_captive_portal;
import '../../../rust/api/probes/dns.dart' as rust_dns;
import '../../../rust/api/probes/interface.dart' as rust_interface;
import '../../../rust/api/probes/target.dart' as rust_target;

/// Data source for network probes on **Native platforms** (Android, iOS, Desktop).
///
/// This implementation directly invokes high-performance Rust functions via
/// the `flutter_rust_bridge` FFI mechanism.
final class NetworkProbesSource {
  /// Calls the Rust backend to check for a captive portal using socket-level probes.
  static Future<CaptivePortalStatus> checkForCaptivePortal({
    required final BigInt timeoutMs,
  }) =>
      rust_captive_portal.checkForCaptivePortal(timeoutMs: timeoutMs);

  /// Calls the Rust backend to validate DNS integrity using low-level OS APIs.
  static Future<bool> detectDnsHijacking({required final String domain}) =>
      rust_dns.detectDnsHijacking(domain: domain);

  /// Calls the Rust backend to inspect system network interfaces (e.g., using `getifaddrs`).
  static Future<(SecurityFlagsResult, ConnectionType)>
      detectSecurityAndNetworkType() =>
          rust_interface.detectSecurityAndNetworkType();

  /// Executes a single reachability check (TCP/ICMP/HTTP) via the Rust engine.
  static Future<TargetReport> checkTarget({
    required final NetworkTarget target,
  }) =>
      rust_target.checkTarget(target: target);
}
