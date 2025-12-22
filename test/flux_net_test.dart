import 'package:flutter_test/flutter_test.dart';
import 'package:flux_net/flux_net.dart';
import 'package:flux_net/flux_net_platform_interface.dart';
import 'package:flux_net/flux_net_method_channel.dart';
import 'package:plugin_platform_interface/plugin_platform_interface.dart';

class MockFluxNetPlatform
    with MockPlatformInterfaceMixin
    implements FluxNetPlatform {
  @override
  Future<String?> getPlatformVersion() => Future.value('42');
}

void main() {
  final FluxNetPlatform initialPlatform = FluxNetPlatform.instance;

  test('$MethodChannelFluxNet is the default instance', () {
    expect(initialPlatform, isInstanceOf<MethodChannelFluxNet>());
  });

  test('getPlatformVersion', () async {
    FluxNet fluxNetPlugin = FluxNet();
    MockFluxNetPlatform fakePlatform = MockFluxNetPlatform();
    FluxNetPlatform.instance = fakePlatform;

    expect(await fluxNetPlugin.getPlatformVersion(), '42');
  });
}
