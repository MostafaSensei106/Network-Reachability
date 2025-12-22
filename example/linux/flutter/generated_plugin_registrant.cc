//
//  Generated file. Do not edit.
//

// clang-format off

#include "generated_plugin_registrant.h"

#include <flux_net/flux_net_plugin.h>

void fl_register_plugins(FlPluginRegistry* registry) {
  g_autoptr(FlPluginRegistrar) flux_net_registrar =
      fl_plugin_registry_get_registrar_for_plugin(registry, "FluxNetPlugin");
  flux_net_plugin_register_with_registrar(flux_net_registrar);
}
