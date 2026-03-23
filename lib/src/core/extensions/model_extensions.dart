import 'package:network_reachability/src/rust/api/models/config.dart';
import 'package:network_reachability/src/rust/api/models/net_info.dart';
import 'package:network_reachability/src/rust/api/models/report.dart';
import 'package:network_reachability/src/rust/api/models/target.dart';

/// Extension on [NetworkConfiguration] to enable easy immutability.
///
/// Use [copyWith] to create a new configuration based on the current one
/// but with specific fields modified.
extension NetworkConfigurationCopyWith on NetworkConfiguration {
  /// Creates a copy of the current configuration with updated fields.
  ///
  /// * [targets]: The list of [NetworkTarget] to monitor.
  /// * [checkIntervalMs]: How often to run periodic checks (0 to disable).
  /// * [cacheValidityMs]: How long to keep results in memory.
  /// * [qualityThreshold]: Custom thresholds for Good/Poor latency.
  /// * [security]: VPN and DNS hijacking security preferences.
  /// * [resilience]: Settings for Circuit Breaker and stability analysis.
  NetworkConfiguration copyWith({
    List<NetworkTarget>? targets,
    BigInt? checkIntervalMs,
    BigInt? cacheValidityMs,
    QualityThresholds? qualityThreshold,
    SecurityConfig? security,
    ResilienceConfig? resilience,
  }) {
    return NetworkConfiguration(
      targets: targets ?? this.targets,
      checkIntervalMs: checkIntervalMs ?? this.checkIntervalMs,
      cacheValidityMs: cacheValidityMs ?? this.cacheValidityMs,
      qualityThreshold: qualityThreshold ?? this.qualityThreshold,
      security: security ?? this.security,
      resilience: resilience ?? this.resilience,
    );
  }
}

/// Extension on [QualityThresholds] for easy updates to latency categorization.
extension QualityThresholdsCopyWith on QualityThresholds {
  /// Returns a new [QualityThresholds] with overridden values.
  ///
  /// * [excellent]: Max latency for 'Excellent' (e.g., < 50ms).
  /// * [great]: Max latency for 'Great' (e.g., < 100ms).
  /// * [good]: Max latency for 'Good' (e.g., < 150ms).
  /// * [moderate]: Max latency for 'Moderate' (e.g., < 250ms).
  /// * [poor]: Max latency for 'Poor' (e.g., < 500ms).
  QualityThresholds copyWith({
    BigInt? excellent,
    BigInt? great,
    BigInt? good,
    BigInt? moderate,
    BigInt? poor,
  }) {
    return QualityThresholds(
      excellent: excellent ?? this.excellent,
      great: great ?? this.great,
      good: good ?? this.good,
      moderate: moderate ?? this.moderate,
      poor: poor ?? this.poor,
    );
  }
}

/// Extension on [ResilienceConfig] to tune circuit breaker and stability scoring.
extension ResilienceConfigCopyWith on ResilienceConfig {
  /// Creates a copy with modified resilience parameters.
  ///
  /// * [strategy]: How multiple targets are evaluated (Consensus vs Race).
  /// * [circuitBreakerThreshold]: Failures before opening the circuit.
  /// * [circuitBreakerCooldownMs]: Wait time before retry probes.
  /// * [numJitterSamples]: Number of probes to use for jitter analysis.
  /// * [jitterThresholdPercent]: Variance threshold for marking as 'Unstable'.
  /// * [stabilityThershold]: Minimum stability score (0-100) required.
  /// * [criticalPacketLossPrecent]: Loss % that triggers an 'Offline' or 'Unstable' status.
  ResilienceConfig copyWith({
    CheckStrategy? strategy,
    int? circuitBreakerThreshold,
    BigInt? circuitBreakerCooldownMs,
    int? numJitterSamples,
    double? jitterThresholdPercent,
    int? stabilityThershold,
    double? criticalPacketLossPrecent,
  }) {
    return ResilienceConfig(
      strategy: strategy ?? this.strategy,
      circuitBreakerThreshold:
          circuitBreakerThreshold ?? this.circuitBreakerThreshold,
      circuitBreakerCooldownMs:
          circuitBreakerCooldownMs ?? this.circuitBreakerCooldownMs,
      numJitterSamples: numJitterSamples ?? this.numJitterSamples,
      jitterThresholdPercent:
          jitterThresholdPercent ?? this.jitterThresholdPercent,
      stabilityThershold: stabilityThershold ?? this.stabilityThershold,
      criticalPacketLossPrecent:
          criticalPacketLossPrecent ?? this.criticalPacketLossPrecent,
    );
  }
}

/// Extension on [SecurityConfig] for toggling network security probes.
extension SecurityConfigCopyWith on SecurityConfig {
  /// Creates a copy with updated security settings.
  ///
  /// * [blockVpn]: If true, VPN interfaces will be flagged.
  /// * [detectDnsHijack]: If true, performs active DNS spoofing checks.
  SecurityConfig copyWith({
    bool? blockVpn,
    bool? detectDnsHijack,
  }) {
    return SecurityConfig(
      blockVpn: blockVpn ?? this.blockVpn,
      detectDnsHijack: detectDnsHijack ?? this.detectDnsHijack,
    );
  }
}

/// Extension on [CaptivePortalStatus] for status immutability.
extension CaptivePortalStatusCopyWith on CaptivePortalStatus {
  /// Returns a modified status of the captive portal probe.
  CaptivePortalStatus copyWith({
    bool? isCaptivePortal,
    String? redirectUrl,
  }) {
    return CaptivePortalStatus(
      isCaptivePortal: isCaptivePortal ?? this.isCaptivePortal,
      redirectUrl: redirectUrl ?? this.redirectUrl,
    );
  }
}

/// Extension on [SecurityFlags] for interface security metadata.
extension SecurityFlagsCopyWith on SecurityFlags {
  /// Returns a copy of [SecurityFlags] with updated values.
  SecurityFlags copyWith({
    bool? isVpnDetected,
    bool? isDnsSpoofed,
    bool? isProxyDetected,
    String? interfaceName,
  }) {
    return SecurityFlags(
      isVpnDetected: isVpnDetected ?? this.isVpnDetected,
      isDnsSpoofed: isDnsSpoofed ?? this.isDnsSpoofed,
      isProxyDetected: isProxyDetected ?? this.isProxyDetected,
      interfaceName: interfaceName ?? this.interfaceName,
    );
  }
}

/// Extension on [SecurityFlagsResult] (Opaque pointer) to allow mutation.
///
/// **Note:** This extension mutates the underlying Rust-managed data directly.
extension SecurityFlagsResultCopyWith on SecurityFlagsResult {
  /// Mutates the opaque result object with new flag values.
  SecurityFlagsResult copyWith({
    bool? isVpnDetected,
    bool? isDnsSpoofed,
    bool? isProxyDetected,
  }) {
    this.isVpnDetected = isVpnDetected ?? this.isVpnDetected;
    this.isDnsSpoofed = isDnsSpoofed ?? this.isDnsSpoofed;
    this.isProxyDetected = isProxyDetected ?? this.isProxyDetected;
    return this;
  }
}

/// Extension on [LatencyStats] for granular metric management.
extension LatencyStatsCopyWith on LatencyStats {
  /// Creates a copy of stats with updated metrics.
  ///
  /// * [latencyMs]: The primary representative latency.
  /// * [jitterMs]: Calculated variance between samples.
  /// * [packetLossPercent]: % of failed probes.
  /// * [stabilityScore]: Overall health score (0-100).
  LatencyStats copyWith({
    BigInt? latencyMs,
    BigInt? jitterMs,
    double? packetLossPercent,
    BigInt? minLatencyMs,
    BigInt? avgLatencyMs,
    BigInt? maxLatencyMs,
    int? stabilityScore,
  }) {
    return LatencyStats(
      latencyMs: latencyMs ?? this.latencyMs,
      jitterMs: jitterMs ?? this.jitterMs,
      packetLossPercent: packetLossPercent ?? this.packetLossPercent,
      minLatencyMs: minLatencyMs ?? this.minLatencyMs,
      avgLatencyMs: avgLatencyMs ?? this.avgLatencyMs,
      maxLatencyMs: maxLatencyMs ?? this.maxLatencyMs,
      stabilityScore: stabilityScore ?? this.stabilityScore,
    );
  }
}

/// Extension on [NetworkReport] (Opaque pointer) to allow mutation.
extension NetworkReportCopyWith on NetworkReport {
  /// Mutates the underlying report object.
  NetworkReport copyWith({
    BigInt? timestampMs,
    NetworkStatus? status,
    ConnectionType? connectionType,
    SecurityFlagsResult? securityFlagsResult,
    List<TargetReport>? targetReports,
  }) {
    this.timestampMs = timestampMs ?? this.timestampMs;
    this.status = status ?? this.status;
    this.connectionType = connectionType ?? this.connectionType;
    this.securityFlagsResult = securityFlagsResult ?? this.securityFlagsResult;
    this.targetReports = targetReports ?? this.targetReports;
    return this;
  }
}

/// Extension on [NetworkStatus] for summary updates.
extension NetworkStatusCopyWith on NetworkStatus {
  /// Returns a copy of the high-level status with modified values.
  NetworkStatus copyWith({
    bool? isConnected,
    ConnectionQuality? quality,
    LatencyStats? latencyStats,
    String? winnerTarget,
  }) {
    return NetworkStatus(
      isConnected: isConnected ?? this.isConnected,
      quality: quality ?? this.quality,
      latencyStats: latencyStats ?? this.latencyStats,
      winnerTarget: winnerTarget ?? this.winnerTarget,
    );
  }
}

/// Extension on [TargetReport] for single target metrics.
extension TargetReportCopyWith on TargetReport {
  /// Returns a copy of the report for a specific target.
  TargetReport copyWith({
    String? label,
    bool? success,
    BigInt? latencyMs,
    String? error,
    bool? isEssential,
  }) {
    return TargetReport(
      label: label ?? this.label,
      success: success ?? this.success,
      latencyMs: latencyMs ?? this.latencyMs,
      error: error ?? this.error,
      isEssential: isEssential ?? this.isEssential,
    );
  }
}

/// Extension on [NetworkTarget] to modify probe endpoint definitions.
extension NetworkTargetCopyWith on NetworkTarget {
  /// Returns a copy of the target configuration.
  ///
  /// * [host]: IP or Domain.
  /// * [protocol]: ICMP, TCP, HTTP, or HTTPS.
  /// * [isEssential]: If true, failure of this target triggers the circuit breaker.
  NetworkTarget copyWith({
    String? label,
    String? host,
    int? port,
    TargetProtocol? protocol,
    BigInt? timeoutMs,
    int? priority,
    bool? isEssential,
  }) {
    return NetworkTarget(
      label: label ?? this.label,
      host: host ?? this.host,
      port: port ?? this.port,
      protocol: protocol ?? this.protocol,
      timeoutMs: timeoutMs ?? this.timeoutMs,
      priority: priority ?? this.priority,
      isEssential: isEssential ?? this.isEssential,
    );
  }
}
