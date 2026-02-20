import 'dart:async';

import 'package:flutter/widgets.dart';
import 'package:network_reachability/core/constants/enums.dart';

import '../err/exceptions.dart';
import '../rust/api/engine.dart' as rust_engine;
import '../rust/api/models/config.dart';
import '../rust/api/models/net_info.dart';
import '../rust/api/models/report.dart';
import '../rust/api/models/target.dart';
import '../rust/api/probes/captive_portal.dart' as captive_portal_probe;
import '../rust/api/probes/dns.dart' as dns_probe;
import '../rust/api/probes/interface.dart' as interface_probe;
import '../rust/api/probes/target.dart' as target_probe;
import '../rust/api/probes/traceroute.dart' as traceroute_probe;

/// The main class for interacting with the network reachability engine.
class NetworkReachability with WidgetsBindingObserver {
  static NetworkReachability? _instance;

  /// The singleton instance of the network reachability engine.
  static NetworkReachability get instance {
    if (_instance == null) {
      throw Exception(
          'NetworkReachability has not been initialized. Call NetworkReachability.init() first.');
    }
    return _instance!;
  }

  late final NetworkConfiguration _config;
  final StreamController<NetworkStatus> _statusController =
      StreamController.broadcast();
  Timer? _periodicTimer;

  // --- State & Performance ---
  NetworkReport? _lastReport;
  DateTime? _lastReportTime;
  Future<NetworkReport>? _pendingCheck;

  // --- Circuit Breaker State ---
  int _consecutiveFailures = 0;
  CircuitBreakerState _circuitState = CircuitBreakerState.closed;
  DateTime? _circuitBreakerResetTime;

  /// Private constructor for the singleton.
  NetworkReachability._(NetworkConfiguration config) {
    _config = config;
    WidgetsBinding.instance.addObserver(this);
    _startPeriodicChecks();
  }

  /// Initializes the network reachability engine.
  static Future<void> init({NetworkConfiguration? config}) async {
    if (_instance != null) {
      _instance?.dispose();
    }
    final finalConfig = config ?? await NetworkConfiguration.default_();
    _instance = NetworkReachability._(finalConfig);
  }

  /// A stream of network status updates.
  Stream<NetworkStatus> get onStatusChange => _statusController.stream;

  /// Performs a network check with request coalescing and caching.
  Future<NetworkReport> check({bool forceRefresh = false}) async {
    // 1. Return cached report if still valid and not forcing refresh
    if (!forceRefresh &&
        _lastReport != null &&
        _lastReportTime != null &&
        DateTime.now().difference(_lastReportTime!).inMilliseconds <
            _config.cacheValidityMs.toInt()) {
      return _lastReport!;
    }

    // 2. Coalesce concurrent requests
    if (_pendingCheck != null) {
      return _pendingCheck!;
    }

    _pendingCheck = _performCheck();
    try {
      final report = await _pendingCheck!;
      return report;
    } finally {
      _pendingCheck = null;
    }
  }

  Future<NetworkReport> _performCheck() async {
    final report = await rust_engine.checkNetwork(config: _config);
    _lastReport = report;
    _lastReportTime = DateTime.now();

    _updateCircuitBreaker(report);
    return report;
  }

  void _updateCircuitBreaker(NetworkReport report) {
    final threshold = _config.resilience.circuitBreakerThreshold;
    if (threshold == 0) return;

    final essentialTargetFailed =
        report.targetReports.any((r) => r.isEssential && !r.success);

    if (essentialTargetFailed) {
      _consecutiveFailures++;
      if (_consecutiveFailures >= threshold) {
        _circuitState = CircuitBreakerState.open;
        _circuitBreakerResetTime = DateTime.now().add(Duration(
            milliseconds: _config.resilience.circuitBreakerCooldownMs.toInt()));
      }
    } else {
      // Any success on essential targets resets or closes the breaker
      _consecutiveFailures = 0;
      _circuitState = CircuitBreakerState.closed;
      _circuitBreakerResetTime = null;
    }
  }

  /// A wrapper to protect network-dependent operations.
  Future<T> guard<T>({
    required Future<T> Function() action,
    ConnectionQuality minQuality = ConnectionQuality.good,
  }) async {
    // 1. Handle Circuit Breaker State
    if (_circuitState == CircuitBreakerState.open) {
      if (_circuitBreakerResetTime != null &&
          DateTime.now().isBefore(_circuitBreakerResetTime!)) {
        final remaining = _circuitBreakerResetTime!.difference(DateTime.now());
        throw CircuitBreakerOpenException(
            'Circuit breaker is open due to recent failures. Retry after ${remaining.inSeconds}s',
            retryAfter: remaining);
      } else {
        // Transition to Half-Open to allow a probe
        _circuitState = CircuitBreakerState.halfOpen;
      }
    }

    // 2. Perform or reuse network check
    final report = await check();

    // 3. Validate security requirements
    if (_config.security.blockVpn && report.securityFlags.isVpnDetected) {
      throw SecurityException(
          SecurityAlert.vpnDetected, 'VPN connection is not allowed.');
    }
    if (_config.security.detectDnsHijack && report.securityFlags.isDnsSpoofed) {
      throw SecurityException(SecurityAlert.dnsHijackDetected,
          'DNS hijacking was detected. Connection is insecure.');
    }

    // 4. Validate quality requirements
    if (!report.status.isConnected ||
        report.status.quality.index > minQuality.index) {
      throw PoorConnectionException(
          'Connection quality (${report.status.quality.name}) is below required (${minQuality.name}).');
    }

    // 5. Success: Close circuit if it was half-open
    if (_circuitState == CircuitBreakerState.halfOpen) {
      _circuitState = CircuitBreakerState.closed;
      _consecutiveFailures = 0;
    }

    return await action();
  }

  /// Lifecycle Management for Battery Efficiency
  @override
  void didChangeAppLifecycleState(AppLifecycleState state) {
    if (state == AppLifecycleState.paused ||
        state == AppLifecycleState.inactive) {
      _stopPeriodicChecks();
    } else if (state == AppLifecycleState.resumed) {
      _startPeriodicChecks();
    }
  }

  void _startPeriodicChecks() {
    _stopPeriodicChecks();
    final interval = _config.checkIntervalMs;
    if (interval > BigInt.zero) {
      _periodicTimer =
          Timer.periodic(Duration(milliseconds: interval.toInt()), (_) async {
        final report = await check(forceRefresh: true);
        if (!_statusController.isClosed) {
          _statusController.add(report.status);
        }
      });
    }
  }

  void _stopPeriodicChecks() {
    _periodicTimer?.cancel();
    _periodicTimer = null;
  }

  /// Disposes of the engine and cleans up all resources.
  void dispose() {
    WidgetsBinding.instance.removeObserver(this);
    _stopPeriodicChecks();
    _statusController.close();
    _instance = null;
  }

  // --- Advanced Probes ---

  Future<CaptivePortalStatus> checkForCaptivePortal(
          {required BigInt timeoutMs}) =>
      captive_portal_probe.checkForCaptivePortal(timeoutMs: timeoutMs);

  Future<bool> detectDnsHijacking({required String domain}) =>
      dns_probe.detectDnsHijacking(domain: domain);

  Future<(SecurityFlags, ConnectionType)> detectSecurityAndNetworkType() =>
      interface_probe.detectSecurityAndNetworkType();

  Future<TargetReport> checkTarget({required NetworkTarget target}) =>
      target_probe.checkTarget(target: target);

  Future<List<TraceHop>> traceRoute({
    required String host,
    required int maxHops,
    required BigInt timeoutPerHopMs,
  }) =>
      traceroute_probe.traceRoute(
        host: host,
        maxHops: maxHops,
        timeoutPerHopMs: timeoutPerHopMs,
      );
}
