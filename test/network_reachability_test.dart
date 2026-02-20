import 'package:flutter/widgets.dart';
import 'package:network_reachability/network_reachability.dart';
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

  tearDown(() {
    try {
      NetworkReachability.instance.dispose();
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

    test('traceRoute: successfully calls probe', () async {
      await NetworkReachability.init();
      final hops = await NetworkReachability.instance.traceRoute(
        host: 'example.com',
        maxHops: 30,
        timeoutPerHopMs: BigInt.from(1000),
      );
      expect(hops, isNotEmpty);
      expect(hops.first.hopNumber, 1);
    });
  });
}
