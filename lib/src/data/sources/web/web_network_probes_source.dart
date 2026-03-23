import '../../../../src/rust/api/models/net_info.dart';
import '../../../../src/rust/api/models/report.dart';
import '../../../../src/rust/api/models/target.dart';
import '../../../../src/rust/api/probes/captive_portal.dart'
    as rust_captive_portal;
import '../../../../src/rust/api/probes/dns.dart' as rust_dns;
import '../../../../src/rust/api/probes/interface.dart' as rust_interface;
import '../../../../src/rust/api/probes/target.dart' as rust_target;

/// Data source for network probes on the **Web platform**.
///
/// This implementation interacts with browser-native APIs (like `Fetch` and
/// `Navigator.connection`) through Rust code compiled to WebAssembly (WASM).
final class NetworkProbesSource {
  /// Uses the browser's `Fetch` API with `redirect: manual` to detect captive portals.
  static Future<CaptivePortalStatus> checkForCaptivePortal(
          {required BigInt timeoutMs}) =>
      rust_captive_portal.checkForCaptivePortalWeb(timeoutMs: timeoutMs);

  /// Always returns false as browser security restrictions (CORS/SOP)
  /// prevent low-level DNS integrity checks on the web.
  static Future<bool> detectDnsHijacking({required String domain}) =>
      rust_dns.detectDnsHijackingWeb(domain: domain);

  /// Uses the browser's `Navigator.connection` API to determine connection type.
  static Future<(SecurityFlagsResult, ConnectionType)>
      detectSecurityAndNetworkType() =>
          rust_interface.detectSecurityAndNetworkTypeWeb();

  /// Performs a reachability probe using the browser's `Fetch` API.
  static Future<TargetReport> checkTarget({required NetworkTarget target}) =>
      rust_target.checkTarget(target: target);
}
