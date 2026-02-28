import 'dart:async';

import 'package:flutter/widgets.dart';
import '../core/constants/enums.dart';
import '../core/exceptions/exceptions.dart';
import '../domain/entities/entities.dart';
import '../domain/repositories/network_probes_repository.dart';
import '../data/repositories/network_probes_repository_impl.dart';
import '../../rust/api/engine.dart' as rust_engine;

/// The main class for interacting with the network reachability engine.
///
/// This service provides a high-level API for:
/// * Performing manual network checks via [check].
/// * Protecting sensitive operations using [guard].
/// * Monitoring real-time status changes through [onStatusChange].
///
/// It implements [WidgetsBindingObserver] to handle app lifecycle changes,
/// ensuring battery efficiency by pausing periodic checks in the background.
final class NetworkReachability with WidgetsBindingObserver {
  static NetworkReachability? _instance;

  /// The singleton instance of the network reachability engine.
  ///
  /// Throws an [Exception] if accessed before calling [init].
  static NetworkReachability get instance {
    if (_instance == null) {
      throw Exception(
          'NetworkReachability has not been initialized. Call NetworkReachability.init() first.');
    }
    return _instance!;
  }

  late final NetworkConfiguration _config;
  final NetworkProbesRepository _probesRepository;

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
  NetworkReachability._(this._config, this._probesRepository) {
    WidgetsBinding.instance.addObserver(this);
    _startPeriodicChecks();
  }

  /// Initializes the network reachability engine.
  ///
  /// This must be called once at app startup. It sets up the singleton
  /// with either a [config] or the default settings fetched from Rust.
  ///
  /// [config] An optional [NetworkConfiguration] to override defaults.
  /// [probesRepository] An optional [NetworkProbesRepository] for custom probe logic or mocking.
  ///
  /// Returns a [Future] that completes when initialization is done.
  static Future<void> init({
    NetworkConfiguration? config,
    NetworkProbesRepository? probesRepository,
  }) async {
    if (_instance != null) {
      _instance?.dispose();
    }
    final finalConfig = config ?? await NetworkConfiguration.default_();
    final finalRepository =
        probesRepository ?? const NetworkProbesRepositoryImpl();
    _instance = NetworkReachability._(finalConfig, finalRepository);
  }

  /// A stream of [NetworkStatus] updates emitted during periodic checks.
  Stream<NetworkStatus> get onStatusChange => _statusController.stream;

  /// Performs a network check with request coalescing and caching.
  ///
  /// If [forceRefresh] is false (default), it may return a cached report if it's
  /// within the `cacheValidityMs` window.
  ///
  /// This method is "Thundering Herd" safe; multiple simultaneous calls will
  /// result in a single underlying network probe.
  ///
  /// [forceRefresh] Whether to bypass the cache and force a new network probe.
  ///
  /// Returns a [Future] that resolves to a [NetworkReport].
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

  /// Internal implementation of the network check orchestration.
  Future<NetworkReport> _performCheck() async {
    final report = await rust_engine.checkNetwork(config: _config);
    _lastReport = report;
    _lastReportTime = DateTime.now();

    _updateCircuitBreaker(report);
    return report;
  }

  /// Updates the internal circuit breaker state machine based on target success/failure.
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

  /// A wrapper to protect network-dependent operations (e.g., API calls).
  ///
  /// This method performs a multi-stage validation:
  /// 1. Checks if the circuit breaker is [CircuitBreakerState.open].
  /// 2. Performs/reuses a network [check].
  /// 3. Validates against [SecurityConfig] (VPN, DNS Hijack).
  /// 4. Validates against [minQuality].
  ///
  /// [action] The asynchronous operation to protect.
  /// [minQuality] The minimum [ConnectionQuality] required to proceed.
  ///
  /// Throws [CircuitBreakerOpenException], [SecurityException], or [PoorConnectionException]
  /// if any check fails.
  ///
  /// Returns the result of [action] if all checks pass.
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
    if (_config.security.blockVpn && report.securityFlagsResult.isVpnDetected) {
      throw SecurityException(
          SecurityAlert.vpnDetected, 'VPN connection is not allowed.');
    }
    if (_config.security.detectDnsHijack &&
        report.securityFlagsResult.isDnsSpoofed) {
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

  /// Automatically manages periodic checks based on the Flutter app lifecycle.
  @override
  void didChangeAppLifecycleState(AppLifecycleState state) {
    if (state == AppLifecycleState.paused ||
        state == AppLifecycleState.inactive) {
      _stopPeriodicChecks();
    } else if (state == AppLifecycleState.resumed) {
      _startPeriodicChecks();
    }
  }

  /// Starts the [Timer] for background monitoring with an adaptive interval.
  void _startPeriodicChecks() {
    _stopPeriodicChecks();
    final interval = _config.checkIntervalMs;
    if (interval > BigInt.zero) {
      _scheduleNextCheck(Duration(milliseconds: interval.toInt()));
    }
  }

  /// Schedules the next periodic check with a dynamic delay.
  void _scheduleNextCheck(Duration delay) {
    _periodicTimer = Timer(delay, () async {
      final report = await check(forceRefresh: true);
      if (!_statusController.isClosed) {
        _statusController.add(report.status);
      }

      // Adaptive interval logic:
      // 1. If quality is Excellent, we can afford to check less frequently (e.g., 2x interval).
      // 2. If quality is Poor or Offline, we stick to the base interval to detect recovery.
      int nextIntervalMs = _config.checkIntervalMs.toInt();
      if (report.status.quality == ConnectionQuality.excellent) {
        nextIntervalMs = (nextIntervalMs * 2).clamp(0, 30000); // Max 30s
      }

      // Only schedule next if we haven't been stopped
      if (_periodicTimer != null) {
        _scheduleNextCheck(Duration(milliseconds: nextIntervalMs));
      }
    });
  }

  /// Stops the background monitoring timer.
  void _stopPeriodicChecks() {
    _periodicTimer?.cancel();
    _periodicTimer = null;
  }

  /// Disposes of the engine, removes lifecycle observers, and closes streams.
  void dispose() {
    WidgetsBinding.instance.removeObserver(this);
    _stopPeriodicChecks();
    _statusController.close();
    _instance = null;
  }

  // --- Advanced Probes ---

  /// Checks for the presence of a captive portal using a non-SSL probe.
  ///
  /// [timeoutMs] The maximum time to wait for the probe response.
  ///
  /// Returns a [Future] that resolves to a [CaptivePortalStatus].
  Future<CaptivePortalStatus> checkForCaptivePortal(
          {required BigInt timeoutMs}) =>
      _probesRepository.checkForCaptivePortal(timeoutMs: timeoutMs);

  /// Detects potential DNS hijacking by comparing system resolution vs Cloudflare.
  ///
  /// [domain] The domain name to test resolution for.
  ///
  /// Returns a [Future] that resolves to `true` if hijacking is detected.
  Future<bool> detectDnsHijacking({required String domain}) =>
      _probesRepository.detectDnsHijacking(domain: domain);

  /// Inspects system network interfaces to determine connection type and security flags.
  ///
  /// Returns a [Future] that resolves to a tuple containing [SecurityFlagsResult] and [ConnectionType].
  Future<(SecurityFlagsResult, ConnectionType)>
      detectSecurityAndNetworkType() =>
          _probesRepository.detectSecurityAndNetworkType();

  /// Performs a low-level reachability check against a single [target].
  ///
  /// [target] The [NetworkTarget] to probe.
  ///
  /// Returns a [Future] that resolves to a [TargetReport].
  Future<TargetReport> checkTarget({required NetworkTarget target}) =>
      _probesRepository.checkTarget(target: target);
}
