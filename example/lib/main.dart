import 'dart:async';

import 'package:flutter/material.dart';
import 'package:network_reachability/network_reachability.dart';

// Ensure you have run `flutter_rust_bridge_codegen` to generate this file.

Future<void> main() async {
  // Mandatory setup for Flutter apps.
  WidgetsFlutterBinding.ensureInitialized();
  await RustLib.init();
  final defaultConfig = await NetworkConfiguration.default_();
  await NetworkReachability.init(config: defaultConfig);
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Netwrok Reachability Demo',
      theme: ThemeData(
        colorScheme: ColorScheme.fromSeed(
          seedColor: Colors.deepPurple,
          brightness: Brightness.dark,
        ),
        useMaterial3: true,
      ),
      home: const NetworkDemoPage(),
    );
  }
}

class NetworkDemoPage extends StatefulWidget {
  const NetworkDemoPage({super.key});

  @override
  State<NetworkDemoPage> createState() => _NetworkDemoPageState();
}

class _NetworkDemoPageState extends State<NetworkDemoPage> {
  NetworkReport? _report;
  StreamSubscription? _statusSubscription;
  bool _isLoading = true;
  String _guardResult = 'Press "Guarded Action" to test.';

  @override
  void initState() {
    super.initState();
    _manualCheck(); // Perform an initial check.
    // Subscribe to periodic updates.
    _statusSubscription = NetworkReachability.instance.onStatusChange.listen((
      status,
    ) {
      // The stream only gives a NetworkStatus, so we run a full check
      // to get the complete report when a change is detected.
      _manualCheck();
    });
  }

  @override
  void dispose() {
    // Clean up the subscription to prevent memory leaks.
    _statusSubscription?.cancel();
    super.dispose();
  }

  Future<void> _manualCheck() async {
    if (!mounted) return;
    setState(() {
      _isLoading = true;
    });
    final report = await NetworkReachability.instance.check();
    if (!mounted) return;
    setState(() {
      _report = report;
      _isLoading = false;
    });
  }

  Future<void> _performGuardedAction() async {
    setState(() {
      _guardResult = 'Checking network and performing action...';
    });
    try {
      // The guard will throw an exception if requirements are not met.
      final result = await NetworkReachability.instance.guard(
        action: () async {
          return "Data fetched successfully at ${DateTime.now().toIso8601String()}";
        },
        minQuality: ConnectionQuality.moderate,
      );
      setState(() {
        _guardResult = result;
      });
    } on NetworkReachabilityException catch (e) {
      setState(() {
        _guardResult = 'Action Failed!\n${e.runtimeType}: ${e.message}';
      });
    } catch (e) {
      setState(() {
        _guardResult = 'An unexpected error occurred: $e';
      });
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Netwrok Reachability Demo'),

        actions: [
          if (_isLoading)
            const Padding(
              padding: EdgeInsets.only(right: 16.0),
              child: SizedBox(
                width: 20,
                height: 20,
                child: CircularProgressIndicator(strokeWidth: 2),
              ),
            ),
        ],
      ),
      body: RefreshIndicator(
        onRefresh: _manualCheck,
        child: ListView(
          padding: const EdgeInsets.all(12.0),
          children: [
            if (_report != null) ...[
              _StatusCard(status: _report!.status),
              const SizedBox(height: 12),
              _DetailsCard(
                connectionType: _report!.connectionType,
                securityFlags: _report!.securityFlags,
              ),
              const SizedBox(height: 12),
              _GuardActionCard(
                result: _guardResult,
                onPressed: _performGuardedAction,
              ),
              const SizedBox(height: 12),
              _TargetReportsCard(reports: _report!.targetReports),
            ] else if (_isLoading)
              const Center(child: Text("Performing initial network check..."))
            else
              const Center(
                child: Text("Failed to get initial network report."),
              ),
          ],
        ),
      ),
      floatingActionButton: FloatingActionButton.extended(
        onPressed: _manualCheck,
        label: const Text('Manual Refresh'),
        icon: const Icon(Icons.refresh),
      ),
    );
  }
}

// --- UI Helper Widgets ---

class _StatusCard extends StatelessWidget {
  final NetworkStatus status;
  const _StatusCard({required this.status});

  @override
  Widget build(BuildContext context) {
    final colors = _getQualityColors(status.quality, context);
    return Card(
      elevation: 4,
      child: Padding(
        padding: const EdgeInsets.all(16.0),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Row(
              children: [
                Icon(
                  status.isConnected ? Icons.wifi : Icons.wifi_off,
                  color: status.isConnected ? Colors.greenAccent : Colors.red,
                  size: 28,
                ),
                const SizedBox(width: 12),
                Text(
                  status.isConnected ? 'Connected' : 'Disconnected',
                  style: Theme.of(context).textTheme.headlineSmall,
                ),
              ],
            ),
            const SizedBox(height: 16),
            const Divider(),
            ListTile(
              title: const Text('Quality'),
              trailing: Text(
                status.quality.name,
                style: TextStyle(color: colors.$1, fontWeight: FontWeight.bold),
              ),
            ),
            ListTile(
              title: const Text('Stability Score 0 - 100'),
              trailing: Text('${status.latencyStats.stabilityScore}%'),
            ),
            ListTile(
              title: const Text('Latency'),
              trailing: Text('${status.latencyStats.latencyMs} ms'),
            ),
            ListTile(
              title: const Text('Max Latency'),
              trailing: Text('${status.latencyStats.maxLatencyMs} ms'),
            ),
            ListTile(
              title: const Text('Min Latency'),
              trailing: Text('${status.latencyStats.minLatencyMs} ms'),
            ),
            ListTile(
              title: const Text('Jitter (Std Dev)'),
              trailing: Text('${status.latencyStats.jitterMs} ms'),
            ),
            ListTile(
              title: const Text('Packet Loss'),
              trailing: Text(
                '${status.latencyStats.packetLossPercent.toStringAsFixed(1)}%',
              ),
            ),

            ListTile(
              title: const Text('Winning Target'),
              trailing: Text(
                status.winnerTarget.isEmpty ? 'N/A' : status.winnerTarget,
              ),
            ),
          ],
        ),
      ),
    );
  }

  (Color, Color) _getQualityColors(
    ConnectionQuality quality,
    BuildContext context,
  ) {
    switch (quality) {
      case ConnectionQuality.excellent:
        return (Colors.greenAccent, Colors.green.shade900);
      case ConnectionQuality.great:
        return (Colors.lightGreen, Colors.lightGreen.shade900);
      case ConnectionQuality.good:
        return (Colors.yellow, Colors.yellow.shade900);
      case ConnectionQuality.moderate:
        return (Colors.orange, Colors.orange.shade900);
      case ConnectionQuality.poor:
        return (Colors.red, Colors.red.shade900);
      case ConnectionQuality.unstable:
        return (Colors.purpleAccent, Colors.purple.shade900);
      case ConnectionQuality.offline:
        return (Colors.grey, Colors.grey.shade800);
    }
  }
}

class _DetailsCard extends StatelessWidget {
  final ConnectionType connectionType;
  final SecurityFlags securityFlags;
  const _DetailsCard({
    required this.connectionType,
    required this.securityFlags,
  });

  @override
  Widget build(BuildContext context) {
    return Card(
      elevation: 2,
      child: Padding(
        padding: const EdgeInsets.all(16.0),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Text('Details', style: Theme.of(context).textTheme.titleLarge),
            const Divider(),
            ListTile(
              title: const Text('Connection Type'),
              trailing: Text(connectionType.name),
            ),
            ListTile(
              title: const Text('Interface Name'),
              trailing: Text(securityFlags.interfaceName),
            ),
            ListTile(
              title: const Text('Proxy Detected'),
              trailing: _boolIcon(securityFlags.isProxyDetected),
            ),
            ListTile(
              title: const Text('VPN Detected'),
              trailing: _boolIcon(securityFlags.isVpnDetected),
            ),
            ListTile(
              title: const Text('DNS Spoofed'),
              trailing: _boolIcon(securityFlags.isDnsSpoofed),
            ),
          ],
        ),
      ),
    );
  }

  Widget _boolIcon(bool value) {
    return Icon(
      value ? Icons.verified_user : Icons.gpp_bad,
      color: value ? Colors.red : Colors.green,
    );
  }
}

class _GuardActionCard extends StatelessWidget {
  final String result;
  final VoidCallback onPressed;
  const _GuardActionCard({required this.result, required this.onPressed});

  @override
  Widget build(BuildContext context) {
    return Card(
      elevation: 2,
      child: Padding(
        padding: const EdgeInsets.all(16.0),
        child: Column(
          children: [
            Text('Guard Demo', style: Theme.of(context).textTheme.titleLarge),
            const SizedBox(height: 12),
            Text(
              result,
              textAlign: TextAlign.center,
              style: Theme.of(context).textTheme.bodyMedium,
            ),
            const SizedBox(height: 16),
            ElevatedButton(
              onPressed: onPressed,
              child: const Text('Perform Guarded Action'),
            ),
          ],
        ),
      ),
    );
  }
}

class _TargetReportsCard extends StatelessWidget {
  final List<TargetReport> reports;
  const _TargetReportsCard({required this.reports});

  @override
  Widget build(BuildContext context) {
    return Card(
      elevation: 2,
      child: Padding(
        padding: const EdgeInsets.all(16.0),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Text(
              'Target Reports',
              style: Theme.of(context).textTheme.titleLarge,
            ),
            const Divider(),
            ...reports.map(
              (r) => ListTile(
                leading: Icon(
                  r.success ? Icons.check_circle : Icons.cancel,
                  color: r.success ? Colors.green : Colors.red,
                ),
                title: Text(r.label),
                subtitle: r.success
                    ? Text('Latency: ${r.latencyMs} ms')
                    : Text(r.error ?? 'Unknown error'),
                isThreeLine: !r.success,
              ),
            ),
          ],
        ),
      ),
    );
  }
}
