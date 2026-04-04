import 'dart:async';

import 'package:flutter/widgets.dart';

import '../core/constants/enums.dart';
import '../core/exceptions/exceptions.dart';
import '../data/repositories/network_probes_repository_impl.dart';
import '../domain/entities/entities.dart';
import '../domain/repositories/network_probes_repository.dart';
import '../rust/api/engine.dart' as rust_engine;

/// The central entry point for the Network Reachability library.
///
/// This service provides a robust, high-level API for monitoring and validating
/// network connectivity. It handles complex logic such as request coalescing,
/// caching, and resilience patterns like the circuit breaker.
///
/// ### Core Features:
/// * **Manual Checks:** Trigger precise network probes using [check].
/// * **Security Guard:** Protect sensitive operations from insecure networks using [guard].
/// * **Real-time Monitoring:** Listen to connectivity changes via [onStatusChange].
/// * **Lifecycle Awareness:** Automatically manages battery efficiency by pausing
///   background checks when the app is minimized.
///
/// ### Example Usage:
/// ```dart
///  1. Initialize at app startup
/// await NetworkReachability.init();
///
///  2. Perform a one-time check
/// final report = await NetworkReachability.instance.check();
/// print('Connected: ${report.status.isConnected}, Quality: ${report.status.quality}');
///
///  3. Protect an API call
/// try {
///   await NetworkReachability.instance.guard(
///     action: () => myApi.fetchData(),
///     minQuality: ConnectionQuality.good,
///   );
/// } on NetworkReachabilityException catch (e) {
///   print('Operation blocked: ${e.message}');
/// }
/// ```
final class NetworkReachability with WidgetsBindingObserver {
  /// Internal constructor. Use [init] to create the instance.
  NetworkReachability._(this._config, this._probesRepository) {
    WidgetsBinding.instance.addObserver(this);
    _startPeriodicChecks();
  }
  static NetworkReachability? _instance;

  /// Access the singleton instance of the network reachability service.
  ///
  /// Throws an [Exception] if [init] has not been called beforehand.
  static NetworkReachability get instance {
    if (_instance == null) {
      throw Exception(
          'NetworkReachability has not been initialized. Call NetworkReachability.init() first.');
    }
    return _instance!;
  }

  /// The active configuration for the network engine.
  late final NetworkConfiguration _config;

  /// The repository responsible for low-level platform probes.
  final NetworkProbesRepository _probesRepository;

  /// Controller for the broadcast stream of network status updates.
  final StreamController<NetworkStatus> _statusController =
      StreamController.broadcast();

  /// Internal timer for periodic monitoring.
  Timer? _periodicTimer;

  // --- State & Performance ---
  NetworkReport? _lastReport;
  DateTime? _lastReportTime;

  /// Holds the active probe future to implement "Request Coalescing" (Thundering Herd safety).
  Future<NetworkReport>? _pendingCheck;

  // --- Circuit Breaker State ---
  int _consecutiveFailures = 0;
  CircuitBreakerState _circuitState = CircuitBreakerState.closed;
  DateTime? _circuitBreakerResetTime;

  /// Initializes the Network Reachability engine.
  ///
  /// This method must be called once, typically inside `main()`, before accessing [instance].
  ///
  /// * [config]: Custom settings for targets, intervals, and security. If null,
  ///   defaults are fetched from the Rust backend.
  /// * [probesRepository]: Custom implementation of probes (useful for testing/mocking).
  ///
  /// Returns a [Future] that completes when the service is ready.
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

  /// A stream that emits [NetworkStatus] updates whenever a periodic check completes.
  ///
  /// This is useful for building reactive UIs that show "Offline" banners or
  /// quality indicators.
  Stream<NetworkStatus> get onStatusChange => _statusController.stream;

  /// Performs a comprehensive network check.
  ///
  /// To ensure efficiency, this method employs:
  /// 1. **Caching:** Returns a recent report if within the `cacheValidityMs` window.
  /// 2. **Request Coalescing:** If multiple callers trigger [check] simultaneously,
  ///    only one network probe is executed, and its result is shared among all callers.
  ///
  /// * [forceRefresh]: If true, ignores the cache and forces a new network probe.
  ///
  /// Returns a [NetworkReport] containing detailed statistics and security flags.
  Future<NetworkReport> check({bool forceRefresh = false}) async {
    // 1. Return cached report if still valid and not forcing refresh
    if (!forceRefresh &&
        _lastReport != null &&
        _lastReportTime != null &&
        DateTime.now().difference(_lastReportTime!).inMilliseconds <
            _config.cacheValidityMs.toInt()) {
      return _lastReport!;
    }

    // 2. Coalesce concurrent requests (Thundering Herd Protection)
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

  /// Internal core logic for executing the check via the Rust bridge.
  Future<NetworkReport> _performCheck() async {
    final report = await rust_engine.checkNetwork(config: _config);
    _lastReport = report;
    _lastReportTime = DateTime.now();

    _updateCircuitBreaker(report);
    return report;
  }

  /// Updates the Circuit Breaker state based on the success of essential targets.
  ///
  /// If essential targets fail repeatedly, the circuit opens to prevent further
  /// useless requests, saving device resources and avoiding backend hammering.
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

  /// Protects a network-sensitive [action] by validating the connection first.
  ///
  /// This is the recommended way to execute critical API calls. It performs
  /// multiple checks in sequence:
  /// 1. **Circuit Breaker:** Blocks immediately if the connection is known to be failing.
  /// 2. **Connectivity:** Ensures the device is actually online.
  /// 3. **Security:** Validates [SecurityConfig] (e.g., checks for VPN or DNS Hijacking).
  /// 4. **Quality:** Ensures the connection meets the [minQuality] requirement.
  ///
  /// ### Parameters:
  /// * [action]: The asynchronous operation to execute if the network is healthy.
  /// * [minQuality]: The minimum acceptable [ConnectionQuality]. Defaults to [ConnectionQuality.good].
  ///
  /// ### Throws:
  /// * [CircuitBreakerOpenException]: If the circuit is open due to recent failures.
  /// * [SecurityException]: If a VPN is detected (if blocked) or DNS spoofing is found.
  /// * [PoorConnectionException]: If the quality is below the requested threshold.
  ///
  /// Returns the result of the [action].
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

  /// Observes Flutter's app lifecycle to manage battery consumption.
  ///
  /// It automatically stops periodic checks when the app goes into the background
  /// and resumes them when the app returns to the foreground.
  @override
  void didChangeAppLifecycleState(AppLifecycleState state) {
    if (state == AppLifecycleState.paused ||
        state == AppLifecycleState.inactive) {
      _stopPeriodicChecks();
    } else if (state == AppLifecycleState.resumed) {
      _startPeriodicChecks();
    }
  }

  /// Initializes background monitoring with the interval defined in the config.
  void _startPeriodicChecks() {
    _stopPeriodicChecks();
    final interval = _config.checkIntervalMs;
    if (interval > BigInt.zero) {
      _scheduleNextCheck(Duration(milliseconds: interval.toInt()));
    }
  }

  /// Schedules the next background probe with an adaptive interval.
  ///
  /// If the connection is [ConnectionQuality.excellent], the interval is doubled
  /// (up to 30s) to save power. If quality drops, it reverts to the base interval.
  void _scheduleNextCheck(Duration delay) {
    _periodicTimer = Timer(delay, () async {
      final report = await check(forceRefresh: true);
      if (!_statusController.isClosed) {
        _statusController.add(report.status);
      }

      // Adaptive interval logic:
      var nextIntervalMs = _config.checkIntervalMs.toInt();
      if (report.status.quality == ConnectionQuality.excellent) {
        nextIntervalMs = (nextIntervalMs * 2).clamp(0, 30000); // Max 30s
      }

      if (_periodicTimer != null) {
        _scheduleNextCheck(Duration(milliseconds: nextIntervalMs));
      }
    });
  }

  /// Cancels the active background timer.
  void _stopPeriodicChecks() {
    _periodicTimer?.cancel();
    _periodicTimer = null;
  }

  /// Releases resources, removes observers, and shuts down the service.
  Future<void> dispose() async {
    WidgetsBinding.instance.removeObserver(this);
    _stopPeriodicChecks();
    await _statusController.close();
    _instance = null;
  }

  // --- Advanced Probes (Low-level API) ---

  /// Detects if the current network is a "Captive Portal" (e.g., public WiFi login page).
  ///
  /// [timeoutMs]: Maximum time to wait for the probe.
  Future<CaptivePortalStatus> checkForCaptivePortal(
          {required BigInt timeoutMs}) =>
      _probesRepository.checkForCaptivePortal(timeoutMs: timeoutMs);

  /// Checks for potential DNS tampering by comparing system results against trusted resolvers.
  ///
  /// [domain]: The domain to test (e.g., 'google.com').
  Future<bool> detectDnsHijacking({required String domain}) =>
      _probesRepository.detectDnsHijacking(domain: domain);

  /// Inspects active network interfaces to determine connectivity type and security status.
  ///
  /// Returns a tuple containing [SecurityFlagsResult] and [ConnectionType].
  Future<(SecurityFlagsResult, ConnectionType)>
      detectSecurityAndNetworkType() =>
          _probesRepository.detectSecurityAndNetworkType();

  /// Performs a targeted reachability probe against a single endpoint.
  ///
  /// [target]: The configuration for the endpoint to probe.
  Future<TargetReport> checkTarget({required NetworkTarget target}) =>
      _probesRepository.checkTarget(target: target);
}
