import 'package:network_reachability/rust/api/models/net_info.dart';
import 'package:network_reachability/rust/api/models/report.dart';
import 'package:network_reachability/rust/api/models/target.dart';

import '../../domain/repositories/network_probes_repository.dart';
import '../sources/network_probes_source.dart';

/// Implementation of [NetworkProbesRepository] using platform-specific data sources.
///
/// This repository acts as a bridge between the domain layer and the data sources,
/// delegating probe operations to [NetworkProbesSource].
final class NetworkProbesRepositoryImpl implements NetworkProbesRepository {
  /// Default constructor for the repository implementation.
  const NetworkProbesRepositoryImpl();

  @override
  Future<CaptivePortalStatus> checkForCaptivePortal(
          {required BigInt timeoutMs}) =>
      NetworkProbesSource.checkForCaptivePortal(timeoutMs: timeoutMs);

  @override
  Future<bool> detectDnsHijacking({required String domain}) =>
      NetworkProbesSource.detectDnsHijacking(domain: domain);

  @override
  Future<(SecurityFlagsResult, ConnectionType)>
      detectSecurityAndNetworkType() =>
          NetworkProbesSource.detectSecurityAndNetworkType();

  @override
  Future<TargetReport> checkTarget({required NetworkTarget target}) =>
      NetworkProbesSource.checkTarget(target: target);
}
