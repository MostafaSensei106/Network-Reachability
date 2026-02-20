import 'package:network_reachability/network_reachability.dart';
import 'package:test/test.dart';
import '../mocks/mock_rust_api.dart';

void main() {
  late MockRustLibApi mockApi;

  setUpAll(() {
    mockApi = MockRustLibApi();
    RustLib.initMock(api: mockApi);
  });

  setUp(() {
    mockApi.reset();
  });

  tearDown(() {
    try {
      NetworkReachability.instance.dispose();
    } catch (_) {}
  });

  group('Circuit Breaker States', () {
    test('Transition: Closed -> Open after threshold failures', () async {
      // Set threshold to 2 failures
      final config = mockApi.mockDefaultConfig;
      final resilience = config.resilience.copyWith(
        circuitBreakerThreshold: 2,
      );
      await NetworkReachability.init(
          config: config.copyWith(resilience: resilience));

      // Simulate a failing essential target
      mockApi.mockNetworkReport = mockApi.mockNetworkReport.copyWith(
        targetReports: [
          TargetReport(
            label: 'essential',
            success: false,
            latencyMs: BigInt.from(0),
            isEssential: true,
          ),
        ],
      );

      // Check 1: Should still be closed
      await NetworkReachability.instance.check();
      await NetworkReachability.instance
          .guard(action: () async => 42); // Should not throw

      // Check 2: Should transition to Open
      await NetworkReachability.instance.check();

      // Guard should now throw immediately
      expect(
        () => NetworkReachability.instance.guard(action: () async => 42),
        throwsA(isA<CircuitBreakerOpenException>()),
        reason: 'Guard should throw when circuit is open',
      );
    });

    test('Transition: Open -> Half-Open after cooldown', () async {
      final config = mockApi.mockDefaultConfig;
      final resilience = config.resilience.copyWith(
        circuitBreakerThreshold: 1,
        circuitBreakerCooldownMs: BigInt.from(100), // 100ms cooldown
      );
      await NetworkReachability.init(
          config: config.copyWith(resilience: resilience));

      // 1. Force open state
      mockApi.mockNetworkReport = mockApi.mockNetworkReport.copyWith(
        targetReports: [
          TargetReport(
              label: 'e',
              success: false,
              latencyMs: BigInt.zero,
              isEssential: true),
        ],
      );
      await NetworkReachability.instance.check();

      expect(() => NetworkReachability.instance.guard(action: () async => 42),
          throwsA(isA<CircuitBreakerOpenException>()));

      // 2. Wait for cooldown
      await Future.delayed(const Duration(milliseconds: 150));

      // 3. Success probe: Half-Open -> Closed
      mockApi.mockNetworkReport = mockApi.mockNetworkReport.copyWith(
        targetReports: [
          TargetReport(
              label: 'e',
              success: true,
              latencyMs: BigInt.from(30),
              isEssential: true),
        ],
      );

      // Next guard should transition to Half-Open, allow probe, then Close circuit
      final result =
          await NetworkReachability.instance.guard(action: () async => 42);
      expect(result, 42,
          reason:
              'Guard should allow traffic in Half-Open state if probe succeeds');
    });

    test('Transition: Half-Open -> Open if probe fails', () async {
      final config = mockApi.mockDefaultConfig;
      final resilience = config.resilience.copyWith(
        circuitBreakerThreshold: 1,
        circuitBreakerCooldownMs: BigInt.from(100),
      );
      await NetworkReachability.init(
          config: config.copyWith(resilience: resilience));

      // 1. Force open
      mockApi.mockNetworkReport = mockApi.mockNetworkReport.copyWith(
        targetReports: [
          TargetReport(
              label: 'e',
              success: false,
              latencyMs: BigInt.zero,
              isEssential: true),
        ],
      );
      await NetworkReachability.instance.check();

      // 2. Wait for cooldown
      await Future.delayed(const Duration(milliseconds: 150));

      // 3. Keep failing: Half-Open -> Open
      // (Guard will try to probe, probe fails, moves back to Open and re-throws)
      expect(
          () => NetworkReachability.instance.guard(action: () async => 42),
          throwsA(
              isA<PoorConnectionException>()), // Fails probe, quality offline
          reason: 'Failing probe should keep/re-open the circuit');
    });
  });
}

extension on NetworkReport {
  NetworkReport copyWith({
    List<TargetReport>? targetReports,
  }) {
    return NetworkReport(
      timestampMs: timestampMs,
      status: status,
      connectionType: connectionType,
      securityFlags: securityFlags,
      targetReports: targetReports ?? this.targetReports,
    );
  }
}

extension on ResilienceConfig {
  ResilienceConfig copyWith({
    int? circuitBreakerThreshold,
    BigInt? circuitBreakerCooldownMs,
  }) {
    return ResilienceConfig(
      strategy: strategy,
      circuitBreakerThreshold:
          circuitBreakerThreshold ?? this.circuitBreakerThreshold,
      circuitBreakerCooldownMs:
          circuitBreakerCooldownMs ?? this.circuitBreakerCooldownMs,
      numJitterSamples: numJitterSamples,
      jitterThresholdPercent: jitterThresholdPercent,
      stabilityThershold: stabilityThershold,
      criticalPacketLossPrecent: criticalPacketLossPrecent,
    );
  }
}

extension on NetworkConfiguration {
  NetworkConfiguration copyWith({
    ResilienceConfig? resilience,
  }) {
    return NetworkConfiguration(
      targets: targets,
      checkIntervalMs: checkIntervalMs,
      cacheValidityMs: cacheValidityMs,
      qualityThreshold: qualityThreshold,
      security: security,
      resilience: resilience ?? this.resilience,
    );
  }
}
