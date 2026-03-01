## 0.0.1+2

- Structural Overhaul (Clean Architecture)

- Refactor(Core): Complete transition to Clean Architecture. The library is now strictly layered into Data, Domain, and Application layers.

- Decoupling: Isolated the Rust FFI (Native Bridge) from the Dart logic. This allows for better testability and easier implementation of future platform-specific probes.


## 0.0.1+1

- feat(packaging): Add ios and macOS support to support the Swift Package Manager.

- feat(web): Add platform-agnostic network probes and adaptive background checks
  Refactor core network probe logic by introducing platform-specific implementations for WebAssembly (WASM). Implement `NetworkProbe` trait to enable platform-agnostic checks. Introduce adaptive interval for background network checks, dynamically adjusting frequency based on connection quality. Add `NetworkTimeoutException` for clearer error handling. Expose new constructors for configuration models via FFI. Update dependencies and build profiles.

- perf(size): Significantly reduce lib network_reachability shared library size across all architectures - platforms: by ٍ 47.9%.
   
## 0.0.1

- Initial version.
