#include "include/flux_net/flux_net_plugin_c_api.h"

#include <flutter/plugin_registrar_windows.h>

#include "flux_net_plugin.h"

void FluxNetPluginCApiRegisterWithRegistrar(
    FlutterDesktopPluginRegistrarRef registrar) {
  flux_net::FluxNetPlugin::RegisterWithRegistrar(
      flutter::PluginRegistrarManager::GetInstance()
          ->GetRegistrar<flutter::PluginRegistrarWindows>(registrar));
}
