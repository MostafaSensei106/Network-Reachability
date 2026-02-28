import 'package:network_reachability/rust/api/models/net_info.dart';
import 'package:network_reachability/rust/api/models/report.dart';
import 'package:network_reachability/rust/api/models/target.dart';

/// A stub data source for network probes that throws [UnimplementedError].
///
/// This implementation is used as a fallback when the platform is neither Native nor Web.
final class NetworkProbesSource {
  /// Stub for checking for captive portals.
  static Future<CaptivePortalStatus> checkForCaptivePortal(
          {required BigInt timeoutMs}) =>
      throw UnimplementedError(
          'checkForCaptivePortal is not implemented on this platform.');

  /// Stub for detecting DNS hijacking.
  static Future<bool> detectDnsHijacking({required String domain}) =>
      throw UnimplementedError(
          'detectDnsHijacking is not implemented on this platform.');

  /// Stub for detecting security and network type.
  static Future<(SecurityFlagsResult, ConnectionType)>
      detectSecurityAndNetworkType() => throw UnimplementedError(
          'detectSecurityAndNetworkType is not implemented on this platform.');

  /// Stub for checking a single target.
  static Future<TargetReport> checkTarget({required NetworkTarget target}) =>
      throw UnimplementedError(
          'checkTarget is not implemented on this platform.');
}
