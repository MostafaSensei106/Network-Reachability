import '../../../rust/api/models/net_info.dart';
import '../../../rust/api/models/report.dart';
import '../../../rust/api/models/target.dart';

/// A **Stub** implementation of the network probe data source.
///
/// This class exists solely to satisfy the Dart compiler on platforms where
/// the specific Native or Web implementations are not available.
///
/// All methods throw an [UnimplementedError] if called.
final class NetworkProbesSource {
  /// Stub for checking for captive portals.
  static Future<CaptivePortalStatus> checkForCaptivePortal({
    required final BigInt timeoutMs,
  }) =>
      throw UnimplementedError(
        'checkForCaptivePortal is not implemented on this platform.',
      );

  /// Stub for detecting DNS hijacking.
  static Future<bool> detectDnsHijacking({required final String domain}) =>
      throw UnimplementedError(
        'detectDnsHijacking is not implemented on this platform.',
      );

  /// Stub for detecting security and network type.
  static Future<(SecurityFlagsResult, ConnectionType)>
      detectSecurityAndNetworkType() => throw UnimplementedError(
            'detectSecurityAndNetworkType is not implemented on this platform.',
          );

  /// Stub for checking a single target.
  static Future<TargetReport> checkTarget({
    required final NetworkTarget target,
  }) =>
      throw UnimplementedError(
        'checkTarget is not implemented on this platform.',
      );
}
