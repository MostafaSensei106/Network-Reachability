## 0.0.1+1

- feat(packaging): Add ios and macOS support to support the Swift Package Manager
- feat(web): Add platform-agnostic network probes and adaptive background checks
  Refactor core network probe logic by introducing platform-specific implementations for WebAssembly (WASM). Implement `NetworkProbe` trait to enable platform-agnostic checks. Introduce adaptive interval for background network checks, dynamically adjusting frequency based on connection quality. Add `NetworkTimeoutException` for clearer error handling. Expose new constructors for configuration models via FFI. Update dependencies and build profiles.

- perf(size): Significantly reduce libnetwork_reachability shared library size across all architectures - platforms: by ٍ 47.9%
   

## 0.0.1

- Initial version.
