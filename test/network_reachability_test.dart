import 'dart:async';
import 'package:flutter/widgets.dart';
import 'package:network_reachability/network_reachability.dart';
import 'package:network_reachability/src/rust/frb_generated.dart';
import 'package:test/test.dart';
import 'mocks/mock_rust_api.dart';

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

  tearDown(() async {
    try {
      await NetworkReachability.instance.dispose();
    } catch (_) {}
  });

  group('NetworkReachability Core API', () {
    test('Initialization: throws if accessed before init', () {
      expect(() => NetworkReachability.instance, throwsA(isA<Exception>()));
    });

    test('Initialization: initializes with default config from Rust', () async {
      await NetworkReachability.init();
      expect(NetworkReachability.instance, isA<NetworkReachability>());
    });

    test('Probe methods: calls the underlying Rust API', () async {
      await NetworkReachability.init();

      // Captive portal
      mockApi.mockCaptivePortalStatus =
          const CaptivePortalStatus(isCaptivePortal: true);
      final captive = await NetworkReachability.instance
          .checkForCaptivePortal(timeoutMs: BigInt.from(1000));
      expect(captive.isCaptivePortal, isTrue);

      // DNS
      mockApi.mockDnsHijackingResult = true;
      final dns = await NetworkReachability.instance
          .detectDnsHijacking(domain: 'example.com');
      expect(dns, isTrue);

      // Interface
      final ifaceResult =
          await NetworkReachability.instance.detectSecurityAndNetworkType();
      expect(ifaceResult.$2, ConnectionType.ethernet);
    });

    test('Shared state and Observer pattern methods', () async {
      await NetworkReachability.init();

      // Before any check, default values are accessible
      expect(NetworkReachability.instance.lastReport, isNull);
      expect(NetworkReachability.instance.lastStatus, isNull);
      expect(NetworkReachability.instance.isConnected, isTrue);
      expect(NetworkReachability.instance.currentQuality,
          ConnectionQuality.excellent);

      // Perform a check to seed the shared state
      final report = await NetworkReachability.instance.check();
      expect(NetworkReachability.instance.lastReport, isNotNull);
      expect(NetworkReachability.instance.lastStatus, isNotNull);
      expect(NetworkReachability.instance.lastReport?.timestampMs,
          report.timestampMs);

      // Test Observer Pattern: listen
      final receivedStatuses = <NetworkStatus>[];
      final sub = NetworkReachability.instance.listen(receivedStatuses.add);

      // Test Observer Pattern: listenGuard
      var healthyTriggered = false;
      final guardSub = NetworkReachability.instance.listenGuard(
        onHealthy: (final _) => healthyTriggered = true,
      );

      expect(healthyTriggered, isFalse);
      expect(sub, isA<StreamSubscription<NetworkStatus>>());
      expect(guardSub, isA<StreamSubscription<NetworkStatus>>());

      await sub.cancel();
      await guardSub.cancel();
    });

    test('Guard method executes action when connection is healthy', () async {
      await NetworkReachability.init();
      final result = await NetworkReachability.instance.guard(
        action: () async => 'success_payload',
      );
      expect(result, 'success_payload');
    });
  });

  group('ConfigPreset and Presets', () {
    test('ConfigPreset enum has all expected profiles', () {
      expect(ConfigPreset.values, contains(ConfigPreset.default_));
      expect(ConfigPreset.values, contains(ConfigPreset.gaming));
      expect(ConfigPreset.values, contains(ConfigPreset.streaming));
      expect(ConfigPreset.values, contains(ConfigPreset.voIp));
      expect(ConfigPreset.values, contains(ConfigPreset.ioT));
      expect(ConfigPreset.values, contains(ConfigPreset.enterprise));
    });
  });
}
