import 'dart:async';

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
// import '../rust/api/probes/traceroute.dart' as traceroute_probe;

/// The main class for interacting with the network reachability engine.
///
/// This class provides a high-level API for performing network checks,
/// monitoring network status changes, and guarding operations based on network
/// quality and security requirements.
///
/// It operates as a singleton, which must be initialized once using the static
/// [init] method before use.
class NetworkReachability {
  static NetworkReachability? _instance;

  /// The singleton instance of the network reachability engine.
  ///
  /// An exception will be thrown if this is accessed before [init] is called.
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
  /// This must be called once before accessing the [instance]. It sets up the
  /// singleton with the given configuration. The user of this library is
  /// responsible for calling `RustLib.init()` *before* calling this method.
  ///
  /// - [config]: The configuration for the engine. If not provided, a default
  ///   configuration is fetched from the underlying Rust implementation.
  static Future<void> init({NetworkConfiguration? config}) async {
    if (_instance != null) {
      _instance?.dispose();
    }

    // Fetch the default config from Rust if none is provided.
    final finalConfig = config ?? await NetworkConfiguration.default_();

    _instance = NetworkReachability._(finalConfig);
  }

  /// A stream of network status updates.
  ///
  /// This stream emits a new [NetworkStatus] whenever a periodic check is
  /// completed successfully. The interval for these checks is defined by
  /// `checkIntervalMs` in the [NetworkConfiguration].
  ///
  /// If periodic checks are disabled (`checkIntervalMs` is zero), this stream
  /// will not emit any values.
  Stream<NetworkStatus> get onStatusChange => _statusController.stream;

  /// Performs a single, on-demand network check.
  ///
  /// This method orchestrates a comprehensive check based on the current
  /// configuration, including running probes against all defined targets.
  /// It also updates the internal circuit breaker state based on the outcome.
  ///
  /// Returns a [Future] that completes with a [NetworkReport] containing
  /// detailed information about the connection, including quality, latency,
  /// and security flags.
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
  /// Before executing the provided [action], this method performs a fresh
  /// network check and validates it against the active [NetworkConfiguration]
  /// and the [minQuality] requirement.
  ///
  /// It serves as a single point of truth to ensure that an operation is only
  /// attempted when the network state is acceptable.
  ///
  /// Throws a [PoorConnectionException], [SecurityException], or
  /// [CircuitBreakerOpenException] if the conditions are not met.
  ///
  /// - [action]: The function to execute if all network checks pass.
  /// - [minQuality]: The minimum required [ConnectionQuality] to proceed.
  ///   Defaults to [ConnectionQuality.good].
  ///
  /// ### Example: Basic Usage
  ///
  /// ```dart
  /// try {
  ///   final data = await NetworkReachability.instance.guard(
  ///     action: () => myApi.fetchSensitiveData(),
  ///   );
  ///   print('Data fetched successfully: $data');
  /// } on PoorConnectionException catch (e) {
  ///   print('Could not fetch data due to poor connection: $e');
  /// } on SecurityException catch (e) {
  ///   print('Security alert! Cannot perform action: $e');
  /// } on CircuitBreakerOpenException catch (e) {
  ///   print('Backend is unstable. Please try again later: $e');
  /// }
  /// ```
  ///
  /// ### Example: Requiring Excellent Quality
  ///
  /// ```dart
  /// try {
  ///   await NetworkReachability.instance.guard(
  ///     action: () => streamingService.start4KStream(),
  ///     minQuality: ConnectionQuality.excellent,
  ///   );
  /// } on PoorConnectionException {
  ///   // Handle case where connection is good, but not excellent
  ///   print('4K streaming requires an excellent connection.');
  /// }
  /// ```
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
    if (_config.security.detectDnsHijack && report.securityFlags.isDnsSpoofed) {
      throw SecurityException(SecurityAlert.dnsHijackDetected,
          'DNS hijacking was detected. Connection is insecure.');
    }
    if (_config.security.allowedInterfaces.isNotEmpty &&
        report.securityFlags.interfaceName.isNotEmpty) {
      final isAllowed = _config.security.allowedInterfaces.any(
          (prefix) => report.securityFlags.interfaceName.startsWith(prefix));
      if (!isAllowed) {
        throw SecurityException(SecurityAlert.unallowedInterface,
            'The active network interface (${report.securityFlags.interfaceName}) is not in the list of allowed interfaces.');
      }
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

  /// Checks for the presence of a captive portal.
  ///
  /// A captive portal is a web page that the user of a public-access network
  /// is obliged to view and interact with before access is granted.
  ///
  /// - [timeoutMs]: The maximum time to wait for the check to complete.
  ///
  /// Returns a [CaptivePortalStatus] indicating whether a portal was detected
  /// and the URL it redirected to.
  Future<CaptivePortalStatus> checkForCaptivePortal(
          {required BigInt timeoutMs}) =>
      captive_portal_probe.checkForCaptivePortal(timeoutMs: timeoutMs);

  /// Detects potential DNS hijacking.
  ///
  /// This is done by comparing the DNS resolution results from the system's
  /// resolver with a trusted DNS-over-HTTPS (DoH) resolver.
  ///
  /// - [domain]: The domain to use for the comparison (e.g., "google.com").
  ///
  /// Returns `true` if a mismatch is found, suggesting a potential hijack.
  Future<bool> detectDnsHijacking({required String domain}) =>
      dns_probe.detectDnsHijacking(domain: domain);

  /// Inspects system network interfaces to detect connection type and security flags.
  ///
  /// This can identify the active network interface and determine if it's a
  /// known VPN, cellular, WiFi, or Ethernet connection.
  ///
  /// Returns a tuple containing the [SecurityFlags] and the detected [ConnectionType].
  Future<(SecurityFlags, ConnectionType)> detectSecurityAndNetworkType() =>
      interface_probe.detectSecurityAndNetworkType();

  /// Performs a network check against a single, specified target.
  ///
  /// This is useful for testing a specific endpoint without running a full
  /// check against all configured targets.
  ///
  /// - [target]: The [NetworkTarget] to check.
  ///
  /// Returns a [TargetReport] with the results of the check.
  Future<TargetReport> checkTarget({required NetworkTarget target}) =>
      target_probe.checkTarget(target: target);

  /// Traces the network path to a specified host.
  ///
  /// This method sends packets with increasing TTLs to discover the
  /// intermediate routers between the device and the destination.
  ///
  /// - [host]: The destination host (e.g., "google.com").
  /// - [maxHops]: The maximum number of hops to trace.
  /// - [timeoutPerHopMs]: The timeout for each individual hop.
  ///
  /// Returns a list of [TraceHop] objects, each representing a step in the path.
  // Future<List<TraceHop>> traceRoute(
  //         {required String host,
  //         required int maxHops,
  //         required BigInt timeoutPerHopMs}) =>
  //     traceroute_probe.traceRoute(
  //         host: host, maxHops: maxHops, timeoutPerHopMs: timeoutPerHopMs);

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

  /// Disposes of the engine and cleans up all resources.
  ///
  /// This method cancels any active timers and closes the status stream.
  /// It should be called when the [NetworkReachability] instance is no longer needed
  /// to prevent memory leaks.
  void dispose() {
    _periodicTimer?.cancel();
    _statusController.close();
    _instance = null;
  }
}
