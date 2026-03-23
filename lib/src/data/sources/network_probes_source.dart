/// Unified entry point for network probes across different platforms.
///
/// This library uses **Conditional Exports** to ensure that the correct
/// implementation of `NetworkProbesSource` is used at compile-time:
///
/// * **Native (io):** Uses `native/native_network_probes_source.dart` which
///   talks to Rust via FFI.
/// * **Web (html/js):** Uses `web/web_network_probes_source.dart` which
///   talks to Rust via WASM.
/// * **Fallback:** Uses `stubs/network_probes_source_stub.dart` to prevent
///   compilation errors on unsupported platforms.
library;

export 'stubs/network_probes_source_stub.dart'
    if (dart.library.io) 'native/native_network_probes_source.dart'
    if (dart.library.html) 'web/web_network_probes_source.dart'
    if (dart.library.js_interop) 'web/web_network_probes_source.dart';
