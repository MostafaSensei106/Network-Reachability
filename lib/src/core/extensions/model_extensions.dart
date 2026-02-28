import 'package:network_reachability/rust/api/models/config.dart';
import 'package:network_reachability/rust/api/models/net_info.dart';
import 'package:network_reachability/rust/api/models/report.dart';
import 'package:network_reachability/rust/api/models/target.dart';

/// Extension on [NetworkConfiguration] to provide an immutable way to copy the config.
extension NetworkConfigurationCopyWith on NetworkConfiguration {
  /// Creates a copy of [NetworkConfiguration] with the given fields replaced by the new values.
  ///
  /// [targets] Optional list of [NetworkTarget] to update.
  /// [checkIntervalMs] Optional interval between checks to update.
  /// [cacheValidityMs] Optional cache duration to update.
  /// [qualityThreshold] Optional [QualityThresholds] to update.
  /// [security] Optional [SecurityConfig] to update.
  /// [resilience] Optional [ResilienceConfig] to update.
  ///
  /// Returns a new [NetworkConfiguration] instance.
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

/// Extension on [QualityThresholds] to provide an immutable way to copy the config.
extension QualityThresholdsCopyWith on QualityThresholds {
  /// Creates a copy of [QualityThresholds] with the given fields replaced by the new values.
  ///
  /// [excellent] Latency threshold for Excellent quality.
  /// [great] Latency threshold for Great quality.
  /// [good] Latency threshold for Good quality.
  /// [moderate] Latency threshold for Moderate quality.
  /// [poor] Latency threshold for Poor quality.
  ///
  /// Returns a new [QualityThresholds] instance.
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

/// Extension on [ResilienceConfig] to provide an immutable way to copy the config.
extension ResilienceConfigCopyWith on ResilienceConfig {
  /// Creates a copy of [ResilienceConfig] with the given fields replaced by the new values.
  ///
  /// [strategy] Evaluation strategy for multiple targets.
  /// [circuitBreakerThreshold] Number of failures before opening the circuit.
  /// [circuitBreakerCooldownMs] Duration to wait before trial checks.
  /// [numJitterSamples] Number of samples for stability analysis.
  /// [jitterThresholdPercent] Threshold for flagging high jitter.
  /// [stabilityThershold] Minimum stability score required.
  /// [criticalPacketLossPrecent] Packet loss threshold for unstable status.
  ///
  /// Returns a new [ResilienceConfig] instance.
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

/// Extension on [SecurityConfig] to provide an immutable way to copy the config.
extension SecurityConfigCopyWith on SecurityConfig {
  /// Creates a copy of [SecurityConfig] with the given fields replaced by the new values.
  ///
  /// [blockVpn] Whether to flag VPN connections.
  /// [detectDnsHijack] Whether to perform DNS spoofing checks.
  ///
  /// Returns a new [SecurityConfig] instance.
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

/// Extension on [CaptivePortalStatus] to provide an immutable way to copy the status.
extension CaptivePortalStatusCopyWith on CaptivePortalStatus {
  /// Creates a copy of [CaptivePortalStatus] with the given fields replaced by the new values.
  ///
  /// [isCaptivePortal] Whether a portal was detected.
  /// [redirectUrl] The URL the user was redirected to.
  ///
  /// Returns a new [CaptivePortalStatus] instance.
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

/// Extension on [SecurityFlags] to provide an immutable way to copy the flags.
extension SecurityFlagsCopyWith on SecurityFlags {
  /// Creates a copy of [SecurityFlags] with the given fields replaced by the new values.
  ///
  /// [isVpnDetected] VPN detection status.
  /// [isDnsSpoofed] DNS spoofing status.
  /// [isProxyDetected] Proxy detection status.
  /// [interfaceName] Name of the network interface.
  ///
  /// Returns a new [SecurityFlags] instance.
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

/// Extension on [SecurityFlagsResult] to provide an immutable way to copy the result.
extension SecurityFlagsResultCopyWith on SecurityFlagsResult {
  /// Creates a copy of [SecurityFlagsResult] by modifying fields on the opaque pointer.
  ///
  /// Note: This performs mutation on the underlying Rust-managed memory.
  ///
  /// [isVpnDetected] VPN detection status.
  /// [isDnsSpoofed] DNS spoofing status.
  /// [isProxyDetected] Proxy detection status.
  ///
  /// Returns the modified [SecurityFlagsResult] instance.
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

/// Extension on [LatencyStats] to provide an immutable way to copy stats.
extension LatencyStatsCopyWith on LatencyStats {
  /// Creates a copy of [LatencyStats] with the given fields replaced by the new values.
  ///
  /// [latencyMs] Representative latency value.
  /// [jitterMs] Measured jitter.
  /// [packetLossPercent] Percentage of packets lost.
  /// [minLatencyMs] Minimum latency sample.
  /// [avgLatencyMs] Average latency sample.
  /// [maxLatencyMs] Maximum latency sample.
  /// [stabilityScore] Calculated stability score (0-100).
  ///
  /// Returns a new [LatencyStats] instance.
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

/// Extension on [NetworkReport] to provide an immutable way to copy the report.
extension NetworkReportCopyWith on NetworkReport {
  /// Creates a copy of [NetworkReport] by modifying fields on the opaque pointer.
  ///
  /// Note: This performs mutation on the underlying Rust-managed memory.
  ///
  /// [timestampMs] Epoch timestamp of the check.
  /// [status] Consolidated network status.
  /// [connectionType] Identified interface type.
  /// [securityFlagsResult] Security probe results.
  /// [targetReports] Individual results for all targets.
  ///
  /// Returns the modified [NetworkReport] instance.
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

/// Extension on [NetworkStatus] to provide an immutable way to copy the status summary.
extension NetworkStatusCopyWith on NetworkStatus {
  /// Creates a copy of [NetworkStatus] with the given fields replaced by the new values.
  ///
  /// [isConnected] Whether connection is active.
  /// [quality] Categorical assessment of quality.
  /// [latencyStats] Detailed performance metrics.
  /// [winnerTarget] Label of the fastest responding target.
  ///
  /// Returns a new [NetworkStatus] instance.
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

/// Extension on [TargetReport] to provide an immutable way to copy the report for a single target.
extension TargetReportCopyWith on TargetReport {
  /// Creates a copy of [TargetReport] with the given fields replaced by the new values.
  ///
  /// [label] Unique identifier for the target.
  /// [success] Whether the target was reached.
  /// [latencyMs] Measured response time.
  /// [error] Error message if failed.
  /// [isEssential] Whether failure triggers circuit breaker.
  ///
  /// Returns a new [TargetReport] instance.
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

/// Extension on [NetworkTarget] to provide an immutable way to copy the target definition.
extension NetworkTargetCopyWith on NetworkTarget {
  /// Creates a copy of [NetworkTarget] with the given fields replaced by the new values.
  ///
  /// [label] Human-readable identifier.
  /// [host] Domain or IP address.
  /// [port] Destination port.
  /// [protocol] Communication protocol (TCP, ICMP, etc.).
  /// [timeoutMs] Connection timeout.
  /// [priority] Relative priority for sorting.
  /// [isEssential] Criticality flag.
  ///
  /// Returns a new [NetworkTarget] instance.
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
