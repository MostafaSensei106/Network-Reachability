#ifndef FLUTTER_PLUGIN_FLUX_NET_PLUGIN_H_
#define FLUTTER_PLUGIN_FLUX_NET_PLUGIN_H_

#include <flutter/method_channel.h>
#include <flutter/plugin_registrar_windows.h>

#include <memory>

namespace flux_net {

class FluxNetPlugin : public flutter::Plugin {
 public:
  static void RegisterWithRegistrar(flutter::PluginRegistrarWindows *registrar);

  FluxNetPlugin();

  virtual ~FluxNetPlugin();

  // Disallow copy and assign.
  FluxNetPlugin(const FluxNetPlugin&) = delete;
  FluxNetPlugin& operator=(const FluxNetPlugin&) = delete;

  // Called when a method is called on this plugin's channel from Dart.
  void HandleMethodCall(
      const flutter::MethodCall<flutter::EncodableValue> &method_call,
      std::unique_ptr<flutter::MethodResult<flutter::EncodableValue>> result);
};

}  // namespace flux_net

#endif  // FLUTTER_PLUGIN_FLUX_NET_PLUGIN_H_
