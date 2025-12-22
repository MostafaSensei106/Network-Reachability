import 'package:plugin_platform_interface/plugin_platform_interface.dart';

import 'flux_net_method_channel.dart';

abstract class FluxNetPlatform extends PlatformInterface {
  /// Constructs a FluxNetPlatform.
  FluxNetPlatform() : super(token: _token);

  static final Object _token = Object();

  static FluxNetPlatform _instance = MethodChannelFluxNet();

  /// The default instance of [FluxNetPlatform] to use.
  ///
  /// Defaults to [MethodChannelFluxNet].
  static FluxNetPlatform get instance => _instance;

  /// Platform-specific implementations should set this with their own
  /// platform-specific class that extends [FluxNetPlatform] when
  /// they register themselves.
  static set instance(FluxNetPlatform instance) {
    PlatformInterface.verifyToken(instance, _token);
    _instance = instance;
  }

  Future<String?> getPlatformVersion() {
    throw UnimplementedError('platformVersion() has not been implemented.');
  }
}
