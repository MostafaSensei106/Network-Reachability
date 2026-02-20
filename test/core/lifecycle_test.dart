import 'package:flutter/widgets.dart';
import 'package:network_reachability/core/rust/api/models/config.dart';
import 'package:network_reachability/network_reachability.dart';
import 'package:network_reachability/core/rust/frb_generated.dart';
import 'package:test/test.dart';
import '../mocks/mock_rust_api.dart';

void main() {
  late MockRustLibApi mockApi;

  setUpAll(() {
    mockApi = MockRustLibApi();
    RustLib.initMock(api: mockApi);
    WidgetsFlutterBinding.ensureInitialized();
  });

  setUp(() {
    mockApi.reset();
  });

  tearDown(() {
    try {
      NetworkReachability.instance.dispose();
    } catch (_) {}
  });

  group('App Lifecycle Management', () {
    test('Periodic checks pause in background', () async {
      final config = mockApi.mockDefaultConfig.copyWith(
        checkIntervalMs: BigInt.from(100), // 100ms interval
      );
      await NetworkReachability.init(config: config);

      // 1. Initially check should trigger
      await Future.delayed(const Duration(milliseconds: 150));
      final countResumed = mockApi.checkCallCount;
      expect(countResumed, greaterThan(0));

      // 2. Pause the app
      NetworkReachability.instance.didChangeAppLifecycleState(AppLifecycleState.paused);
      
      // Clear counter
      mockApi.checkCallCount = 0;
      
      // Wait for a few intervals
      await Future.delayed(const Duration(milliseconds: 300));
      
      expect(mockApi.checkCallCount, 0, 
          reason: 'Periodic checks must stop when app is paused');

      // 3. Resume the app
      NetworkReachability.instance.didChangeAppLifecycleState(AppLifecycleState.resumed);
      
      await Future.delayed(const Duration(milliseconds: 150));
      expect(mockApi.checkCallCount, greaterThan(0), 
          reason: 'Periodic checks must resume when app is resumed');
    });
  });
}

extension on NetworkConfiguration {
  NetworkConfiguration copyWith({
    BigInt? checkIntervalMs,
  }) {
    return NetworkConfiguration(
      targets: targets,
      checkIntervalMs: checkIntervalMs ?? this.checkIntervalMs,
      cacheValidityMs: cacheValidityMs,
      qualityThreshold: qualityThreshold,
      security: security,
      resilience: resilience,
    );
  }
}
