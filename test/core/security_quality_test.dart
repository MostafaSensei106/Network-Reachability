import 'package:flutter/widgets.dart';
import 'package:network_reachability/core/constants/enums.dart';
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

  group('Guard: Security Enforcement', () {
    test('Throws SecurityException when VPN is blocked and detected', () async {
      final config = mockApi.mockDefaultConfig.copyWith(
        security: const SecurityConfig(blockVpn: true, detectDnsHijack: false),
        cacheValidityMs: BigInt.zero,
      );
      await NetworkReachability.init(config: config);

      mockApi.mockNetworkReport = mockApi.mockNetworkReport.copyWith(
        securityFlags: mockApi.mockNetworkReport.securityFlags.copyWith(
          isVpnDetected: true,
        ),
      );

      expect(
        () => NetworkReachability.instance.guard(action: () async => 42),
        throwsA(predicate((e) =>
            e is SecurityException && e.reason == SecurityAlert.vpnDetected)),
      );
    });

    test('Throws SecurityException when DNS hijack is detected', () async {
      final config = mockApi.mockDefaultConfig.copyWith(
        security: const SecurityConfig(blockVpn: false, detectDnsHijack: true),
        cacheValidityMs: BigInt.zero,
      );
      await NetworkReachability.init(config: config);

      mockApi.mockNetworkReport = mockApi.mockNetworkReport.copyWith(
        securityFlags: mockApi.mockNetworkReport.securityFlags.copyWith(
          isDnsSpoofed: true,
        ),
      );

      expect(
        () => NetworkReachability.instance.guard(action: () async => 42),
        throwsA(predicate((e) =>
            e is SecurityException &&
            e.reason == SecurityAlert.dnsHijackDetected)),
      );
    });
  });

  group('Guard: Quality Enforcement', () {
    test('Throws PoorConnectionException when quality is below threshold',
        () async {
      await NetworkReachability.init(
        config:
            mockApi.mockDefaultConfig.copyWith(cacheValidityMs: BigInt.zero),
      );

      mockApi.mockNetworkReport = mockApi.mockNetworkReport.copyWith(
        status: mockApi.mockNetworkReport.status.copyWith(
          quality: ConnectionQuality.poor,
        ),
      );

      // Guarding with 'good' quality should fail for 'poor' connection
      expect(
        () => NetworkReachability.instance.guard(
          action: () async => 42,
          minQuality: ConnectionQuality.good,
        ),
        throwsA(isA<PoorConnectionException>()),
      );
    });

    test('Throws PoorConnectionException when disconnected', () async {
      await NetworkReachability.init(
        config:
            mockApi.mockDefaultConfig.copyWith(cacheValidityMs: BigInt.zero),
      );

      mockApi.mockNetworkReport = mockApi.mockNetworkReport.copyWith(
        status: mockApi.mockNetworkReport.status.copyWith(
          isConnected: false,
        ),
      );

      expect(
        () => NetworkReachability.instance.guard(action: () async => 42),
        throwsA(isA<PoorConnectionException>()),
      );
    });

    test('Allows action when connection is sufficient', () async {
      await NetworkReachability.init();

      mockApi.mockNetworkReport = mockApi.mockNetworkReport.copyWith(
        status: mockApi.mockNetworkReport.status.copyWith(
          isConnected: true,
          quality: ConnectionQuality.excellent,
        ),
      );

      final result =
          await NetworkReachability.instance.guard(action: () async => 100);
      expect(result, 100);
    });
  });
}
