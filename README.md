# Flux-Net (formerly Network Reachability)

A powerful and robust network monitoring and reachability library for Flutter, backed by a high-performance Rust core. Designed for applications that require reliable, detailed, and secure network state awareness, such as banking and enterprise apps.

This library provides a rich set of features including:

- **Comprehensive Network Reports:** Get detailed information about connection quality, latency, jitter, and packet loss.
- **Smart Guard Function:** Protect critical network operations by ensuring connection quality and security policies are met before execution.
- **Circuit Breaker:** Automatically halt requests to unstable backends to prevent cascading failures.
- **Advanced Security Probes:** Detect VPNs, DNS spoofing, and restrictive network interfaces.
- **High-Level Dart API:** Enjoy a clean, easy-to-use Dart interface over a powerful Rust engine.
- **Configurable & Extensible:** Fine-tune the engine's behavior with detailed configuration for targets, quality thresholds, and security policies.

## Getting Started

Follow these steps to integrate Flux-Net into your Flutter project.

### 1. Installation

First, you need to add the library to your `pubspec.yaml`.

_(Note: This library is not yet published on pub.dev. The following is a placeholder for when it is.)_

You will also need to set up `flutter_rust_bridge` according to its documentation to link the Rust core with your Flutter application.

### 2. Initialize the Library

Before using any of the library's features, you must initialize it. This should typically be done in your `main()` function, after ensuring the Flutter bindings are initialized and the Rust library is loaded.

```dart
import 'package:flutter/material.dart';
import 'package:flux_net/network_reachability.dart';
// Import your generated Rust library bindings
import 'package:flux_net/core/rust/frb_generated.dart';

Future<void> main() async {
  WidgetsFlutterBinding.ensureInitialized();

  // 1. Initialize the Rust library
  // This is a prerequisite for Flux-Net
  await RustLib.init();

  // 2. Initialize Flux-Net
  // You can initialize with a default or custom configuration
  await NetworkReachability.init();

  runApp(const MyApp());
}
```

### 3. Basic Usage

Once initialized, you can access the `NetworkReachability` singleton instance anywhere in your app.

#### Performing a One-off Check

To get a detailed, on-demand report of the current network status, use the `check()` method.

```dart
import 'package:flux_net/network_reachability.dart';

Future<void> printNetworkStatus() async {
  final report = await NetworkReachability.instance.check();

  if (report.status.isConnected) {
    print('Network is connected!');
    print('Quality: ${report.status.quality.name}');
    print('Latency: ${report.status.latencyMs}ms');
    print('Jitter: ${report.status.jitterMs}ms');
    print('Packet Loss: ${report.status.packetLossPercent.toStringAsFixed(2)}%');
  } else {
    print('Network is disconnected.');
  }
}
```

#### Listening for Status Changes

Flux-Net can perform periodic checks and notify your app of any status changes through a `Stream`.

```dart
void listenToNetworkChanges() {
  final subscription = NetworkReachability.instance.onStatusChange.listen((status) {
    print('Network status updated: ${status.quality.name}');
  });

  // Don't forget to cancel the subscription when you're done!
  // subscription.cancel();
}
```

#### Protecting Network Calls with `guard()`

The `guard()` method is the library's most powerful feature. It ensures that a network operation only runs if the connection meets your quality and security requirements.

```dart
Future<void> fetchSensitiveData() async {
  try {
    final data = await NetworkReachability.instance.guard(
      // The action to perform if the checks pass
      action: () => myApi.fetchData(),
      // Optional: require a minimum connection quality
      minQuality: ConnectionQuality.good,
    );
    print('Data fetched successfully: $data');
  } on PoorConnectionException catch (e) {
    print('Could not fetch data due to a poor connection: $e');
  } on SecurityException catch (e) {
    print('Action blocked due to a security risk: $e');
  } on CircuitBreakerOpenException catch (e) {
    print('Backend is unstable. Please try again later: $e');
  }
}
```

## Configuration

You can customize the engine's behavior by providing a `NetworkConfiguration` during initialization.

```dart
import 'package:flux_net/core/rust/api/models.dart';

Future<void> initializeWithCustomConfig() async {
  // Fetch the default config to use as a base
  final config = await NetworkConfiguration.default_();

  // Modify the configuration
  final customConfig = config.copyWith(
    // Check every 10 seconds instead of the default 5
    checkIntervalMs: 10000,
    security: SecurityConfig(
      // Block the app from running on a VPN
      blockVpn: true,
      // Enable DNS hijack detection
      detectDnsHijack: true,
      // Only allow the app to run on specific Wi-Fi networks
      allowedInterfaces: ['wlan0', 'wifi_eth'],
    ),
    resilience: config.resilience.copyWith(
      // Open the circuit breaker after 2 consecutive failures
      circuitBreakerThreshold: 2,
    ),
  );

  // Initialize with the custom configuration
  await NetworkReachability.init(config: customConfig);
}
```

This provides a robust foundation for building secure and reliable Flutter applications.
