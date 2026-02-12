import 'dart:async';

import 'package:network_reachability/core/rust/api/models/report.dart';
import 'package:network_reachability/network_reachability.dart';
import 'package:network_reachability/core/rust/frb_generated.dart';
import 'package:test/test.dart';

// A mock implementation of the Rust API to test the Dart logic in isolation.
class MockRustLibApi implements RustLibApi {
  // --- Controllable mock values ---
  late NetworkReport mockNetworkReport;
  late NetworkConfiguration mockDefaultConfig;
  late QualityThresholds mockDefaultQualityThresholds;
  late SecurityConfig mockDefaultSecurityConfig;
  late ResilienceConfig mockDefaultResilienceConfig;

  // --- Mocks for Probe Methods ---
  late CaptivePortalStatus mockCaptivePortalStatus;
  late bool mockDnsHijackingResult;
  late (SecurityFlags, ConnectionType) mockSecurityAndNetworkTypeResult;

  late TargetReport mockTargetReportProbe;
  late List<TraceHop> mockTraceRouteResult;

  MockRustLibApi() {
    // Initialize with a default "good" report
    reset();
  }

  void reset() {
    mockDefaultQualityThresholds = QualityThresholds(
      excellent: BigInt.from(50),
      great: BigInt.from(100),
      good: BigInt.from(200),
      moderate: BigInt.from(400),
      poor: BigInt.from(1000),
    );
    mockDefaultSecurityConfig = SecurityConfig(
      blockVpn: false,
      detectDnsHijack: false,
      allowedInterfaces: [],
    );
    mockDefaultResilienceConfig = ResilienceConfig(
      strategy: CheckStrategy.race,
      circuitBreakerThreshold: 3,
      numJitterSamples: 3,
      jitterThresholdPercent: 0.2,
      stabilityThershold: 70,
      criticalPacketLossPrecent: 15,
    );

    mockNetworkReport = NetworkReport(
      timestampMs: BigInt.from(DateTime.now().millisecondsSinceEpoch),
      status: NetworkStatus(
        isConnected: true,
        winnerTarget: 'cloudflare',
        quality: ConnectionQuality.excellent,
        latencyStats: LatencyStats(
          latencyMs: BigInt.from(30),
          jitterMs: BigInt.from(10),
          packetLossPercent: 0,
          stabilityScore: 85,
        ),
      ),
      connectionType: ConnectionType.wifi,
      securityFlags: SecurityFlags(
        isVpnDetected: false,
        isDnsSpoofed: false,
        isProxyDetected: false,
        interfaceName: 'wlan0',
      ),
      targetReports: [
        TargetReport(
          label: 'cloudflare',
          success: true,
          latencyMs: BigInt.from(30),
          isEssential: false,
        ),
      ],
    );

    mockDefaultConfig = NetworkConfiguration(
      targets: [
        NetworkTarget(
          label: 'default',
          host: '1.1.1.1',
          port: 53,
          protocol: TargetProtocol.tcp,
          timeoutMs: BigInt.from(1000),
          priority: 1,
          isEssential: false,
        )
      ],
      checkIntervalMs: BigInt.from(5000),
      qualityThreshold: mockDefaultQualityThresholds,
      security: mockDefaultSecurityConfig,
      resilience: mockDefaultResilienceConfig,
    );

    // Reset probe mocks
    mockCaptivePortalStatus = const CaptivePortalStatus(isCaptivePortal: false);
    mockDnsHijackingResult = false;
    mockSecurityAndNetworkTypeResult = (
      const SecurityFlags(
        isVpnDetected: false,
        isDnsSpoofed: false,
        isProxyDetected: false,
        interfaceName: 'eth0',
      ),
      ConnectionType.ethernet,
    );

    mockTargetReportProbe = TargetReport(
      label: 'test_target',
      success: true,
      latencyMs: BigInt.from(50),
      isEssential: false,
    );
    mockTraceRouteResult = [
      TraceHop(
          hopNumber: 1, ipAddress: '192.168.1.1', latencyMs: BigInt.from(10))
    ];
  }

  @override
  Future<NetworkReport> crateApiEngineCheckNetwork(
      {required NetworkConfiguration config}) async {
    return mockNetworkReport;
  }

  @override
  Future<NetworkConfiguration>
      crateApiModelsConfigNetworkConfigurationDefault() async {
    return mockDefaultConfig;
  }

  @override
  Future<QualityThresholds>
      crateApiModelsConfigQualityThresholdsDefault() async {
    return mockDefaultQualityThresholds;
  }

  @override
  Future<SecurityConfig> crateApiModelsConfigSecurityConfigDefault() async {
    return mockDefaultSecurityConfig;
  }

  @override
  Future<ResilienceConfig> crateApiModelsConfigResilienceConfigDefault() async {
    return mockDefaultResilienceConfig;
  }

  // --- Mock implementations for Probe methods ---

  @override
  Future<CaptivePortalStatus> crateApiProbesCaptivePortalCheckForCaptivePortal(
      {required BigInt timeoutMs}) async {
    return mockCaptivePortalStatus;
  }

  @override
  Future<bool> crateApiProbesDnsDetectDnsHijacking(
      {required String domain}) async {
    return mockDnsHijackingResult;
  }

  @override
  Future<(SecurityFlags, ConnectionType)>
      crateApiProbesInterfaceDetectSecurityAndNetworkType() async {
    return mockSecurityAndNetworkTypeResult;
  }

  @override
  Future<TargetReport> crateApiProbesTargetCheckTarget(
      {required NetworkTarget target}) async {
    // Return a copy with the label from the input target to simulate real behavior
    return mockTargetReportProbe.copyWith(label: target.label);
  }

  @override
  Future<List<TraceHop>> crateApiProbesTracerouteTraceRoute(
      {required String host,
      required int maxHops,
      required BigInt timeoutPerHopMs}) async {
    return mockTraceRouteResult;
  }

  // --- Unused mock methods ---
  @override
  dynamic noSuchMethod(Invocation invocation) {
    print('Unmocked method called: ${invocation.memberName}');
    // Return a default future to prevent tests from hanging
    return Future.value(null);
  }
}

void main() {
  late MockRustLibApi mockApi;

  setUpAll(() {
    // Initialize the Rust bridge in mock mode for all tests
    mockApi = MockRustLibApi();
    RustLib.initMock(api: mockApi);
  });

  tearDown(() {
    // Reset mocks and dispose the singleton after each test
    mockApi.reset();
    try {
      NetworkReachability.instance.dispose();
    } catch (_) {
      // Intentionally ignore if not initialized, as some tests verify this.
    }
  });

  group('NetworkReachability Initialization', () {
    test('throws if instance is accessed before init', () {
      // This test does not call init(), so it should throw.
      expect(() => NetworkReachability.instance, throwsA(isA<Exception>()));
    });

    test('initializes with default config from Rust', () async {
      await NetworkReachability.init();
      expect(NetworkReachability.instance, isA<NetworkReachability>());
    });

    test('initializes with a custom provided config', () async {
      final customConfig = NetworkConfiguration(
        targets: [],
        checkIntervalMs: BigInt.zero, // Disable periodic checks for this test
        qualityThreshold: await QualityThresholds.default_(),
        security: await SecurityConfig.default_(),
        resilience: await ResilienceConfig.default_(),
      );
      await NetworkReachability.init(config: customConfig);
      expect(NetworkReachability.instance, isA<NetworkReachability>());
    });
  });

  group('Guard Method', () {
    setUp(() async {
      // Initialize with a default config for this group
      final config = await NetworkConfiguration.default_();
      await NetworkReachability.init(
          config: config.copyWith(checkIntervalMs: BigInt.zero));
    });

    test('executes action when connection is good', () async {
      mockApi.mockNetworkReport = mockApi.mockNetworkReport
          .copyWith(status: ConnectionQuality.excellent.asStatus());

      final result = await NetworkReachability.instance
          .guard(action: () => Future.value(42));
      expect(result, 42);
    });

    test('throws PoorConnectionException when quality is below minimum',
        () async {
      mockApi.mockNetworkReport = mockApi.mockNetworkReport
          .copyWith(status: ConnectionQuality.poor.asStatus());

      expect(
        () => NetworkReachability.instance.guard(
          action: () => Future.value(42),
          minQuality: ConnectionQuality.good,
        ),
        throwsA(isA<PoorConnectionException>()),
      );
    });

    test('throws SecurityException when VPN is detected and blocked', () async {
      // Re-init with a specific security config for this test
      final config = (await NetworkConfiguration.default_()).copyWith(
        security: const SecurityConfig(
          blockVpn: true,
          detectDnsHijack: false,
          allowedInterfaces: [],
        ),
      );
      NetworkReachability.instance.dispose(); // Dispose previous instance
      await NetworkReachability.init(config: config);

      mockApi.mockNetworkReport = mockApi.mockNetworkReport.copyWith(
          securityFlags: const SecurityFlags(
              isVpnDetected: true,
              isDnsSpoofed: false,
              isProxyDetected: false,
              interfaceName: 'tun0'));

      expect(
        () =>
            NetworkReachability.instance.guard(action: () => Future.value(42)),
        throwsA(isA<SecurityException>()),
      );
    });

    test('throws CircuitBreakerOpenException after consecutive failures',
        () async {
      // Re-init with a specific resilience config for this test
      final config = (await NetworkConfiguration.default_()).copyWith(
        resilience: (await ResilienceConfig.default_())
            .copyWith(circuitBreakerThreshold: 2),
      );
      NetworkReachability.instance.dispose();
      await NetworkReachability.init(config: config);

      // Simulate first failure
      mockApi.mockNetworkReport =
          mockApi.mockNetworkReport.copyWith(targetReports: [
        TargetReport(
            label: 'essential',
            success: false,
            isEssential: true,
            error: 'timeout',
            latencyMs: BigInt.from(50)),
      ]);
      await NetworkReachability.instance.check();

      // Simulate second failure
      await NetworkReachability.instance.check();

      // Now, the guard should throw
      expect(
        () =>
            NetworkReachability.instance.guard(action: () => Future.value(42)),
        throwsA(isA<CircuitBreakerOpenException>()),
      );
    });
  });

  group('Status Stream', () {
    test('emits status on a periodic interval', () async {
      final config = (await NetworkConfiguration.default_()).copyWith(
        checkIntervalMs: BigInt.from(100), // Short interval for testing
      );
      await NetworkReachability.init(config: config);

      // Expect two emissions: one good, one poor
      mockApi.mockNetworkReport = mockApi.mockNetworkReport
          .copyWith(status: ConnectionQuality.excellent.asStatus());

      final completer = Completer<void>();
      final receivedQualities = <ConnectionQuality>[];

      final streamSub =
          NetworkReachability.instance.onStatusChange.listen((status) {
        receivedQualities.add(status.quality);
        if (receivedQualities.length == 2 &&
            receivedQualities.last == ConnectionQuality.poor) {
          completer.complete();
        }
      });

      // After a short delay, change the mock to trigger a different status
      await Future.delayed(const Duration(milliseconds: 150));
      mockApi.mockNetworkReport = mockApi.mockNetworkReport
          .copyWith(status: ConnectionQuality.poor.asStatus());

      await completer.future; // Wait for the second emission
      await streamSub.cancel();

      expect(
          receivedQualities,
          containsAllInOrder(
              [ConnectionQuality.excellent, ConnectionQuality.poor]));
    }, timeout: const Timeout(Duration(seconds: 2)));
  });

  group('Probe Methods', () {
    setUp(() async {
      // Initialize with a default config for this group, no periodic checks
      final config = await NetworkConfiguration.default_();
      await NetworkReachability.init(
          config: config.copyWith(checkIntervalMs: BigInt.zero));
    });

    test('checkForCaptivePortal calls the probe and returns status', () async {
      mockApi.mockCaptivePortalStatus =
          const CaptivePortalStatus(isCaptivePortal: true);
      final result = await NetworkReachability.instance
          .checkForCaptivePortal(timeoutMs: BigInt.from(1000));
      expect(result.isCaptivePortal, isTrue);
    });

    test('detectDnsHijacking calls the probe and returns boolean', () async {
      mockApi.mockDnsHijackingResult = true;
      final result = await NetworkReachability.instance
          .detectDnsHijacking(domain: 'example.com');
      expect(result, isTrue);
    });

    test('detectSecurityAndNetworkType calls the probe and returns tuple',
        () async {
      final expectedResult = (
        const SecurityFlags(
          isVpnDetected: true,
          isDnsSpoofed: false,
          isProxyDetected: false,
          interfaceName: 'tun0',
        ),
        ConnectionType.vpn,
      );
      mockApi.mockSecurityAndNetworkTypeResult = expectedResult;

      final result =
          await NetworkReachability.instance.detectSecurityAndNetworkType();
      expect(result.$1.isVpnDetected, isTrue);
      expect(result.$1.interfaceName, 'tun0');
      expect(result.$2, ConnectionType.vpn);
    });

    test('checkTarget calls the probe and returns a report', () async {
      final target = NetworkTarget(
          label: 'google-dns',
          host: '8.8.8.8',
          port: 53,
          protocol: TargetProtocol.udp,
          timeoutMs: BigInt.from(1000),
          priority: 1,
          isEssential: false);

      mockApi.mockTargetReportProbe = TargetReport(
          label: 'google-dns',
          success: true,
          latencyMs: BigInt.from(44),
          isEssential: false);

      final result =
          await NetworkReachability.instance.checkTarget(target: target);

      expect(result.success, isTrue);
      expect(result.label, 'google-dns');
      expect(result.latencyMs, BigInt.from(44));
    });

    // test('traceRoute calls the probe and returns hops', () async {
    //   final expectedHops = <TraceHop>[
    //     TraceHop(
    //         hopNumber: 1, ipAddress: 'hop1.com', latencyMs: BigInt.from(10)),
    //     TraceHop(
    //         hopNumber: 2, ipAddress: 'hop2.com', latencyMs: BigInt.from(20)),
    //   ];
    //   mockApi.mockTraceRouteResult = expectedHops;

    //   final result = await NetworkReachability.instance.traceRoute(
    //       host: 'example.com', maxHops: 5, timeoutPerHopMs: BigInt.from(500));

    //   expect(result, hasLength(2));
    //   expect(result.last.ipAddress, 'hop2.com');
    // });
  });
}

// --- Helper extensions for tests ---

extension on NetworkReport {
  NetworkReport copyWith({
    NetworkStatus? status,
    SecurityFlags? securityFlags,
    List<TargetReport>? targetReports,
  }) {
    return NetworkReport(
      timestampMs: timestampMs,
      status: status ?? this.status,
      connectionType: connectionType,
      securityFlags: securityFlags ?? this.securityFlags,
      targetReports: targetReports ?? this.targetReports,
    );
  }
}

extension on TargetReport {
  TargetReport copyWith({String? label}) {
    return TargetReport(
      label: label ?? this.label,
      success: success,
      latencyMs: latencyMs,
      isEssential: isEssential,
      error: error,
    );
  }
}

extension on ResilienceConfig {
  ResilienceConfig copyWith({
    int? circuitBreakerThreshold,
  }) {
    return ResilienceConfig(
      strategy: strategy,
      circuitBreakerThreshold:
          circuitBreakerThreshold ?? this.circuitBreakerThreshold,
      numJitterSamples: numJitterSamples,
      jitterThresholdPercent: jitterThresholdPercent,
      stabilityThershold: 70,
      criticalPacketLossPrecent: 15,
    );
  }
}

extension on NetworkConfiguration {
  NetworkConfiguration copyWith(
      {BigInt? checkIntervalMs,
      SecurityConfig? security,
      ResilienceConfig? resilience}) {
    return NetworkConfiguration(
      targets: targets,
      checkIntervalMs: checkIntervalMs ?? this.checkIntervalMs,
      qualityThreshold: qualityThreshold,
      security: security ?? this.security,
      resilience: resilience ?? this.resilience,
    );
  }
}

extension on ConnectionQuality {
  // Helper to create a status with a specific quality for mocking
  NetworkStatus asStatus() {
    return NetworkStatus(
      isConnected: this != ConnectionQuality.offline,
      quality: this,
      latencyStats: LatencyStats(
        latencyMs: BigInt.from(300), // 300ms target latency
        jitterMs: BigInt.from(50), // 50ms target jitter
        packetLossPercent: 0, stabilityScore: 0, // 0% packet loss target
      ),
      winnerTarget: 'mock',
    );
  }
}
