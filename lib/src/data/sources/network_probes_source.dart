/// Unified network probes data source using conditional exports.
library;

export 'stubs/network_probes_source_stub.dart'
    if (dart.library.io) 'native/native_network_probes_source.dart'
    if (dart.library.html) 'web/web_network_probes_source.dart'
    if (dart.library.js_interop) 'web/web_network_probes_source.dart';
