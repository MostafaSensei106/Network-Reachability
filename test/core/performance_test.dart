import 'package:flutter/widgets.dart';
import 'package:network_reachability/core/logic/network_reachability_logic.dart';
import 'package:network_reachability/core/rust/frb_generated.dart';
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

  group('Performance - Caching & Coalescing', () {
    test('Request Coalescing: Multiple simultaneous calls result in one probe',
        () async {
      final config = mockApi.mockDefaultConfig;
      await NetworkReachability.init(config: config);

      // Fire 3 calls simultaneously
      final futures = Future.wait([
        NetworkReachability.instance.check(),
        NetworkReachability.instance.check(),
        NetworkReachability.instance.check(),
      ]);

      await futures;
      expect(mockApi.checkCallCount, 1,
          reason: 'Only 1 probe should be triggered for concurrent requests');
    });

    test('Caching: Subsequent calls within validity window use cache',
        () async {
      final config = mockApi.mockDefaultConfig; // Cache is 500ms
      await NetworkReachability.init(config: config);

      await NetworkReachability.instance.check(); // Probe 1
      await NetworkReachability.instance.check(); // Cache hit
      await NetworkReachability.instance.check(); // Cache hit

      expect(mockApi.checkCallCount, 1,
          reason: 'Subsequent calls should use the cache');

      // Wait for cache to expire
      await Future.delayed(const Duration(milliseconds: 600));
      await NetworkReachability.instance.check(); // Probe 2

      expect(mockApi.checkCallCount, 2,
          reason: 'A new probe should trigger after cache expiration');
    });

    test('forceRefresh bypasses cache', () async {
      final config = mockApi.mockDefaultConfig;
      await NetworkReachability.init(config: config);

      await NetworkReachability.instance.check(); // Probe 1
      await NetworkReachability.instance.check(forceRefresh: true); // Probe 2

      expect(mockApi.checkCallCount, 2);
    });
  });
}
