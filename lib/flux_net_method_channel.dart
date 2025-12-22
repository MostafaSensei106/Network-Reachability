import 'package:flutter/foundation.dart';
import 'package:flutter/services.dart';

import 'flux_net_platform_interface.dart';

/// An implementation of [FluxNetPlatform] that uses method channels.
class MethodChannelFluxNet extends FluxNetPlatform {
  /// The method channel used to interact with the native platform.
  @visibleForTesting
  final methodChannel = const MethodChannel('flux_net');

  @override
  Future<String?> getPlatformVersion() async {
    final version = await methodChannel.invokeMethod<String>(
      'getPlatformVersion',
    );
    return version;
  }
}
