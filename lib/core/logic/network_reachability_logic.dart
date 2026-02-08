// lib/logic/network_reachability_logic.dart
import 'dart:async';

import 'package:network_reachability/core/err/exceptions.dart';

import '../rust/frb_generated.dart'; // Assumed to be correctly generated
import '../rust/api/models.dart'; // Assumed to be correctly generated
import '../rust/api/engine.dart'; // Assumed to be correctly generated

/// The main entry point for interacting with the network reachability library.
class NetworkReachability {
  static final NetworkReachability _instance = NetworkReachability._internal();
  factory NetworkReachability() => _instance;

  NetworkReachability._internal();

  // Initialize with dummy values, will be overwritten by default_() call after init
  NetwrokConfiguration _currentConfig = NetwrokConfiguration(
    targets: const [],
    checkStrategy: CheckStrategy.race,
    qualityThreshold: QualityThresholds(
      excellent: BigInt.zero,
      great: BigInt.zero,
      good: BigInt.zero,
      moderate: BigInt.zero,
      poor: BigInt.zero,
    ),
    checkIntervalMs: BigInt.zero,
    blockRequestWhenPoor: false,
    numJitterSamples: 0,
    jitterThresholdPercent: 0.0,
  );

  final _onStatusChangeController = StreamController<NetworkReport>.broadcast();
  Stream<NetworkReport> get onStatusChange => _onStatusChangeController.stream;

  /// Initializes the Rust library.
  /// This must be called once before using any other functionality.
  static Future<void> init() async {
    // Ensure `flutter_rust_bridge` is initialized
    await RustLib.init();

    // Fetch default configuration after init
    _instance._currentConfig = await NetwrokConfiguration.default_();
  }

  /// Configures the network reachability checks.
  Future<void> configure({
    List<NetworkTarget>? targets,
    CheckStrategy? strategy,
    QualityThresholds? thresholds,
    BigInt? checkIntervalMs,
    bool? blockRequestWhenPoor,
    int? numJitterSamples,
    double? jitterThresholdPercent,
  }) async {
    // Note: This is a simplified configuration. A real implementation might merge or replace.
    // For now, it replaces the entire configuration or uses existing values if not provided.
    _currentConfig = NetwrokConfiguration(
      targets: targets ?? _currentConfig.targets,
      checkStrategy: strategy ?? _currentConfig.checkStrategy,
      qualityThreshold: thresholds ?? _currentConfig.qualityThreshold,
      checkIntervalMs: checkIntervalMs ?? _currentConfig.checkIntervalMs,
      blockRequestWhenPoor:
          blockRequestWhenPoor ?? _currentConfig.blockRequestWhenPoor,
      numJitterSamples: numJitterSamples ?? _currentConfig.numJitterSamples,
      jitterThresholdPercent:
          jitterThresholdPercent ?? _currentConfig.jitterThresholdPercent,
    );
  }

  /// Performs an immediate network reachability check.
  Future<NetworkReport> check() async {
    final report = await checkNetwork(config: _currentConfig);
    _onStatusChangeController.add(report);
    return report;
  }

  /// Starts a periodic network check and emits reports to [onStatusChange].
  ///
  /// Note: This is a placeholder for demonstration. A robust implementation would
  /// manage subscriptions and potential background tasks more carefully.
  StreamSubscription? _monitoringSubscription;
  void startMonitoring() {
    _monitoringSubscription?.cancel(); // Cancel any existing subscription

    _monitoringSubscription = Stream.periodic(
      Duration(milliseconds: _currentConfig.checkIntervalMs.toInt()),
      (_) => check(),
    ).listen((_) {}); // The check() call already adds to the controller.
  }

  /// Stops the periodic network monitoring.
  void stopMonitoring() {
    _monitoringSubscription?.cancel();
    _monitoringSubscription = null;
  }

  /// Executes a Future, but only if the network quality is not Poor or Dead.
  /// Throws [PoorConnectionException] if network quality is insufficient.
  Future<T> guard<T>(Future<T> Function() action) async {
    final currentReport = await check();
    if (currentReport.status.quality == ConnectionQuality.poor ||
        currentReport.status.quality == ConnectionQuality.dead) {
      throw PoorConnectionException(currentReport);
    }
    return action();
  }

  /// Scans the local network for active devices within a given subnet.
  ///
  /// [subnet] is the network range in CIDR notation (e.g., "192.168.1.0/24").
  /// [scanPort] is the TCP port to try connecting to on each host.
  /// [timeoutMs] is the timeout for each connection attempt in milliseconds.
  Future<List<LocalDevice>> scanLocalNetwork({
    String subnet = "89.207.132.170/24",
    int scanPort = 80,
    required BigInt timeoutMs,
  }) async {
    return await scanLocalNetwork(
      subnet: subnet,
      scanPort: scanPort,
      timeoutMs: timeoutMs,
    );
  }

  /// Checks if the current network connection is behind a captive portal.
  ///
  /// [timeoutMs] is the timeout for the HTTP request in milliseconds.
  Future<CaptivePortalStatus> checkForCaptivePortal({
    int timeoutMs = 5000,
  }) async {
    return await checkForCaptivePortal(timeoutMs: timeoutMs);
  }

  /// Performs a simplified traceroute to a given host.
  ///
  /// [host] is the target IP address or domain name.
  /// [maxHops] is the maximum number of hops to trace.
  /// [timeoutPerHopMs] is the timeout for each hop in milliseconds.
  Future<List<TraceHop>> traceRoute({
    required String host,
    int maxHops = 30,
    int timeoutPerHopMs = 1000,
  }) async {
    return await traceRoute(
      host: host,
      maxHops: maxHops,
      timeoutPerHopMs: timeoutPerHopMs,
    );
  }
}
