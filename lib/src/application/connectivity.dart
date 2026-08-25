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

  // Pre-allocated constant result lists to avoid GC pressure on frequent events.
  static const _none = [ConnectivityResult.none];
  static const _wifi = [ConnectivityResult.wifi];
  static const _mobile = [ConnectivityResult.mobile];
  static const _ethernet = [ConnectivityResult.ethernet];
  static const _vpn = [ConnectivityResult.vpn];
  static const _bluetooth = [ConnectivityResult.bluetooth];
  static const _other = [ConnectivityResult.other];

  /// Cached transformed stream to avoid rebuilding the pipeline on every access.
  Stream<List<ConnectivityResult>>? _cachedStream;

  /// Discover network connectivity types that can be used.
  Future<List<ConnectivityResult>> checkConnectivity() async {
    final report = await NetworkReachability.instance.check();

    return _mapToConnectivityResult(
        report.connectionType, report.status.isConnected);
  }

  /// Listen for active connectivity types changes.
  ///
  /// The stream transformation is cached so repeated access to this getter
  /// reuses the same pipeline instead of creating new closures each time.
  Stream<List<ConnectivityResult>> get onConnectivityChanged {
    return _cachedStream ??=
        NetworkReachability.instance.onStatusChange.asyncMap(
      (final status) async {
        // Reuse the cached report instead of making a redundant FFI call
        final report = await NetworkReachability.instance.check();
        return _mapToConnectivityResult(
            report.connectionType, status.isConnected);
      },
    );
  }

  List<ConnectivityResult> _mapToConnectivityResult(
      final net_info.ConnectionType type, final bool isConnected) {
    if (!isConnected) {
      return _none;
    }
    switch (type) {
      case net_info.ConnectionType.wifi:
        return _wifi;
      case net_info.ConnectionType.cellular:
        return _mobile;
      case net_info.ConnectionType.ethernet:
        return _ethernet;
      case net_info.ConnectionType.vpn:
        return _vpn;
      case net_info.ConnectionType.bluetooth:
        return _bluetooth;
      case net_info.ConnectionType.loopback:
      case net_info.ConnectionType.unknown:
        return _other;
    }
  }
}
