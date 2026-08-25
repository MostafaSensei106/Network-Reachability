import 'dart:async';
import 'package:flutter/material.dart';
import 'package:network_reachability/network_reachability.dart';

void main() async {
  WidgetsFlutterBinding.ensureInitialized();

  try {
    await NetworkReachability.init();
  } catch (e, st) {
    debugPrint('NetworkReachability.init() error: $e\n$st');
  }

  runApp(
    const MaterialApp(
      debugShowCheckedModeBanner: false,
      home: NetworkEngineHub(),
    ),
  );
}

class NetworkEngineHub extends StatefulWidget {
  const NetworkEngineHub({super.key});

  @override
  State<NetworkEngineHub> createState() => _NetworkEngineHubState();
}

class _NetworkEngineHubState extends State<NetworkEngineHub> {
  NetworkReport? _report;
  CaptivePortalStatus? _cpStatus;
  StreamSubscription? _statusSub;
  bool _isBusy = false;
  String? _errorMessage;

  @override
  void initState() {
    super.initState();
    _initAndFetch();
  }

  Future<void> _initAndFetch() async {
    setState(() => _isBusy = true);
    try {
      try {
        final _ = NetworkReachability.instance;
      } catch (_) {
        await NetworkReachability.init();
      }
      _statusSub?.cancel();
      _statusSub = NetworkReachability.instance.onStatusChange.listen(
        (_) => _fetchReport(),
      );
      await _fetchReport();
    } catch (e) {
      if (mounted) {
        setState(() {
          _errorMessage = e.toString();
          _isBusy = false;
        });
      }
    }
  }

  @override
  void dispose() {
    _statusSub?.cancel();
    super.dispose();
  }

  Future<void> _fetchReport() async {
    setState(() => _isBusy = true);
    try {
      final report = await NetworkReachability.instance.check(
        forceRefresh: true,
      );
      if (mounted) {
        setState(() {
          _report = report;
          _errorMessage = null;
        });
      }
    } catch (e) {
      if (mounted) {
        setState(() {
          _errorMessage = e.toString();
        });
      }
    } finally {
      if (mounted) setState(() => _isBusy = false);
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: const Color(0xFF020617),
      appBar: _buildAppBar(),
      body: _report != null
          ? _buildBody()
          : _errorMessage != null
          ? Center(
              child: Padding(
                padding: const EdgeInsets.all(24.0),
                child: Column(
                  mainAxisAlignment: MainAxisAlignment.center,
                  children: [
                    const Icon(
                      Icons.error_outline,
                      color: Colors.redAccent,
                      size: 48,
                    ),
                    const SizedBox(height: 16),
                    Text(
                      'Initialization or Reachability Issue:\n$_errorMessage',
                      textAlign: TextAlign.center,
                      style: const TextStyle(
                        color: Colors.white70,
                        fontSize: 13,
                      ),
                    ),
                    const SizedBox(height: 24),
                    ElevatedButton.icon(
                      onPressed: _fetchReport,
                      icon: const Icon(Icons.refresh),
                      label: const Text('RETRY'),
                      style: ElevatedButton.styleFrom(
                        backgroundColor: Colors.cyanAccent,
                        foregroundColor: Colors.black,
                      ),
                    ),
                  ],
                ),
              ),
            )
          : const Center(
              child: CircularProgressIndicator(color: Colors.cyanAccent),
            ),
    );
  }

  PreferredSizeWidget _buildAppBar() {
    return AppBar(
      backgroundColor: Colors.transparent,
      scrolledUnderElevation: 0,
      elevation: 0,
      title: const Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Text(
            'NETWORK REACHABILITY',
            style: TextStyle(
              fontSize: 14,
              fontWeight: FontWeight.bold,
              letterSpacing: 1.5,
              color: Colors.cyanAccent,
            ),
          ),
          Text(
            'CORE ENGINE v0.1.0',
            style: TextStyle(fontSize: 10, color: Colors.white38),
          ),
        ],
      ),
      actions: [
        if (_isBusy)
          const Center(
            child: Padding(
              padding: EdgeInsets.all(16.0),
              child: SizedBox(
                width: 16,
                height: 16,
                child: CircularProgressIndicator(
                  strokeWidth: 2,
                  color: Colors.cyanAccent,
                ),
              ),
            ),
          ),
        IconButton(
          icon: const Icon(Icons.refresh, color: Colors.white70),
          onPressed: _fetchReport,
        ),
      ],
    );
  }

  Widget _buildBody() {
    final status = _report!.status;
    return SingleChildScrollView(
      padding: const EdgeInsets.all(16),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          _buildMainStatusCard(status),
          const SizedBox(height: 24),
          _buildSectionHeader('SIMULATION & TOOLS'),
          _buildToolGrid(),
          const SizedBox(height: 24),
          _buildSectionHeader('LATENCY DASHBOARD'),
          _buildLatencyGrid(status.latencyStats),
          const SizedBox(height: 24),
          _buildSectionHeader('TARGET ANALYSIS (GRANULAR)'),
          ..._report!.targetReports.map((t) => _buildTargetDetailedCard(t)),
          const SizedBox(height: 24),
          _buildSectionHeader('SECURITY & SYSTEM INTERFACE'),
          _buildSecurityInfo(),
          const SizedBox(height: 50),
        ],
      ),
    );
  }

  Widget _buildMainStatusCard(NetworkStatus status) {
    final color = _getQualityColor(status.quality);
    return Container(
      padding: const EdgeInsets.all(24),
      decoration: BoxDecoration(
        color: const Color(0xFF1E293B).withAlpha(0x50),
        borderRadius: BorderRadius.circular(24),
        border: Border.all(color: color.withAlpha(0x30), width: 2),
      ),
      child: Column(
        children: [
          Row(
            mainAxisAlignment: MainAxisAlignment.spaceBetween,
            children: [
              Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Text(
                    status.quality.name.toUpperCase(),
                    style: TextStyle(
                      color: color,
                      fontSize: 32,
                      fontWeight: FontWeight.bold,
                    ),
                  ),
                  const SizedBox(height: 4),
                  Row(
                    children: [
                      Icon(
                        status.isConnected ? Icons.check_circle : Icons.error,
                        color: status.isConnected ? Colors.green : Colors.red,
                        size: 14,
                      ),
                      const SizedBox(width: 6),
                      Text(
                        status.isConnected ? 'SYSTEM ONLINE' : 'SYSTEM OFFLINE',
                        style: const TextStyle(
                          color: Colors.white60,
                          fontSize: 12,
                          fontWeight: FontWeight.bold,
                        ),
                      ),
                    ],
                  ),
                ],
              ),
              _buildStabilityRing(status.latencyStats.stabilityScore),
            ],
          ),
          const Divider(height: 40, color: Colors.white10),
          Row(
            mainAxisAlignment: MainAxisAlignment.spaceAround,
            children: [
              _buildMiniInfo(
                'WINNER TARGET',
                status.winnerTarget.isEmpty
                    ? 'NONE'
                    : status.winnerTarget.toUpperCase(),
              ),
              _buildMiniInfo(
                'TYPE',
                _report!.connectionType.name.toUpperCase(),
              ),
              _buildMiniInfo('LATENCY', '${status.latencyStats.latencyMs}ms'),
            ],
          ),
        ],
      ),
    );
  }

  Widget _buildTargetDetailedCard(TargetReport t) {
    final bool isFailed = !t.success;
    return Container(
      margin: const EdgeInsets.only(bottom: 12),
      decoration: BoxDecoration(
        color: const Color(0xFF0F172A),
        borderRadius: BorderRadius.circular(16),
        border: Border.all(
          color: isFailed
              ? Colors.red.withAlpha(0x30)
              : Colors.white.withAlpha(0x05),
        ),
      ),
      child: ExpansionTile(
        leading: Icon(
          isFailed ? Icons.dangerous : Icons.check_circle,
          color: isFailed ? Colors.redAccent : Colors.greenAccent,
        ),
        title: Text(
          t.label,
          style: const TextStyle(
            color: Colors.white,
            fontWeight: FontWeight.bold,
            fontSize: 14,
          ),
        ),
        subtitle: Text(
          isFailed ? 'FAILED PROBE' : 'RESPONSE: ${t.latencyMs}ms',
          style: TextStyle(
            color: isFailed ? Colors.redAccent : Colors.white38,
            fontSize: 11,
          ),
        ),
        trailing: t.isEssential
            ? Container(
                padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 2),
                decoration: BoxDecoration(
                  color: Colors.cyanAccent.withAlpha(0x10),
                  borderRadius: BorderRadius.circular(4),
                ),
                child: const Text(
                  'ESSENTIAL',
                  style: TextStyle(
                    color: Colors.cyanAccent,
                    fontSize: 9,
                    fontWeight: FontWeight.bold,
                  ),
                ),
              )
            : null,
        childrenPadding: const EdgeInsets.all(16),
        expandedCrossAxisAlignment: CrossAxisAlignment.start,
        children: [
          if (isFailed) ...[
            const Text(
              'ERROR LOG:',
              style: TextStyle(
                color: Colors.redAccent,
                fontSize: 10,
                fontWeight: FontWeight.bold,
              ),
            ),
            const SizedBox(height: 4),
            Container(
              width: double.infinity,
              padding: const EdgeInsets.all(12),
              decoration: BoxDecoration(
                color: Colors.black26,
                borderRadius: BorderRadius.circular(8),
              ),
              child: Text(
                t.error ?? 'Unknown connection timeout or DNS failure',
                style: const TextStyle(
                  color: Colors.white70,
                  fontSize: 12,
                  fontFamily: 'monospace',
                ),
              ),
            ),
            const SizedBox(height: 12),
          ],
          _buildRowDetail('Success State', t.success ? 'TRUE' : 'FALSE'),
          _buildRowDetail('Response Time', '${t.latencyMs}ms'),
          _buildRowDetail(
            'Criticality',
            t.isEssential ? 'HIGH (Essential)' : 'LOW (Redundant)',
          ),
          _buildRowDetail('Internal ID', t.hashCode.toString()),
        ],
      ),
    );
  }

  Widget _buildLatencyGrid(LatencyStats s) {
    return GridView.count(
      shrinkWrap: true,
      physics: const NeverScrollableScrollPhysics(),
      crossAxisCount: 2,
      childAspectRatio: 2.2,
      mainAxisSpacing: 12,
      crossAxisSpacing: 12,
      children: [
        _buildStatCard('AVG LATENCY', '${s.latencyMs}ms', Icons.timer),
        _buildStatCard(
          'JITTER (STD DEV)',
          '${s.jitterMs}ms',
          Icons.timelapse_rounded,
        ),
        _buildStatCard(
          'PACKET LOSS',
          '${s.packetLossPercent}%',
          Icons.trending_down,
        ),
        _buildStatCard('P95 PEAK', '${s.maxLatencyMs ?? 0}ms', Icons.speed),
      ],
    );
  }

  Widget _buildSecurityInfo() {
    final f = _report!.securityFlagsResult;
    return Container(
      padding: const EdgeInsets.all(20),
      decoration: BoxDecoration(
        color: const Color(0xFF1E293B).withAlpha(0x30),
        borderRadius: BorderRadius.circular(20),
      ),
      child: Column(
        children: [
          _buildSecurityRow('Interface', f.interfaceName, Icons.lan),
          _buildSecurityRow(
            'VPN Access',
            f.isVpnDetected ? 'DETECTION ACTIVE' : 'NONE',
            Icons.vpn_lock,
            active: f.isVpnDetected,
            alert: true,
          ),
          _buildSecurityRow(
            'DNS Status',
            f.isDnsSpoofed ? 'SPOOFING SUSPECTED' : 'TRUSTED/VERIFIED',
            Icons.security,
            active: f.isDnsSpoofed,
            alert: true,
          ),
          _buildSecurityRow(
            'System Proxy',
            f.isProxyDetected ? 'REDIRECTED' : 'DIRECT CONNECTION',
            Icons.router,
            active: f.isProxyDetected,
          ),
          if (_cpStatus != null)
            _buildSecurityRow(
              'Captive Portal',
              _cpStatus!.isCaptivePortal ? 'AUTH REQUIRED' : 'FREE ACCESS',
              Icons.web,
              active: _cpStatus!.isCaptivePortal,
            ),
        ],
      ),
    );
  }

  // --- Helpers & UI Components ---

  Widget _buildSectionHeader(String title) {
    return Padding(
      padding: const EdgeInsets.only(bottom: 12, left: 4),
      child: Text(
        title,
        style: const TextStyle(
          color: Colors.cyanAccent,
          fontSize: 11,
          fontWeight: FontWeight.bold,
          letterSpacing: 1.5,
        ),
      ),
    );
  }

  Widget _buildStatCard(String label, String value, IconData icon) {
    return Container(
      padding: const EdgeInsets.all(12),
      decoration: BoxDecoration(
        color: const Color(0xFF1E293B).withAlpha(0x30),
        borderRadius: BorderRadius.circular(12),
        border: Border.all(color: Colors.white.withAlpha(0x05)),
      ),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        mainAxisAlignment: MainAxisAlignment.center,
        children: [
          Row(
            children: [
              Icon(icon, size: 10, color: Colors.cyanAccent),
              const SizedBox(width: 4),
              Text(
                label,
                style: const TextStyle(
                  color: Colors.white38,
                  fontSize: 9,
                  fontWeight: FontWeight.bold,
                ),
              ),
            ],
          ),
          const SizedBox(height: 4),
          Text(
            value,
            style: const TextStyle(
              overflow: TextOverflow.fade,
              color: Colors.white,
              fontSize: 18,
              fontWeight: FontWeight.bold,
            ),
          ),
        ],
      ),
    );
  }

  Widget _buildSecurityRow(
    String label,
    String value,
    IconData icon, {
    bool active = false,
    bool alert = false,
  }) {
    return Padding(
      padding: const EdgeInsets.symmetric(vertical: 8),
      child: Row(
        children: [
          Icon(
            icon,
            size: 16,
            color: active && alert ? Colors.redAccent : Colors.white24,
          ),
          const SizedBox(width: 12),
          Text(
            label,
            style: const TextStyle(color: Colors.white70, fontSize: 13),
          ),
          const Spacer(),
          Text(
            value,
            style: TextStyle(
              color: active && alert
                  ? Colors.redAccent
                  : (active ? Colors.orangeAccent : Colors.cyanAccent),
              fontSize: 12,
              fontWeight: FontWeight.bold,
            ),
          ),
        ],
      ),
    );
  }

  Widget _buildRowDetail(String label, String value) {
    return Padding(
      padding: const EdgeInsets.symmetric(vertical: 4),
      child: Row(
        mainAxisAlignment: MainAxisAlignment.spaceBetween,
        children: [
          Text(
            label,
            style: const TextStyle(color: Colors.white38, fontSize: 12),
          ),
          Text(
            value,
            style: const TextStyle(
              color: Colors.white70,
              fontSize: 12,
              fontWeight: FontWeight.w500,
            ),
          ),
        ],
      ),
    );
  }

  Widget _buildStabilityRing(int score) {
    return Stack(
      alignment: Alignment.center,
      children: [
        SizedBox(
          width: 50,
          height: 50,
          child: CircularProgressIndicator(
            value: score / 100,
            backgroundColor: Colors.white10,
            color: Colors.cyanAccent,
            strokeWidth: 4,
          ),
        ),
        Text(
          '$score',
          style: const TextStyle(
            color: Colors.white,
            fontWeight: FontWeight.bold,
            fontSize: 16,
          ),
        ),
      ],
    );
  }

  Widget _buildMiniInfo(String label, String value) {
    return Column(
      children: [
        Text(
          label,
          style: const TextStyle(
            color: Colors.white24,
            fontSize: 9,
            fontWeight: FontWeight.bold,
          ),
        ),
        const SizedBox(height: 2),
        Text(
          value,
          style: const TextStyle(
            color: Colors.white,
            fontSize: 13,
            fontWeight: FontWeight.bold,
          ),
        ),
      ],
    );
  }

  ConfigPreset _activePreset = ConfigPreset.default_;

  Widget _buildToolGrid() {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Wrap(
          spacing: 8,
          runSpacing: 8,
          children: ConfigPreset.values.map((p) {
            final isSelected = p == _activePreset;
            return ChoiceChip(
              label: Text(
                p.name.toUpperCase(),
                style: TextStyle(
                  color: isSelected ? Colors.black : Colors.white70,
                  fontSize: 10,
                  fontWeight: FontWeight.bold,
                ),
              ),
              selected: isSelected,
              selectedColor: Colors.cyanAccent,
              backgroundColor: const Color(0xFF1E293B),
              onSelected: (selected) {
                if (selected) _switchPreset(p);
              },
            );
          }).toList(),
        ),
        const SizedBox(height: 12),
        Wrap(
          spacing: 10,
          runSpacing: 10,
          children: [
            _toolButton('GUARD: ACTION', _runGuardDemo, Icons.shield),
            _toolButton('OBSERVER: LISTEN', _runObserverDemo, Icons.radar),
            _toolButton('PORTAL CHECK', _runPortalCheck, Icons.web),
          ],
        ),
      ],
    );
  }

  Widget _toolButton(String label, VoidCallback action, IconData icon) {
    return InkWell(
      onTap: _isBusy ? null : action,
      child: Container(
        padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 12),
        decoration: BoxDecoration(
          color: Colors.cyanAccent.withAlpha(0x05),
          borderRadius: BorderRadius.circular(12),
          border: Border.all(color: Colors.cyanAccent.withAlpha(0x10)),
        ),
        child: Row(
          mainAxisSize: MainAxisSize.min,
          children: [
            Icon(icon, size: 14, color: Colors.cyanAccent),
            const SizedBox(width: 8),
            Text(
              label,
              style: const TextStyle(
                color: Colors.cyanAccent,
                fontSize: 11,
                fontWeight: FontWeight.bold,
              ),
            ),
          ],
        ),
      ),
    );
  }

  // --- Logic Execution ---

  Future<void> _switchPreset(ConfigPreset preset) async {
    setState(() => _isBusy = true);
    try {
      final config = await NetworkConfiguration.fromPreset(preset: preset);
      await NetworkReachability.init(config: config);
      _statusSub?.cancel();
      _statusSub = NetworkReachability.instance.onStatusChange.listen(
        (_) => _fetchReport(),
      );
      setState(() => _activePreset = preset);
      await _fetchReport();
      _showSnack(
        'Switched to ${preset.name.toUpperCase()} preset',
        Colors.cyan,
      );
    } catch (e) {
      _showSnack(e.toString(), Colors.redAccent);
    } finally {
      setState(() => _isBusy = false);
    }
  }

  void _runObserverDemo() {
    final sub = NetworkReachability.instance.listenGuard(
      minQuality: ConnectionQuality.good,
      onHealthy: (status) {
        _showSnack(
          'Observer: Network is Healthy (${status.quality.name})',
          Colors.green,
        );
      },
      onDegraded: (status) {
        _showSnack(
          'Observer: Network Degraded (${status.quality.name})',
          Colors.orange,
        );
      },
    );
    Future.delayed(const Duration(seconds: 10), () => sub.cancel());
    _showSnack('Observer Pattern active for 10s', Colors.cyan);
  }

  Future<void> _runGuardDemo() async {
    setState(() => _isBusy = true);
    try {
      final res = await NetworkReachability.instance.guard(
        minQuality: ConnectionQuality.good,
        action: () async =>
            "SUCCESS: Critical Logic Executed (Shared State Checked)",
      );
      _showSnack(res, Colors.green);
    } catch (e) {
      _showSnack(e.toString(), Colors.redAccent);
    } finally {
      setState(() => _isBusy = false);
    }
  }

  Future<void> _runPortalCheck() async {
    setState(() => _isBusy = true);
    final status = await NetworkReachability.instance.checkForCaptivePortal(
      timeoutMs: BigInt.from(2000),
    );
    setState(() {
      _cpStatus = status;
      _isBusy = false;
    });
    _showSnack(
      status.isCaptivePortal
          ? "PORTAL DETECTED: Authentication needed"
          : "CLEAN: Direct Internet Access",
      status.isCaptivePortal ? Colors.orange : Colors.blue,
    );
  }

  void _showSnack(String msg, Color color) {
    ScaffoldMessenger.of(context).showSnackBar(
      SnackBar(
        content: Text(
          msg,
          style: const TextStyle(fontSize: 12, fontWeight: FontWeight.bold),
        ),
        backgroundColor: color,
        behavior: SnackBarBehavior.floating,
      ),
    );
  }

  Color _getQualityColor(ConnectionQuality q) {
    switch (q) {
      case ConnectionQuality.excellent:
        return Colors.cyanAccent;
      case ConnectionQuality.great:
        return Colors.greenAccent;
      case ConnectionQuality.good:
        return Colors.yellowAccent;
      case ConnectionQuality.moderate:
        return Colors.orangeAccent;
      case ConnectionQuality.poor:
        return Colors.redAccent;
      default:
        return Colors.white24;
    }
  }
}
