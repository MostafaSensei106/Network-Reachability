import 'dart:async';
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

  group('onStatusChange Stream Monitoring', () {
    test('Periodic checks emit status changes', () async {
      final config = mockApi.mockDefaultConfig.copyWith(
        checkIntervalMs: BigInt.from(100), // 100ms interval
        cacheValidityMs: BigInt.zero,
      );

      // Initially mock excellent quality
      mockApi.mockNetworkReport.status = mockApi.mockNetworkReport.status.copyWith(
        quality: ConnectionQuality.excellent,
      );

      await NetworkReachability.init(config: config);

      final statusEvents = <NetworkStatus>[];
      final subscription =
          NetworkReachability.instance.onStatusChange.listen(statusEvents.add);

      // 1. Wait for first check
      await Future.delayed(const Duration(milliseconds: 150));
      expect(statusEvents.isNotEmpty, isTrue);
      expect(statusEvents.last.quality, ConnectionQuality.excellent);

      // 2. Change quality in mock
      mockApi.mockNetworkReport.status = mockApi.mockNetworkReport.status.copyWith(
        quality: ConnectionQuality.moderate,
      );

      // 3. Wait for next check
      await Future.delayed(const Duration(milliseconds: 150));
      expect(statusEvents.last.quality, ConnectionQuality.moderate);

      await subscription.cancel();
    });

    test('Stream closes upon disposal', () async {
      await NetworkReachability.init();
      final stream = NetworkReachability.instance.onStatusChange;

      final completer = Completer<void>();
      stream.listen(null, onDone: () => completer.complete());

      NetworkReachability.instance.dispose();

      expect(completer.future, completes);
    });
  });
}
