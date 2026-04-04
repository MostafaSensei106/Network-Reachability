## 0.0.1+4

- 

## 0.0.1+3

- deps: Update Flutter and Rust dependencies

- Flutter Dependencies (`example/pubspec.lock`):
*   **`matcher`**: Updated from `0.12.18` to `0.12.19`.
*   **A local path dependency**: Updated from `0.0.1+1` to `0.0.1+2`.
*   **`test_api`**: Updated from `0.7.9` to `0.7.10`.
These updates primarily involve minor version bumps for testing utilities and a project-specific local package.

- Rust Dependencies (`rust/Cargo.lock`):

-   **Networking & System Interaction:**
    *   **`socket2`**: Updated from `0.6.2` to `0.6.3`. This foundational networking crate's update impacts various downstream dependencies.
    *   **`tokio`**: Updated from `1.49.0` to `1.50.0`, along with `tokio-macros` from `2.6.0` to `2.6.1`. These updates bring the latest improvements and fixes to the asynchronous runtime.
    *   **`windows-sys`**: The underlying `windows-sys` dependency version utilized by several crates (e.g., `socket2`, `quinn-udp`, `tokio`) has been bumped from `0.60.2` to `0.61.2`, incorporating updates to Windows API bindings.
    *   **`schannel`**: Updated from `0.1.28` to `0.1.29`, potentially for Windows security channel improvements.

-   **Cryptographic & Security Libraries:**
    *   **`aws-lc-rs`**: Updated from `1.16.0` to `1.16.2`.
    *   **`aws-lc-sys`**: Updated from `0.37.1` to `0.39.0`. These updates likely include security patches, bug fixes, and performance enhancements for AWS Libcrypto (a fork of BoringSSL) bindings.
    *   **`rustls`**: Updated from `0.23.36` to `0.23.37`.
    *   **`rustls-webpki`**: Updated from `0.103.9` to `0.103.10`. These are minor updates for the Rustls TLS library and its Web PKI component, bringing stability and potential security improvements.

-   **FFI (Foreign Function Interface) & WebAssembly:**
    *   **`jni`**: The `jni` crate updated its direct `jni-sys` dependency to `0.3.1`, which transitively introduced `jni-sys 0.4.1` and `jni-sys-macros 0.4.1`. This indicates improvements or fixes related to Java Native Interface interactions.
    *   **`wasm-bindgen` family**: This includes `wasm-bindgen`, `wasm-bindgen-futures`, `wasm-bindgen-macro`, `wasm-bindgen-macro-support`, `wasm-bindgen-shared`, and `web-sys`. All received significant minor version bumps (e.g., `wasm-bindgen` from `0.2.108` to `0.2.114`, `web-sys` from `0.3.85` to `0.3.91`). These updates likely bring new features, bug fixes, and improved compatibility for WebAssembly integration.

-   **General Utilities & Core Libraries:**
    *   `anyhow`: `1.0.101` to `1.0.102` (error handling).
    *   `bumpalo`: `3.20.1` to `3.20.2` (arena allocation).
    *   `cc`: `1.2.56` to `1.2.57` (C/C++ compiler tools).
    *   `chrono`: `0.4.43` to `0.4.44` (date and time utilities).
    *   `ipnet`: `2.11.0` to `2.12.0` (IP network utilities).
    *   `iri-string`: `0.7.10` to `0.7.11` (IRI parsing).
    *   `itoa`: `1.0.17` to `1.0.18` (integer to string conversion).
    *   `js-sys`: `0.3.85` to `0.3.91` (JavaScript FFI).
    *   `libc`: `0.2.182` to `0.2.183` (C standard library bindings).
    *   `once_cell`: `1.21.3` to `1.21.4` (single assignment cells).
    *   `pin-project-lite`: `0.2.16` to `0.2.17` (pin projection utility).
    *   `quinn-proto`: `0.11.13` to `0.11.14` (QUIC protocol implementation).
    *   `quote`: `1.0.44` to `1.0.45` (procedural macro utility).
    *   `regex-syntax`: `0.8.9` to `0.8.10` (regex parsing).
    *   `syn`: `2.0.116` to `2.0.117` (Rust syntax tree parsing).
    *   `tinyvec`: `1.10.0` to `1.11.0` (small, inlineable vectors).
    *   `zerocopy` & `zerocopy-derive`: Both updated from `0.8.39` to `0.8.47` (safe zero-copy operations).

Overall, this update ensures the project leverages the latest stable versions of its dependencies, incorporating a wide array of improvements across various functional areas without introducing known breaking changes.


## 0.0.1+2

- Structural Overhaul (Clean Architecture).

- Refactor(Core): Complete transition to Clean Architecture. The library is now strictly layered into Data, Domain, and Application layers.

- Decoupling: Isolated the Rust FFI (Native Bridge) from the Dart logic. This allows for better testability and easier implementation of future platform-specific probes.


## 0.0.1+1

- feat(packaging): Add ios and macOS support to support the Swift Package Manager.

- feat(web): Add platform-agnostic network probes and adaptive background checks
  Refactor core network probe logic by introducing platform-specific implementations for WebAssembly (WASM). Implement `NetworkProbe` trait to enable platform-agnostic checks. Introduce adaptive interval for background network checks, dynamically adjusting frequency based on connection quality. Add `NetworkTimeoutException` for clearer error handling. Expose new constructors for configuration models via FFI. Update dependencies and build profiles.

- perf(size): Significantly reduce lib network_reachability shared library size across all architectures - platforms: by ٍ 47.9%.
   
## 0.0.1

- Initial version.
