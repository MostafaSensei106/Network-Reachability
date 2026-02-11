import 'dart:async';

import 'package:network_reachability/core/constants/enums.dart';

import '../err/exceptions.dart';
import '../rust/api/engine.dart' as rust_engine;
import '../rust/api/models/config.dart';
import '../rust/api/models/report.dart';

/// The main class for interacting with the network reachability engine.
///
/// This class provides a high-level API for network checks, monitoring, and guarding operations.
/// It should be initialized once using the static `init` method.
final class NetworkReachability {
  static NetworkReachability? _instance;

  /// The singleton instance of the network reachability engine.
  ///
  /// Must be initialized by calling `NetworkReachability.init()` before use.
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

  // --- Circuit Breaker State ---
  int _consecutiveFailures = 0;
  bool _isCircuitBreakerOpen = false;
  DateTime? _circuitBreakerOpenUntil;

  /// Private constructor for the singleton.
  NetworkReachability._(NetworkConfiguration config) {
    _config = config;
    _startPeriodicChecks();
  }

  /// Initializes the network reachability engine.
  ///
  /// The user of this library is responsible for calling `RustLib.init()` *before* this.
  ///
  /// [config]: The configuration for the engine. If not provided, a default configuration is used.
  static Future<void> init({NetworkConfiguration? config}) async {
    if (_instance != null) {
      // If called again, dispose the old instance to re-configure.
      _instance!.dispose();
    }

    // Fetch the default config from Rust if none is provided.
    final finalConfig = config ?? await NetworkConfiguration.default_();

    _instance = NetworkReachability._(finalConfig);
  }

  /// A stream of network status updates.
  ///
  /// Emits a new `NetworkStatus` whenever a periodic check is completed.
  Stream<NetworkStatus> get onStatusChange => _statusController.stream;

  /// Performs a single, one-off network check.
  ///
  /// Returns a `NetworkReport` with detailed information about the connection.
  Future<NetworkReport> check() async {
    final report = await rust_engine.checkNetwork(config: _config);

    // --- Update Circuit Breaker State ---
    final threshold = _config.resilience.circuitBreakerThreshold;
    if (threshold > 0) {
      final essentialTargetFailed =
          report.targetReports.any((r) => r.isEssential && !r.success);

      if (essentialTargetFailed) {
        _consecutiveFailures++;
        if (_consecutiveFailures >= threshold) {
          _isCircuitBreakerOpen = true;
          // Keep the circuit open for a short period (e.g., 1 minute)
          _circuitBreakerOpenUntil =
              DateTime.now().add(const Duration(minutes: 1));
        }
      } else {
        // Any success resets the counter
        _consecutiveFailures = 0;
        _isCircuitBreakerOpen = false;
        _circuitBreakerOpenUntil = null;
      }
    }

    return report;
  }

  /// A wrapper to protect network-dependent operations.
  ///
  /// Before executing the [action], this method performs a network check
  /// and validates it against the provided [requirements].
  ///
  /// Throws a [PoorConnectionException], [SecurityException], or [CircuitBreakerOpenException]
  /// if the conditions are not met.
  Future<T> guard<T>({
    required Future<T> Function() action,
    ConnectionQuality minQuality = ConnectionQuality.good,
  }) async {
    // 1. Check if circuit breaker is open
    if (_isCircuitBreakerOpen) {
      if (_circuitBreakerOpenUntil != null &&
          DateTime.now().isBefore(_circuitBreakerOpenUntil!)) {
        throw CircuitBreakerOpenException(
            'Circuit breaker is open due to recent backend failures. Please try again later.');
      } else {
        // Reset the breaker if the timeout has passed
        _isCircuitBreakerOpen = false;
        _consecutiveFailures = 0;
      }
    }

    // 2. Perform a fresh network check
    final report = await check();

    // 3. Validate security requirements
    if (_config.security.blockVpn && report.securityFlags.isVpnDetected) {
      throw SecurityException(
          SecurityAlert.vpnDetected, 'VPN connection is not allowed.');
    }

    // 4. Validate quality requirements
    if (!report.status.isConnected ||
        report.status.quality.index > minQuality.index) {
      throw PoorConnectionException(
          'Connection quality is below the required minimum (${minQuality.name}).');
    }

    // 5. If all checks pass, execute the action
    return await action();
  }

  /// Starts the periodic checks based on the configured interval.
  void _startPeriodicChecks() {
    _periodicTimer?.cancel();
    final interval = _config.checkIntervalMs;
    if (interval > BigInt.zero) {
      _periodicTimer =
          Timer.periodic(Duration(milliseconds: interval.toInt()), (_) async {
        final report = await check();
        if (!_statusController.isClosed) {
          _statusController.add(report.status);
        }
      });
    }
  }

  /// Disposes of the engine and cleans up resources.
  void dispose() {
    _periodicTimer?.cancel();
    _statusController.close();
    _instance = null;
  }
}
