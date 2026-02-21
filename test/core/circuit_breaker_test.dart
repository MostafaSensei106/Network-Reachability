import 'package:flutter/widgets.dart';
import 'package:network_reachability/core/extensions/model_extensions.dart';
import 'package:network_reachability/network_reachability.dart';
import 'package:test/test.dart';
import '../mocks/mock_rust_api.dart';

void main() {
  late MockRustLibApi mockApi;

  setUpAll(() {
    WidgetsFlutterBinding.ensureInitialized();
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
          config: config.copyWith(
              resilience: resilience, cacheValidityMs: BigInt.zero));

      // Simulate a failing essential target
      mockApi.mockNetworkReport.targetReports = [
        TargetReport(
          label: 'essential',
          success: false,
          latencyMs: BigInt.from(0),
          isEssential: true,
        ),
      ];

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
          config: config.copyWith(
              resilience: resilience, cacheValidityMs: BigInt.zero));

      // 1. Force open state
      mockApi.mockNetworkReport.status = mockApi.mockNetworkReport.status.copyWith(isConnected: false);
      mockApi.mockNetworkReport.targetReports = [
        TargetReport(
            label: 'e',
            success: false,
            latencyMs: BigInt.zero,
            isEssential: true),
      ];
      await NetworkReachability.instance.check();

      expect(() => NetworkReachability.instance.guard(action: () async => 42),
          throwsA(isA<CircuitBreakerOpenException>()));

      // 2. Wait for cooldown
      await Future.delayed(const Duration(milliseconds: 150));

      // 3. Success probe: Half-Open -> Closed
      mockApi.mockNetworkReport.status = mockApi.mockNetworkReport.status.copyWith(isConnected: true);
      mockApi.mockNetworkReport.targetReports = [
        TargetReport(
            label: 'e',
            success: true,
            latencyMs: BigInt.from(30),
            isEssential: true),
      ];

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
          config: config.copyWith(
              resilience: resilience, cacheValidityMs: BigInt.zero));

      // 1. Force open
      mockApi.mockNetworkReport.status = mockApi.mockNetworkReport.status.copyWith(isConnected: false);
      mockApi.mockNetworkReport.targetReports = [
        TargetReport(
            label: 'e',
            success: false,
            latencyMs: BigInt.zero,
            isEssential: true),
      ];
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

    test('Non-essential target failure does NOT open the circuit', () async {
      final config = mockApi.mockDefaultConfig.copyWith(
        resilience: mockApi.mockDefaultResilienceConfig.copyWith(
          circuitBreakerThreshold: 1,
        ),
        cacheValidityMs: BigInt.zero,
      );
      await NetworkReachability.init(config: config);

      // Simulate failure of a NON-essential target
      mockApi.mockNetworkReport.targetReports = [
        TargetReport(
            label: 'non-e',
            success: false,
            latencyMs: BigInt.zero,
            isEssential: false),
      ];

      await NetworkReachability.instance.check();

      // Guard should NOT throw because it was not an essential failure
      final result =
          await NetworkReachability.instance.guard(action: () async => 42);
      expect(result, 42);
    });

    test('Successful essential target resets failure count', () async {
      final config = mockApi.mockDefaultConfig.copyWith(
        resilience: mockApi.mockDefaultResilienceConfig.copyWith(
          circuitBreakerThreshold: 2,
        ),
        cacheValidityMs: BigInt.zero,
      );
      await NetworkReachability.init(config: config);

      // 1. One failure
      mockApi.mockNetworkReport.targetReports = [
        TargetReport(
            label: 'e',
            success: false,
            latencyMs: BigInt.zero,
            isEssential: true),
      ];
      await NetworkReachability.instance.check();

      // 2. One success (resets the internal counter)
      mockApi.mockNetworkReport.targetReports = [
        TargetReport(
            label: 'e',
            success: true,
            latencyMs: BigInt.from(50),
            isEssential: true),
      ];
      await NetworkReachability.instance.check();

      // 3. Another failure (if count was not reset, this would open the circuit)
      mockApi.mockNetworkReport.targetReports = [
        TargetReport(
            label: 'e',
            success: false,
            latencyMs: BigInt.zero,
            isEssential: true),
      ];
      await NetworkReachability.instance.check();

      // Should NOT throw because count was reset to 0 in step 2
      final result =
          await NetworkReachability.instance.guard(action: () async => 42);
      expect(result, 42);
    });
  });
}
