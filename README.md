<h1 align="center">Network-Reachability</h1>
<p align="center">
  <img src="https://socialify.git.ci/MostafaSensei106/Network-Reachability/image?custom_language=Rust&font=KoHo&language=1&logo=https%3A%2F%2Favatars.githubusercontent.com%2Fu%2F138288138%3Fv%3D4&name=1&owner=1&pattern=Floating+Cogs&theme=Light" alt="Banner">
</p>

<p align="center">
  <strong>An advanced network monitoring and resilience library for Flutter, powered by a high-performance Rust core.</strong><br>
  Go beyond simple connectivity checks. Understand the <i>quality</i>, <i>stability</i>, and <i>security</i> of your user's network.
</p>

<p align="center">
  <a href="#-why-choose-network-reachability">Why?</a> •
  <a href="#-key-features">Key Features</a> •
  <a href="#-workload-presets">Workload Presets</a> •
  <a href="#-connectivity_plus-drop-in-replacement">connectivity_plus Migration</a> •
  <a href="#-installation">Installation</a> •
  <a href="#-basic-usage">Basic Usage</a> •
  <a href="#-observer-pattern--shared-state">Observer Pattern</a> •
  <a href="#-advanced-usage">Advanced Usage</a> •
  <a href="#-webassembly--web-support">Web Support</a> •
  <a href="#-architecture">Architecture</a> •
  <a href="#-donations--support">Support & Donate</a> •
  <a href="#-contributing">Contributing</a>
</p>

---

## 🤔 Why Choose Network-Reachability?

> **Stop guessing. Start knowing.** 
> In a world where a "Connected" status is often a lie, your app needs more than a boolean. It needs a pulse.

Most network libraries tell you if you're `connected` or `disconnected`. In the real world, this is simply not enough. A user might be "connected" but on a network so slow it's unusable, behind a login page (Captive Portal), or on an insecure public WiFi exposing your data.

### 📊 How we compare

| Feature | `connectivity_plus` | `internet_checker` | **Network-Reachability** |
| :--- | :---: | :---: | :--- |
| **Connection Type** | ✅ | ❌ | **✅ WiFi, Cellular, Ethernet, VPN, Bluetooth** |
| **Internet Verification** | ❌ | ✅ | **✅ Deep Multi-Target Probing (HTTP/TCP/DNS)** |
| **Performance Engine** | Dart/Native | Dart | **🚀 Multi-Threaded Rust Native Engine** |
| **UI Responsiveness** | ✅ | ⚠️ | **⚡ Zero UI-Thread Jitter / Off-Main-Thread** |
| **Detailed Metrics** | ❌ | ❌ | **📈 Latency, Jitter (StdDev), Packet Loss** |
| **Pre-Tuned Presets** | ❌ | ❌ | **🎮 Gaming, Streaming, VoIP, IoT, Enterprise** |
| **Security Suite** | ❌ | ❌ | **🛡️ VPN, System Proxy & DNS Tamper Detection** |
| **Resilience Logic** | ❌ | ❌ | **🔋 Adaptive Circuit Breaker (Closed/Open/Half-Open)** |
| **Observer Pattern** | ❌ | ❌ | **👁️ `listen()` & `listenGuard()` Reactive Hooks** |
| **Request Coalescing** | ❌ | ❌ | **🤝 Thundering Herd Protection** |
| **Battery Management** | ❌ | ❌ | **🔋 Adaptive Interval & Lifecycle Awareness** |
| **Action Protection** | ❌ | ❌ | **🔒 `guard()` Smart Execution Wrapper** |
| **Cross-Platform** | Mobile/Web/Desktop | Mobile/Desktop | **🌐 Android, iOS, macOS, Linux, Windows, Web (WASM)** |

---

## ✨ Key Features

- **🚀 High-Performance Rust Core**: All heavy lifting—multi-target probing, DNS validation, and statistical analysis—executes inside a compiled native Rust library with zero impact on Flutter frame rates.
- **🎮 Workload Presets**: Pre-configured profiles fine-tuned for Gaming, Video Streaming, VoIP, IoT, and Enterprise applications.
- **🔒 The `guard()` Pattern**: Wrap API calls in a smart shield that validates connection quality before execution and uses cached shared state to eliminate redundant probes.
- **👁️ Observer Pattern & Shared State**: Subscribe to network events with `listenGuard` or read instant synchronous properties (`isConnected`, `currentQuality`, `lastReport`) without polling.
- **🔋 Battery-Aware Intelligence**: Automatically doubles polling intervals when quality is `Excellent` and pauses monitoring when the app is in the background.
- **🤝 Thundering Herd Protection**: Built-in request coalescing guarantees that concurrent requests within the cache window share a single underlying probe.
- **🛡️ Enterprise Security Probes**: Detects Captive Portals (WiFi login pages), VPN configurations, proxy redirection, and DNS spoofing.
- **🔌 `connectivity_plus` API Parity**: 1:1 drop-in replacement facade allowing instantaneous migration.
- **🌐 Universal Web & Native Support**: Works seamlessly across mobile, desktop, and web with pre-bundled WebAssembly (WASM).

---

## 🎮 Workload Presets

Tune the entire reachability engine for your application's exact needs in one line of code:

```dart
// Choose from 6 specialized presets:
final config = await NetworkConfiguration.fromPreset(preset: ConfigPreset.gaming);
await NetworkReachability.init(config: config);
```

| Preset | Target Workloads | Check Interval | Jitter Samples | Strategy | Optimization Focus |
| :--- | :--- | :---: | :---: | :---: | :--- |
| `ConfigPreset.gaming` | Real-time FPS, MOBA, multiplayer | **2s** | **8** | Race | Ultra-low latency, strict jitter sensitivity, fast circuit breaking |
| `ConfigPreset.streaming` | Video / Audio streaming (YouTube, Twitch) | **8s** | **4** | Consensus | High throughput, buffer-friendly, relaxed single-target resilience |
| `ConfigPreset.voip` | Voice & Video calls (Zoom, Discord) | **3s** | **6** | Race | Packet-loss sensitivity, voice jitter stutter prevention |
| `ConfigPreset.iot` | Background sync, telemetry, sensors | **30s** | **3** | Race | Extreme battery & CPU savings, relaxed timeout thresholds |
| `ConfigPreset.enterprise` | ERPs, banking apps, internal tools | **10s** | **5** | Consensus | Multi-target consensus, aggressive backend DDOS protection |
| `ConfigPreset.default_` | Standard social, e-commerce, REST apps | **5s** | **5** | Race | Balanced general-purpose mobile & web profile |

---

## 🔌 `connectivity_plus` Drop-In Replacement

Upgrade from `connectivity_plus` with zero UI code refactoring:

```dart
import 'package:network_reachability/network_reachability.dart';

final connectivity = Connectivity();

// 1. Check active connectivity types
final List<ConnectivityResult> results = await connectivity.checkConnectivity();
if (results.contains(ConnectivityResult.mobile)) {
  print('Connected via Cellular Network');
}

// 2. Listen to connectivity stream
connectivity.onConnectivityChanged.listen((List<ConnectivityResult> results) {
  print('Network status changed: $results');
});
```

---

## 📦 Installation

### 1. Prerequisites (Rust Toolchain)

Since `network_reachability` uses a compiled native Rust engine for desktop/mobile, ensure Rust is installed on your machine:

- **Windows**: Download and run [rustup-init.exe](https://rustup.rs).
- **macOS / Linux**: Run in your terminal:
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

### 2. Add Dependency

Add to your `pubspec.yaml`:

```yaml
dependencies:
  network_reachability: ^0.1.0
```

Then fetch packages:

```bash
flutter pub get
```

### 3. Platform Configuration

#### **Android** (`android/app/src/main/AndroidManifest.xml`)
```xml
<manifest xmlns:android="http://schemas.android.com/apk/res/android">
    <uses-permission android:name="android.permission.INTERNET" />
    <uses-permission android:name="android.permission.ACCESS_NETWORK_STATE" />
</manifest>
```

#### **iOS / macOS** (`ios/Runner/Info.plist`)
If you probe local network endpoints, add:
```xml
<key>NSLocalNetworkUsageDescription</key>
<string>This app needs access to monitor local network connectivity and stability.</string>
```

---

## 🚀 Basic Usage

### 1. Initialization

Initialize the engine once during app startup in `main()`:

```dart
import 'package:flutter/material.dart';
import 'package:network_reachability/network_reachability.dart';

Future<void> main() async {
  WidgetsFlutterBinding.ensureInitialized();
  
  // Initializes Rust native core / WASM and starts background monitoring
  await NetworkReachability.init();

  runApp(const MyApp());
}
```

### 2. Protecting Network Calls with `guard()`

```dart
Future<void> submitOrder() async {
  try {
    final result = await NetworkReachability.instance.guard(
      action: () => orderService.placeOrder(),
      minQuality: ConnectionQuality.good, // Requires good or excellent connection
    );
    print('Order placed successfully: $result');
  } on PoorConnectionException catch (e) {
    print('Blocked: Connection is too slow or packet loss is too high ($e)');
  } on SecurityException catch (e) {
    print('Blocked: Security policy violation (e.g., untrusted network) ($e)');
  } on CircuitBreakerOpenException catch (e) {
    print('Blocked: Backend is currently in cooldown ($e)');
  }
}
```

---

## 👁️ Observer Pattern & Shared State

Avoid calling FFI probes on every action. Use reactive listeners and shared state getters:

### 1. Synchronous Shared State

```dart
// Instant access to cached state (0ms latency, zero FFI overhead):
final bool isOnline = NetworkReachability.instance.isConnected;
final ConnectionQuality quality = NetworkReachability.instance.currentQuality;
final NetworkReport? latestReport = NetworkReachability.instance.lastReport;
```

### 2. Reactive `listenGuard` Observer

Execute handlers automatically whenever network health transitions:

```dart
final subscription = NetworkReachability.instance.listenGuard(
  minQuality: ConnectionQuality.good,
  onHealthy: (status) {
    print('Network is healthy: ${status.quality.name} (Latency: ${status.latencyStats.latencyMs}ms)');
  },
  onDegraded: (status) {
    print('Network degraded below threshold: ${status.quality.name}');
  },
);

// Cancel when no longer needed:
subscription.cancel();
```

---

## 🔬 Advanced Usage

### Custom Targets & Fine-Grained Resilience

```dart
final customConfig = NetworkConfiguration(
  targets: [
    NetworkTarget(
      label: 'production-api',
      host: 'api.mycompany.com',
      port: 443,
      protocol: TargetProtocol.http,
      timeoutMs: BigInt.from(2500),
      isEssential: true, // Triggers circuit breaker on consecutive failures
      priority: 1,
    ),
    NetworkTarget(
      label: 'fallback-dns',
      host: '1.1.1.1',
      port: 53,
      protocol: TargetProtocol.tcp,
      timeoutMs: BigInt.from(1000),
      isEssential: false,
      priority: 2,
    ),
  ],
  checkIntervalMs: BigInt.from(6000),
  cacheValidityMs: BigInt.from(2000),
  qualityThreshold: QualityThresholds(
    excellent: BigInt.from(30),
    great: BigInt.from(70),
    good: BigInt.from(120),
    moderate: BigInt.from(200),
    poor: BigInt.from(500),
  ),
  security: const SecurityConfig(
    blockVpn: false,
    detectDnsHijack: true,
  ),
  resilience: const ResilienceConfig(
    strategy: CheckStrategy.race,
    circuitBreakerThreshold: 3,
    circuitBreakerCooldownMs: BigInt.from(30000),
    numJitterSamples: 6,
    jitterThresholdPercent: 0.15,
    stabilityThreshold: 70,
    criticalPacketLossPercent: 4.0,
  ),
);

await NetworkReachability.init(config: customConfig);
```

### Direct Security Probes

```dart
// Check for captive portals (hotel/airport WiFi login screens)
final portalStatus = await NetworkReachability.instance.checkForCaptivePortal(
  timeoutMs: BigInt.from(3000),
);
if (portalStatus.isCaptivePortal) {
  print('Captive Portal detected at: ${portalStatus.redirectUrl}');
}

// Check for DNS hijacking / ISP tampering
final isTampered = await NetworkReachability.instance.detectDnsHijacking(
  domain: 'api.mycompany.com',
);
```

---

## 🏗️ Architecture

The library follows Clean Architecture principles:

```
┌─────────────────────────────────────────────────────────────┐
│                      FLUTTER UI LAYER                       │
│     Widgets • State Managers • Connectivity Facade          │
└──────────────────────────────┬──────────────────────────────┘
                               │
┌──────────────────────────────▼──────────────────────────────┐
│                  APPLICATION SERVICE LAYER                  │
│   NetworkReachability • guard() • listenGuard() • Caching   │
└──────────────────────────────┬──────────────────────────────┘
                               │
┌──────────────────────────────▼──────────────────────────────┐
│                    DOMAIN ENTITIES LAYER                    │
│   NetworkReport • NetworkStatus • ConfigPreset • Models     │
└──────────────────────────────┬──────────────────────────────┘
                               │
┌──────────────────────────────▼──────────────────────────────┐
│              FLUTTER RUST BRIDGE 2.13 (FFI / WASM)          │
└──────────────────────────────┬──────────────────────────────┘
                               │
┌──────────────────────────────▼──────────────────────────────┐
│                   NATIVE RUST ENGINE                        │
│   Multi-Target Pings • Jitter Stats • Security Probes       │
└─────────────────────────────────────────────────────────────┘
```
---

## 🤝 Contributing

Contributions are welcome! Here’s how to get started:

1.  Fork the repository.
2.  Create a new branch:
    `git checkout -b feature/YourFeature`
3.  Commit your changes:
    `git commit -m "Add amazing feature"`
4.  Push to your branch:
    `git push origin feature/YourFeature`
5.  Open a pull request.

> 💡 Please read our **[Contributing Guidelines](CONTRIBUTING.md)** and open an issue first for major feature ideas or changes.

---

## ⚖️ License

This project is dual-licensed:

1. **Open Source License**: GPL-3.0
   - Free to use, modify, and distribute under GPL terms.
   - Any distributed modified version must also be GPL-3.0.

2. **Commercial License**:
   - Required for using the library in proprietary / closed-source products.
   - Only available from the copyright holder (Mostafa Mahmoud).
   - Contact: [mostafasensei106@gmail.com]

See the [LICENSE](LICENSE) file for full details.

<p align="center">
  Made with ❤️ by <a href="https://github.com/MostafaSensei106">MostafaSensei106</a>
</p>
