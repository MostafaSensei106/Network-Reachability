// ignore_for_file: public_member_api_docs
import 'dart:async';

import '../core/constants/enums.dart';
import '../domain/entities/net_info.dart' as net_info;
import 'network_reachability_service.dart';

/// A compatibility facade that mirrors the `connectivity_plus` API.
///
/// This allows developers to use `network_reachability` as a drop-in
/// replacement for `connectivity_plus`.
class Connectivity {
  factory Connectivity() => _instance;

  Connectivity._();

  static final Connectivity _instance = Connectivity._();

  /// Discover network connectivity types that can be used.
  Future<List<ConnectivityResult>> checkConnectivity() async {
    final status = await NetworkReachability.instance.check();
    final typeAndFlags =
        await NetworkReachability.instance.detectSecurityAndNetworkType();

    return _mapToConnectivityResult(typeAndFlags.$2, status.status.isConnected);
  }

  /// Listen for active connectivity types changes.
  Stream<List<ConnectivityResult>> get onConnectivityChanged {
    return NetworkReachability.instance.onStatusChange
        .asyncMap((final status) async {
      final typeAndFlags =
          await NetworkReachability.instance.detectSecurityAndNetworkType();
      return _mapToConnectivityResult(typeAndFlags.$2, status.isConnected);
    });
  }

  List<ConnectivityResult> _mapToConnectivityResult(
      final net_info.ConnectionType type, final bool isConnected) {
    if (!isConnected) {
      return [ConnectivityResult.none];
    }
    switch (type) {
      case net_info.ConnectionType.wifi:
        return [ConnectivityResult.wifi];
      case net_info.ConnectionType.cellular:
        return [ConnectivityResult.mobile];
      case net_info.ConnectionType.ethernet:
        return [ConnectivityResult.ethernet];
      case net_info.ConnectionType.vpn:
        return [ConnectivityResult.vpn];
      case net_info.ConnectionType.bluetooth:
        return [ConnectivityResult.bluetooth];
      case net_info.ConnectionType.loopback:
      case net_info.ConnectionType.unknown:
        return [ConnectivityResult.other];
    }
  }
}
