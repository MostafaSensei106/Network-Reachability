import 'flux_net_platform_interface.dart';

class FluxNet {
  Future<String?> getPlatformVersion() {
    return FluxNetPlatform.instance.getPlatformVersion();
  }
}
