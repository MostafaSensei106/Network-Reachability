<h1 align="center">Network-Reachability</h1>
<p align="center">
  <img src="https://socialify.git.ci/MostafaSensei106/Network-Reachability/image?font=KoHo&language=1&logo=https%3A%2F%2Favatars.githubusercontent.com%2Fu%2F138288138%3Fv%3D4&name=1&owner=1&pattern=Floating+Cogs&theme=Light" alt="Banner">
</p>

<p align="center">
  <strong>An advanced network monitoring and resilience library for Flutter, powered by a high-performance Rust core.</strong><br>
  Go beyond simple connectivity checks. Understand the <i>quality</i>, <i>stability</i>, and <i>security</i> of your user's network.
</p>

<p align="center">
  <a href="#-why-choose-network-reachability">Why?</a> •
  <a href="#-key-features">Key Features</a> •
  <a href="#-installation">Installation</a> •
  <a href="#-usage">Usage</a> •
  <a href="#-api-overview">API Overview</a> •
  <a href="#-contributing">Contributing</a>
</p>

---

## 🤔 Why Choose Network-Reachability?

Most network libraries tell you if you're `connected` or `disconnected`. In the real world, this is not enough. A user might be "connected" but on a network so slow that your app is unusable, or on a public WiFi that is insecure.

**Network-Reachability answers the questions that truly matter for building robust applications:**

-   **Is the connection good enough?** Instead of a simple boolean, you get a detailed `ConnectionQuality` report (`excellent`, `good`, `poor`, `unstable`), including concrete metrics like **latency**, **jitter**, and **packet loss**. This allows you to tailor the user experience—for example, by disabling video streaming on a `poor` connection.
-   **Is the backend reachable and stable?** This library doesn't just check for a generic internet connection. It probes your actual server endpoints (`NetworkTarget`). If your backend is down, the app will know.
-   **Is the network secure?** For sensitive applications (banking, enterprise), knowing the network environment is critical. This library actively detects security risks like **VPNs**, **DNS hijacking**, and **proxies**, allowing you to block operations on untrusted networks.
-   **How should my app behave during network issues?** With a built-in **Circuit Breaker**, the library can automatically stop your app from hammering a failing backend service, preventing cascading failures and providing a better user experience until the service recovers.

This library gives you the deep network intelligence needed to build resilient, secure, and user-friendly applications that adapt gracefully to real-world network conditions.

---

## ✨ Key Features

This library is built on a high-performance Rust core, providing deep network insights without compromising your app's performance.

<br>

| Feature                           | Description                                                                                                                                                                                                                           |
| --------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Deep Quality Analysis**         | Get a multi-faceted view of the network quality, including **latency**, **jitter** (latency variation), and **packet loss** percentage. The final `ConnectionQuality` enum gives you an instant, actionable summary.               |
| **`guard()` Protected Actions**   | The library's crown jewel. Wrap any network-dependent function (like an API call) in a `guard()`. It will only execute if the network meets your predefined quality and security rules, throwing specific exceptions otherwise.    |
| **Built-in Circuit Breaker**      | Automatically detect when essential backend services are failing. The circuit breaker will "open" and temporarily block further requests, preventing your app from causing server overloads and providing immediate feedback to the user. |
| **Advanced Security Probes**      | Go beyond application-level security. Detect and react to environmental threats: <br> • **VPN & Proxy Detection**: Block or flag connections from anonymized networks. <br> • **DNS Hijack Detection**: Protect against man-in-the-middle attacks. <br> • **Captive Portal Detection**: Identify when the user is stuck on a public WiFi login page. |
| **Granular Configuration**        | Take full control. Customize the `NetworkConfiguration` to: <br> • Define multiple `NetworkTarget` endpoints (TCP/UDP) with priorities. <br> • Set your own `QualityThresholds` for what constitutes an "excellent" or "poor" connection. <br> • Fine-tune the `ResilienceConfig` like the circuit breaker sensitivity. |
| **High-Performance Rust Core**    | All heavy lifting and network probing is executed in a native Rust engine, ensuring that these complex checks are fast, efficient, and don't block the Flutter UI thread.                                                         |

---

## 📦 Installation

1.  Add `flutter_rust_bridge` and this library to your `pubspec.yaml` file:

    ```yaml
    dependencies:
      # This package is not on pub.dev yet.
      # network_reachability: ^1.0.0

    dev_dependencies:
      flutter_rust_bridge: ^2.11.1 # Or the latest version
    ```

2.  Set up `flutter_rust_bridge` by following its [official documentation](https://cjycode.com/flutter_rust_bridge/index.html). This is a crucial step to generate the bindings that link the Rust core to your Flutter app.

3.  Install the dependencies from your terminal:

    ```bash
    flutter pub get
    ```

---

## 🚀 Usage

### 1. Initialization

First, initialize the library in your `main()` function. This sets up the Rust engine and the Dart singleton with your desired configuration.

```dart
import 'package:flutter/material.dart';
import 'package:network_reachability/network_reachability.dart';

// This path will correspond to your generated Rust bindings
import 'package:network_reachability/core/rust/frb_generated.dart';

Future<void> main() async {
  WidgetsFlutterBinding.ensureInitialized();

  // 1. Initialize the Rust library (generated by flutter_rust_bridge)
  await RustLib.init();

  // 2. Initialize Network-Reachability with a default or custom configuration
  await NetworkReachability.init();

  runApp(const MyApp());
}
```

### 2. Performing a Detailed Check

To get a full, on-demand report of the current network status, use the `check()` method.

```dart
Future<void> printNetworkStatus() async {
  final report = await NetworkReachability.instance.check();

  if (report.status.isConnected) {
    print('Network is connected!');
    // Go beyond a simple boolean. Is it actually usable?
    print('Quality: ${report.status.quality.name}'); // e.g., 'good', 'unstable'
    print('Latency: ${report.status.latencyStats.latencyMs}ms');
    print('Jitter: ${report.status.latencyStats.jitterMs}ms');
    print('Packet Loss: ${report.status.latencyStats.packetLossPercent}%');
  } else {
    print('Network is disconnected.');
  }

  // You also get security info
  if(report.securityFlags.isVpnDetected) {
      print('Security Warning: VPN connection detected.');
  }
}
```

### 3. Protecting Network Calls with `guard()`

This is the most powerful way to use the library. The `guard()` method acts as an intelligent gatekeeper for your critical functions. It performs a fresh network check and only executes the `action` if all quality and security rules are met.

```dart
Future<void> fetchSensitiveData() async {
  try {
    // Wrap your API call with the guard.
    final data = await NetworkReachability.instance.guard(
      // The action to perform ONLY if checks pass.
      action: () => myApi.fetchImportantData(),
      // Optional: Require a minimum quality for this specific action.
      minQuality: ConnectionQuality.good,
    );
    print('Data fetched successfully: $data');

  } on PoorConnectionException catch (e) {
    // Thrown if quality is below 'good'.
    // Use this to show a friendly message instead of a generic "Error".
    print('Could not fetch data: The connection is too slow or unstable.');
    print('Details: ${e.message}');

  } on SecurityException catch (e) {
    // Thrown if a security policy is violated (e.g., VPN detected and disallowed).
    print('Action blocked due to a security risk: ${e.message}');

  } on CircuitBreakerOpenException catch (e) {
    // Thrown if the backend is unstable.
    // Prevents the app from making repeated failed requests.
    print('Our servers are temporarily unavailable. Please try again later.');
    print('Details: ${e.message}');
  }
}
```

---

## 🔬 API Overview

Network-Reachability exposes a rich set of models and functions. Here are some of the key components:

### Main Class

-   `NetworkReachability`: The singleton instance to access all library features.
    -   `init()`: Initializes the engine.
    -   `check()`: Performs a one-off, comprehensive network check.
    -   `onStatusChange`: A `Stream<NetworkStatus>` for periodic updates.
    -   `guard()`: Protects a function execution with network validation.
    -   `dispose()`: Cleans up resources.

### Core Data Models

-   `NetworkReport`: The complete output of a `check()`, containing `NetworkStatus`, `ConnectionType`, `SecurityFlags`, and a list of `TargetReport` for each endpoint.
-   `NetworkStatus`: A high-level summary including `isConnected`, `ConnectionQuality`, and `LatencyStats`.
-   `NetworkConfiguration`: The main configuration object to customize targets, quality thresholds, security, and resilience settings.
-   `ConnectionQuality`: An enum representing connection quality from `excellent` to `offline`.
-   `SecurityFlags`: A report on detected security issues like VPNs or DNS spoofing.

### Exceptions

-   `PoorConnectionException`: Thrown by `guard()` if the connection quality is below the required minimum.
-   `SecurityException`: Thrown by `guard()` if a security policy is violated (e.g., VPN is disallowed but detected).
-   `CircuitBreakerOpenException`: Thrown by `guard()` if the circuit breaker is open due to repeated failures of essential targets.

---

## 🤝 Contributing

Contributions are welcome! Please open an issue first to discuss major feature ideas or architectural changes.

1.  Fork the repository.
2.  Create a new branch: `git checkout -b feature/YourFeature`
3.  Commit your changes: `git commit -m "Add amazing feature"`
4.  Push to your branch: `git push origin feature/YourFeature`
5.  Open a pull request.

---

## 📜 License

This project is licensed under the **GPL-3.0 License**. See the `LICENSE` file for full details.

<p align="center">
  Made with ❤️ by <a href="https://github.com/MostafaSensei106">MostafaSensei106</a>
</p>
