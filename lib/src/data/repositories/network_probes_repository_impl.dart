import '../../domain/repositories/network_probes_repository.dart';
import '../../rust/api/models/net_info.dart';
import '../../rust/api/models/report.dart';
import '../../rust/api/models/target.dart';
import '../sources/network_probes_source.dart';

/// Concrete implementation of [NetworkProbesRepository].
///
/// This class delegates all operations to the [NetworkProbesSource], which
/// is conditionally exported to use the correct platform-specific logic
/// (Native FFI vs Web WASM).
final class NetworkProbesRepositoryImpl implements NetworkProbesRepository {
  /// Default constructor for the repository implementation.
  const NetworkProbesRepositoryImpl();

  @override
  Future<CaptivePortalStatus> checkForCaptivePortal({
    required final BigInt timeoutMs,
  }) =>
      NetworkProbesSource.checkForCaptivePortal(timeoutMs: timeoutMs);

  @override
  Future<bool> detectDnsHijacking({required final String domain}) =>
      NetworkProbesSource.detectDnsHijacking(domain: domain);

  @override
  Future<(SecurityFlagsResult, ConnectionType)>
      detectSecurityAndNetworkType() =>
          NetworkProbesSource.detectSecurityAndNetworkType();

  @override
  Future<TargetReport> checkTarget({required final NetworkTarget target}) =>
      NetworkProbesSource.checkTarget(target: target);
}
