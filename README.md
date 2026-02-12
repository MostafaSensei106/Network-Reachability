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
  <a href="#-how-it-works">How It Works</a> •
  <a href="#-installation">Installation</a> •
  <a href="#-basic-usage">Basic Usage</a> •
  <a href="#-advanced-usage">Advanced Usage</a> •
  <a href="#-contributing">Contributing</a>
</p>

---

## 🤔 Why Choose Network-Reachability?

Most network libraries tell you if you're `connected` or `disconnected`. In the real world, this is not enough. A user might be "connected" but on a network so slow that your app is unusable, or on an insecure public WiFi that exposes them to risk.

**Network-Reachability answers the questions that truly matter for building robust applications:**

-   **Is the connection good enough?** Instead of a simple boolean, you get a detailed `ConnectionQuality` report (`excellent`, `good`, `poor`, `unstable`), including concrete metrics like **latency**, **jitter**, and **packet loss**. This allows you to tailor the user experience—for example, by disabling video streaming on a `poor` connection.
-   **Is the backend reachable and stable?** This library doesn't just check for a generic internet connection. It probes your actual server endpoints (`NetworkTarget`). If your backend is down, the app will know.
-   **Is the network secure?** For sensitive applications (banking, enterprise), knowing the network environment is critical. This library actively detects security risks like **VPNs**, **DNS hijacking**, and **proxies**, allowing you to block operations on untrusted networks.
-   **How should my app behave during network issues?** With a built-in **Circuit Breaker**, the library can automatically stop your app from hammering a failing backend service, preventing cascading failures and providing a better user experience until the service recovers.

This library gives you the deep network intelligence needed to build resilient, secure, and user-friendly applications that adapt gracefully to real-world network conditions.

---

## ✨ Key Features

-   **Deep Quality Analysis**: Get a multi-faceted view of the network quality, including **average latency**, **jitter** (latency variation), and **packet loss** percentage. The final `ConnectionQuality` enum gives you an instant, actionable summary.

-   **`guard()` Protected Actions**: The library's crown jewel. Wrap any network-dependent function (like an API call) in a `guard()`. It will only execute if the network meets your predefined quality and security rules, throwing specific, catchable exceptions otherwise.

-   **Built-in Circuit Breaker**: Automatically detects when essential backend services are failing. The circuit breaker will "open" and temporarily block further requests, preventing your app from causing server overloads and providing immediate feedback to the user.

-   **Advanced Security Probes**: Go beyond application-level security. Detect and react to environmental threats:
    -   **VPN & Proxy Detection**: Block or flag connections from anonymized networks.
    -   **DNS Hijack Detection**: Protect against man-in-the-middle attacks by comparing system DNS against a trusted resolver.
    -   **Captive Portal Detection**: Identify when the user is stuck on a public WiFi login page.

-   **Granular Configuration**: Take full control. Customize the `NetworkConfiguration` to:
    -   Define multiple `NetworkTarget` endpoints (TCP/UDP) with priorities.
    -   Set your own `QualityThresholds` for what constitutes an "excellent" or "poor" connection.
    -   Fine-tune the `ResilienceConfig` like the circuit breaker sensitivity and jitter tolerance.

-   **High-Performance Rust Core**: All heavy lifting and network probing is executed in a native Rust engine, ensuring that these complex checks are fast, efficient, and don't block the Flutter UI thread.

---

## 🔧 How It Works

Understanding the lifecycle of the library's core functions is key to using it effectively.

### The Anatomy of `check()`

When you call `NetworkReachability.instance.check()`, a multi-stage process is initiated:

1.  **Rust Engine Execution**: The call is delegated to the high-performance Rust core, which performs the following:
    -   **Parallel Probing**: It sends probes to all `NetworkTarget` endpoints defined in your configuration.
    -   **Data Collection**: It gathers multiple latency samples to calculate statistics like min/max/avg latency, jitter (standard deviation), and packet loss.
    -   **Security Analysis**: It inspects network interfaces to detect the `ConnectionType` (e.g., WiFi, Cellular) and security flags (e.g., `isVpnDetected`).
    -   **Quality Evaluation**: The final metrics are compared against your `QualityThresholds` to determine an overall `ConnectionQuality` score.
2.  **Report Generation**: The Rust engine compiles all this data into a comprehensive `NetworkReport`.
3.  **Circuit Breaker Update**: Back in the Dart layer, the `check()` method inspects the `NetworkReport`. If any `isEssential` target failed, it increments a failure counter. If the counter exceeds the `circuitBreakerThreshold` from your configuration, the circuit is "opened". A successful check on an essential target resets the counter and closes the circuit.

### The Lifecycle of `guard()`

The `guard()` method is an intelligent sequence of validations that wrap your action:

1.  **Circuit Breaker Check**: The very first step. Is the circuit currently open? If yes, the method fails immediately by throwing a `CircuitBreakerOpenException`. This prevents any network activity if the backend is known to be unstable.
2.  **Live Network Check**: It calls the full `check()` method described above to get a fresh, up-to-the-moment `NetworkReport`.
3.  **Security Validation**: It compares the `report.securityFlags` against your `SecurityConfig`.
    -   Is `blockVpn` true and a VPN is detected? Throw `SecurityException`.
    -   Is `detectDnsHijack` true and DNS spoofing is found? Throw `SecurityException`.
    -   Are `allowedInterfaces` defined and the current interface isn't one of them? Throw `SecurityException`.
4.  **Quality Validation**: It compares the `report.status.quality` against the `minQuality` parameter you provided to `guard()`. If the current quality is worse than the minimum required (e.g., you require `good` but the connection is `poor`), it throws a `PoorConnectionException`.
5.  **Execute Action**: Only if all the above checks pass does the `action` function you provided get executed. The return value of your function is then passed back as the result of `guard()`.

This robust, multi-step validation process is what makes `guard()` so powerful.

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

## 🚀 Basic Usage

### 1. Initialization

Initialize the library in your `main()` function. This sets up the Rust engine and the Dart singleton.

```dart
import 'package:flutter/material.dart';
import 'package:network_reachability/network_reachability.dart';
import 'package:network_reachability/core/rust/frb_generated.dart';

Future<void> main() async {
  WidgetsFlutterBinding.ensureInitialized();
  // Initialize the Rust library bindings
  await RustLib.init();
  // Initialize Network-Reachability with a default or custom configuration
  await NetworkReachability.init();
  runApp(const MyApp());
}
```

### 2. Protecting Network Calls with `guard()`

This is the primary and most recommended way to use the library. `guard()` ensures your critical functions only run when the network is in a known good state.

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
    print('Could not fetch data: The connection is too slow or unstable. Details: ${e.message}');

  } on SecurityException catch (e) {
    // Thrown if a security policy is violated (e.g., VPN detected).
    print('Action blocked due to a security risk: ${e.message}');

  } on CircuitBreakerOpenException catch (e) {
    // Thrown if the backend is known to be unstable.
    print('Our servers are temporarily unavailable. Please try again later. Details: ${e.message}');
  }
}
```

### 3. Monitoring Status Changes

You can listen to a stream of network status updates for background monitoring or to update your UI in real-time.

```dart
void listenToNetworkChanges() {
  final subscription = NetworkReachability.instance.onStatusChange.listen((status) {
    // Note: The stream provides a lightweight `NetworkStatus` object.
    // For a full report, you would call `check()` inside the listener.
    print('Network status updated: ${status.quality.name}');
    // Update your UI based on the new status
  });

  // Don't forget to cancel the subscription in your widget's dispose() method.
  // subscription.cancel();
}
```

---

## 🔬 Advanced Usage

### Custom Configuration

Tailor the engine's behavior by providing a `NetworkConfiguration` during initialization.

```dart
import 'package:network_reachability/core/rust/api/models/config.dart';

Future<void> initializeWithCustomConfig() async {
  final config = await NetworkConfiguration.default_();

  final customConfig = NetworkConfiguration(
    checkIntervalMs: BigInt.from(15000), // Check every 15 seconds
    security: SecurityConfig(
      blockVpn: true,
      detectDnsHijack: true,
      allowedInterfaces: ['wlan0', 'eth0'], // Only allow WiFi and Ethernet
    ),
    resilience: config.resilience.copyWith(
      circuitBreakerThreshold: 3, // Open circuit after 3 consecutive failures
    ),
    qualityThreshold: config.qualityThreshold.copyWith(
      good: BigInt.from(150), // Consider latency <= 150ms as 'good'
    ),
    // Define custom targets to your own backend
    targets: [
        NetworkTarget(
            label: 'my-backend-primary',
            host: 'api.mydomain.com',
            port: 443,
            protocol: TargetProtocol.tcp,
            timeoutMs: BigInt.from(2000),
            isEssential: true, // This target affects the circuit breaker
            priority: 1,
        ),
    ],
  );

  await NetworkReachability.init(config: customConfig);
}
```

### Direct Probe Access

For specific, one-off checks, you can call the individual probe functions directly without running a full `check()`.

```dart
// Check if the user is behind a WiFi login page
final captiveStatus = await NetworkReachability.instance.checkForCaptivePortal(
  timeoutMs: BigInt.from(5000),
);
if (captiveStatus.isCaptivePortal) {
  print('User may need to log in to the network at ${captiveStatus.redirectUrl}');
}

// Check for DNS tampering
final isHijacked = await NetworkReachability.instance.detectDnsHijacking(
  domain: 'my-api.com',
);
if (isHijacked) {
  print('Warning: Potential DNS hijacking detected!');
}
```

---

## 🤝 Contributing

Contributions are welcome! Please open an issue first to discuss major feature ideas or architectural changes.

1.  Fork the repository.
2.  Create a new branch: `git checkout -b feature/YourFeature`
3.  Commit your changes: `git commit -m "Add amazing feature"`
4.  Push to your branch: `git push origin feature/YourFeature`
5.  Open a pull request.
