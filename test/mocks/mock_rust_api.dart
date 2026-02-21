import 'dart:async';
import 'package:network_reachability/core/rust/api/models/config.dart';
import 'package:network_reachability/core/rust/api/models/net_info.dart';
import 'package:network_reachability/core/rust/api/models/report.dart';
import 'package:network_reachability/core/rust/api/models/target.dart';
import 'package:network_reachability/core/rust/frb_generated.dart';

class MockSecurityFlagsResult implements SecurityFlagsResult {
  @override
  String interfaceName;
  @override
  bool isDnsSpoofed;
  @override
  bool isProxyDetected;
  @override
  bool isVpnDetected;

  @override
  void dispose() {}
  @override
  bool get isDisposed => false;

  MockSecurityFlagsResult({
    required this.interfaceName,
    required this.isDnsSpoofed,
    required this.isProxyDetected,
    required this.isVpnDetected,
  });
}

class MockNetworkReport implements NetworkReport {
  @override
  ConnectionType connectionType;
  @override
  SecurityFlagsResult securityFlagsResult;
  @override
  NetworkStatus status;
  @override
  List<TargetReport> targetReports;
  @override
  BigInt timestampMs;

  @override
  void dispose() {}
  @override
  bool get isDisposed => false;

  MockNetworkReport({
    required this.connectionType,
    required this.securityFlagsResult,
    required this.status,
    required this.targetReports,
    required this.timestampMs,
  });
}

class MockRustLibApi implements RustLibApi {
  int checkCallCount = 0;

  late MockNetworkReport mockNetworkReport;
  late NetworkConfiguration mockDefaultConfig;
  late QualityThresholds mockDefaultQualityThresholds;
  late SecurityConfig mockDefaultSecurityConfig;
  late ResilienceConfig mockDefaultResilienceConfig;

  late CaptivePortalStatus mockCaptivePortalStatus;
  late bool mockDnsHijackingResult;
  late (SecurityFlagsResult, ConnectionType) mockSecurityAndNetworkTypeResult;
  late TargetReport mockTargetReportProbe;
  late List<TraceHop> mockTraceRouteResult;

  MockRustLibApi() {
    reset();
  }

  void reset() {
    checkCallCount = 0;

    mockDefaultQualityThresholds = QualityThresholds(
      excellent: BigInt.from(50),
      great: BigInt.from(100),
      good: BigInt.from(200),
      moderate: BigInt.from(400),
      poor: BigInt.from(1000),
    );
    mockDefaultSecurityConfig = const SecurityConfig(
      blockVpn: false,
      detectDnsHijack: false,
    );
    mockDefaultResilienceConfig = ResilienceConfig(
      strategy: CheckStrategy.race,
      circuitBreakerThreshold: 3,
      circuitBreakerCooldownMs: BigInt.from(1000), // 1s for faster testing
      numJitterSamples: 3,
      jitterThresholdPercent: 0.2,
      stabilityThershold: 70,
      criticalPacketLossPrecent: 15.0,
    );

    mockNetworkReport = MockNetworkReport(
      timestampMs: BigInt.from(DateTime.now().millisecondsSinceEpoch),
      status: NetworkStatus(
        isConnected: true,
        winnerTarget: 'cloudflare',
        quality: ConnectionQuality.excellent,
        latencyStats: LatencyStats(
          latencyMs: BigInt.from(30),
          jitterMs: BigInt.from(10),
          packetLossPercent: 0.0,
          stabilityScore: 85,
        ),
      ),
      connectionType: ConnectionType.wifi,
      securityFlagsResult: MockSecurityFlagsResult(
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
          isEssential: true,
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
          isEssential: true,
        )
      ],
      checkIntervalMs: BigInt.from(5000),
      cacheValidityMs: BigInt.from(500), // 500ms cache
      qualityThreshold: mockDefaultQualityThresholds,
      security: mockDefaultSecurityConfig,
      resilience: mockDefaultResilienceConfig,
    );

    mockCaptivePortalStatus = const CaptivePortalStatus(isCaptivePortal: false);
    mockDnsHijackingResult = false;
    mockSecurityAndNetworkTypeResult = (
      MockSecurityFlagsResult(
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
    checkCallCount++;
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
  Future<(SecurityFlagsResult, ConnectionType)>
      crateApiProbesInterfaceDetectSecurityAndNetworkType() async {
    return mockSecurityAndNetworkTypeResult;
  }

  @override
  Future<TargetReport> crateApiProbesTargetCheckTarget(
      {required NetworkTarget target}) async {
    return mockTargetReportProbe;
  }

  @override
  Future<List<TraceHop>> crateApiProbesTracerouteTraceRoute(
      {required String host,
      required int maxHops,
      required BigInt timeoutPerHopMs}) async {
    return mockTraceRouteResult;
  }

  @override
  dynamic noSuchMethod(Invocation invocation) => super.noSuchMethod(invocation);
}

