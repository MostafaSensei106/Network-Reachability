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
  <a href="#-installation">Installation</a> •
  <a href="#-basic-usage">Basic Usage</a> •
  <a href="#-advanced-usage">Advanced Usage</a> •
  <a href="#-contributing">Contributing</a>
</p>

---

## 🤔 Why Choose Network-Reachability?

> **Stop guessing. Start knowing.** 
> In a world where a "Connected" status is often a lie, your app needs more than a boolean. It needs a pulse.

Most network libraries tell you if you're `connected` or `disconnected`. In the real world, this is simply not enough. A user might be "connected" but on a network so slow it's unusable, behind a login page (Captive Portal), or on an insecure public WiFi exposing your data.

### 📊 How we compare

| Feature | `connectivity_plus` | `internet_connection_checker` | **Network-Reachability** |
| :--- | :---: | :---: | :---: |
| **Connection Type (WiFi/Cellular)** | ✅ | ❌ | ✅ |
| **Actual Internet Verification** | ❌ | ✅ | ✅ |
| **High-Performance Rust Core** | ❌ | ❌ | **🚀 Yes** |
| **Latency, Jitter & Packet Loss** | ❌ | ❌ | **📈 Detailed Stats** |
| **Security (VPN & DNS Hijack)** | ❌ | ❌ | **🛡️ Advanced** |
| **Guarded Actions (`guard()`)** | ❌ | ❌ | **🔒 Built-in** |
| **Circuit Breaker Pattern** | ❌ | ❌ | **🔋 Resilient** |
| **Captive Portal Detection** | ❌ | ✅ | **🌐 Integrated** |

**Network-Reachability answers the questions that truly matter:**

- **Is the connection good enough?** Instead of a simple boolean, you get a detailed `ConnectionQuality` report (`Excellent`, `Great`, `Good`, `Moderate`, `Poor`, `Unstable`, `CaptivePortal`, `Offline`), including concrete metrics like **latency**, **jitter**, and **packet loss**. This allows you to tailor the user experience—for example, by disabling video streaming on a `Poor` connection.
- **Is the backend reachable and stable?** This library doesn't just check for a generic internet connection. It probes your actual server endpoints (`NetworkTarget`). If your backend is down, the app will know.
- **Is the network secure?** For sensitive applications (banking, enterprise), knowing the network environment is critical. This library actively detects security risks like **VPNs**, **DNS hijacking**, and **proxies**, allowing you to block operations on untrusted networks.
- **How should my app behave during network issues?** With a built-in **Circuit Breaker**, the library can automatically stop your app from hammering a failing backend service, preventing cascading failures and providing a better user experience until the service recovers.

This library gives you the deep network intelligence needed to build resilient, secure, and user-friendly applications that adapt gracefully to real-world network conditions.

---

## ✨ Key Features

- **Deep Quality Analysis**: Get a multi-faceted view of the network quality, including **average latency**, **jitter** (latency variation), and **packet loss** percentage. The final `ConnectionQuality` enum gives you an instant, actionable summary.

- **`guard()` Protected Actions**: The library's crown jewel. Wrap any network-dependent function (like an API call) in a `guard()`. It will only execute if the network meets your predefined quality and security rules, throwing specific, catchable exceptions otherwise.

- **Built-in Circuit Breaker**: Automatically detects when essential backend services are failing. The circuit breaker will "open" and temporarily block further requests, preventing your app from causing server overloads and providing immediate feedback to the user.

- **Advanced Security Probes**: Go beyond application-level security. Detect and react to environmental threats:
  - **VPN & Proxy Detection**: Block or flag connections from anonymized networks.
  - **DNS Hijack Detection**: Protect against man-in-the-middle attacks by comparing system DNS against a trusted resolver.
  - **Captive Portal Detection**: Identify when the user is stuck on a public WiFi login page.

- **Granular Configuration**: Take full control. Customize the `NetworkConfiguration` to:
  - Define multiple `NetworkTarget` endpoints (HTTP , HTTPS , TCP , ICMP ) with priorities.
  - Set your own `QualityThresholds` for what constitutes an "excellent" or "poor" connection.
  - Fine-tune the `ResilienceConfig` like the circuit breaker sensitivity and jitter tolerance.

- **High-Performance Rust Core**: All heavy lifting and network probing is executed in a native Rust engine, ensuring that these complex checks are fast, efficient, and don't block the Flutter UI thread.

---

## 📦 Installation

> [!TIP]
> **Don't worry about the "Rust Core"!**
> Adding **Network-Reachability** to your project is designed to be as simple as adding any other Flutter package. While it uses a high-performance Rust engine, you don't need to be a Rust expert or manage complex builds manually. You just install the language once, and the library handles all the heavy lifting, compiling itself automatically for whatever platform (Android, iOS, etc.) or architecture you are targeting.

### 1. Prerequisites (The Rust Toolchain)

Since this library uses **Cargokit** to bridge Flutter and Rust, you need the Rust compiler installed on your development machine.

- **Windows**: Download and run [rustup-init.exe](https://rustup.rs).
- **macOS / Linux**: Run the following command in your terminal:
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

> [!IMPORTANT]
> Once Rust is installed, **Cargokit** will automatically detect your Flutter build target and compile the Rust core into a high-performance native shared library  specifically for that OS and architecture. You only need to set this up once!

### 2. Add the Dependency

Add the package to your `pubspec.yaml`:

```yaml
dependencies:
  network_reachability: ^0.0.1
```

Then, fetch the package:

```bash
flutter pub get
```

### 3. Platform Configuration

> [!IMPORTANT]
> You must add the following permissions to your application to allow it to monitor network status and quality. Without these, the library cannot accurately detect the network type or perform deep probes.

#### **Android**
Add these permissions to your `android/app/src/main/AndroidManifest.xml`:

```xml
<manifest ...>
    <!-- Required to make network requests -->
    <uses-permission android:name="android.permission.INTERNET" />
    <!-- Required to check the type of connection (WiFi, Cellular, etc.) -->
    <uses-permission android:name="android.permission.ACCESS_NETWORK_STATE" />
    ...
</manifest>
```

#### **iOS**
Standard internet checks don't require explicit permissions. 

> [!CAUTION]
> If your configuration probes **local network targets** (e.g., a local server or IoT device), you **must** add the following to your `ios/Runner/Info.plist` to avoid system blocks:

```xml
<key>NSLocalNetworkUsageDescription</key>
<string>This app needs access to the local network to monitor connectivity stability and quality.</string>
```

---

## 🚀 Basic Usage

### 1. Initialization

Initialize the library in your `main()` function. 

> [!IMPORTANT]
> You **must** call `RustLib.init()` before `NetworkReachability.init()`. This ensures the native Rust engine is loaded into memory correctly.

```dart
import 'package:flutter/material.dart';
import 'package:network_reachability/network_reachability.dart';
import 'package:network_reachability/core/rust/frb_generated.dart';

Future<void> main() async {
  WidgetsFlutterBinding.ensureInitialized();
  
  // 1. Initialize the Rust library bindings.
  await RustLib.init();
  
  // 2. Initialize Network-Reachability with a configuration.
  await NetworkReachability.init();
  
  runApp(const MyApp());
}
```

### 2. Protecting Network Calls with `guard()`

> [!TIP]
> Using `guard()` is the best way to handle intermittent connectivity. It automatically checks the network *just before* your action runs, preventing unnecessary server hits when the connection is failing.

```dart
Future<void> fetchSensitiveData() async {
  try {
    // Wrap your API call with the guard.
    final data = await NetworkReachability.instance.guard(
      action: () => myApi.fetchImportantData(),
      minQuality: ConnectionQuality.good,
    );
    print('Data fetched successfully: $data');

  } on PoorConnectionException catch (e) {
    print('Connection too slow: ${e.message}');
  } on SecurityException catch (e) {
    print('Security risk detected: ${e.message}');
  } on CircuitBreakerOpenException catch (e) {
    print('Server cooling down: ${e.message}');
  }
}
```

> [!CAUTION]
> If the `CircuitBreaker` opens, all subsequent `guard()` calls for that target will fail immediately with a `CircuitBreakerOpenException` until the cooldown period expires.

### 3. Monitoring Status Changes

```dart
void listenToNetworkChanges() {
  final subscription = NetworkReachability.instance.onStatusChange.listen((status) {
    print('Quality: ${status.quality.name}');
  });
}
```

---

## 🔬 Advanced Usage

### Custom Configuration

> [!TIP]
> You can create multiple `NetworkTarget` objects to monitor different microservices or fallback endpoints.

```dart
import 'package:network_reachability/network_reachability.dart';

Future<void> initializeWithCustomConfig() async {
  final customConfig = NetworkConfiguration(
    targets: [
      NetworkTarget(
        label: 'main-api',
        host: 'api.example.com',
        isEssential: true, // If this fails, the app considers itself "offline"
      ),
    ],
    // ...
  );

  await NetworkReachability.init(config: customConfig);
}
```

### Direct Probe Access

> [!TIP]
> Direct probes are useful for "pre-flight" checks, like checking for a Captive Portal before showing a login button.

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

## 📜 License

This project is licensed under the **GPL-3.0 License**.
See the [LICENSE](LICENSE) file for full details.

<p align="center">
  Made with ❤️ by <a href="https://github.com/MostafaSensei106">MostafaSensei106</a>
</p>
