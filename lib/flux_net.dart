import 'flux_net_platform_interface.dart';

export 'package:flux_net/src/rust/api/connectivity.dart';

class FluxNet {
  Future<String?> getPlatformVersion() {
    return FluxNetPlatform.instance.getPlatformVersion();
  }
}
