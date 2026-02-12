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
      title: 'Network Reachability Demo',
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
  // State for core report
  NetworkReport? _report;
  StreamSubscription? _statusSubscription;
  bool _isLoading = true;
  String _guardResult = 'Press "Guarded Action" to test.';

  // State for probe tools
  bool _isProbeLoading = false;
  String? _probeError;
  dynamic _probeResult;

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
          await Future.delayed(
            const Duration(seconds: 1),
          ); // Simulate network call
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

  Future<void> _runProbe(Future<dynamic> Function() probeFn) async {
    if (_isProbeLoading) return;
    setState(() {
      _isProbeLoading = true;
      _probeResult = null;
      _probeError = null;
    });
    try {
      final result = await probeFn();
      setState(() {
        _probeResult = result;
      });
    } catch (e) {
      setState(() {
        _probeError = e.toString();
      });
    } finally {
      if (mounted) {
        setState(() {
          _isProbeLoading = false;
        });
      }
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Network Reachability Demo'),
        actions: [
          if (_isLoading || _isProbeLoading)
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
              _ProbesCard(
                isLoading: _isProbeLoading,
                result: _probeResult,
                error: _probeError,
                onCheckCaptivePortal: () => _runProbe(
                  () => NetworkReachability.instance.checkForCaptivePortal(
                    timeoutMs: BigInt.from(8000),
                  ),
                ),
                onDetectDns: () => _runProbe(
                  () => NetworkReachability.instance.detectDnsHijacking(
                    domain: 'google.com',
                  ),
                ),
                onDetectSecurity: () => _runProbe(
                  () => NetworkReachability.instance
                      .detectSecurityAndNetworkType(),
                ),
                onCheckTarget: () => _runProbe(() {
                  final target = NetworkTarget(
                    label: 'google-dns-single',
                    host: '8.8.8.8',
                    port: 53,
                    protocol: TargetProtocol.udp,
                    timeoutMs: BigInt.from(2000),
                    priority: 1,
                    isEssential: false,
                  );
                  return NetworkReachability.instance.checkTarget(
                    target: target,
                  );
                }),
                // onTraceRoute: () => _runProbe(
                //   () => NetworkReachability.instance.traceRoute(
                //     host: 'google.com',
                //     maxHops: 20,
                //     timeoutPerHopMs: BigInt.from(1000),
                //   ),
                // ),
              ),
              const SizedBox(height: 12),
              _TargetReportsCard(reports: _report!.targetReports),
            ] else if (_isLoading)
              const Center(child: Text("Performing initial network check..."))
            else
              const Center(
                child: Text("Failed to get initial network report."),
              ),
            const SizedBox(height: 80),
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
              title: const Text('Stability Score (0-100)'),
              trailing: Text('${status.latencyStats.stabilityScore}%'),
            ),
            ListTile(
              title: const Text('Avg Latency'),
              trailing: Text('${status.latencyStats.avgLatencyMs ?? 'N/A'} ms'),
            ),
            ListTile(
              title: const Text('Jitter'),
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

class _ProbesCard extends StatelessWidget {
  final bool isLoading;
  final dynamic result;
  final String? error;
  final VoidCallback onCheckCaptivePortal;
  final VoidCallback onDetectDns;
  final VoidCallback onDetectSecurity;
  final VoidCallback onCheckTarget;
  // final VoidCallback onTraceRoute;

  const _ProbesCard({
    required this.isLoading,
    this.result,
    this.error,
    required this.onCheckCaptivePortal,
    required this.onDetectDns,
    required this.onDetectSecurity,
    required this.onCheckTarget,
    // required this.onTraceRoute,
  });

  @override
  Widget build(BuildContext context) {
    final textTheme = Theme.of(context).textTheme;
    return Card(
      elevation: 2,
      child: Padding(
        padding: const EdgeInsets.all(16.0),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Text('Probe Tools', style: textTheme.titleLarge),
            const SizedBox(height: 8),
            if (isLoading) const LinearProgressIndicator(),
            const SizedBox(height: 8),
            Wrap(
              spacing: 8,
              runSpacing: 8,
              children: [
                ElevatedButton(
                  onPressed: isLoading ? null : onCheckCaptivePortal,
                  child: const Text('Captive Portal'),
                ),
                ElevatedButton(
                  onPressed: isLoading ? null : onDetectDns,
                  child: const Text('DNS Hijack'),
                ),
                ElevatedButton(
                  onPressed: isLoading ? null : onDetectSecurity,
                  child: const Text('Security Info'),
                ),
                ElevatedButton(
                  onPressed: isLoading ? null : onCheckTarget,
                  child: const Text('Check Target'),
                ),
                // ElevatedButton(
                //   onPressed: isLoading ? null : onTraceRoute,
                //   child: const Text('Traceroute'),
                // ),
              ],
            ),
            const SizedBox(height: 16),
            if (error != null)
              _buildResultTile(
                'Error',
                Text(error!, style: const TextStyle(color: Colors.red)),
              ),
            if (result != null) _buildResultView(result, context),
          ],
        ),
      ),
    );
  }

  Widget _buildResultView(dynamic res, BuildContext context) {
    if (res is CaptivePortalStatus) {
      return _buildResultTile(
        'Captive Portal',
        Text('Detected: ${res.isCaptivePortal}\nURL: ${res.redirectUrl}'),
      );
    }
    if (res is bool) {
      return _buildResultTile('DNS Hijack', Text('Detected: $res'));
    }
    if (res is (SecurityFlags, ConnectionType)) {
      return Column(
        children: [
          _buildResultTile('Connection Type', Text(res.$2.name)),
          _buildResultTile('Interface Name', Text(res.$1.interfaceName)),
          _buildResultTile('VPN Detected', Text('${res.$1.isVpnDetected}')),
        ],
      );
    }

    if (res is TargetReport) {
      return _buildResultTile(
        'Target Report (${res.label})',
        Text(
          'Success: ${res.success}\nLatency: ${res.latencyMs} ms\nError: ${res.error ?? 'None'}',
        ),
      );
    }
    if (res is List<TraceHop>) {
      return _buildResultTile(
        'Traceroute',
        SizedBox(
          height: 150,
          child: ListView(
            shrinkWrap: true,
            children: res
                .map(
                  (h) => Text(
                    '${h.hopNumber}. ${h.ipAddress} (${h.hostname ?? '...'}) - ${h.latencyMs} ms',
                  ),
                )
                .toList(),
          ),
        ),
      );
    }
    return const SizedBox.shrink();
  }

  Widget _buildResultTile(String title, Widget trailing) {
    return ListTile(title: Text(title), subtitle: trailing, dense: true);
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
