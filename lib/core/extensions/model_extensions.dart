import '../rust/api/models/config.dart';
import '../rust/api/models/net_info.dart';
import '../rust/api/models/report.dart';
import '../rust/api/models/target.dart';

/// Extension on [NetworkConfiguration] to provide an immutable way to copy the config.
extension NetworkConfigurationCopyWith on NetworkConfiguration {
  /// Creates a copy of [NetworkConfiguration] with the given fields replaced by the new values.
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

/// Extension on [TraceHop] to provide an immutable way to copy the hop data.
extension TraceHopCopyWith on TraceHop {
  /// Creates a copy of [TraceHop] with the given fields replaced by the new values.
  TraceHop copyWith({
    int? hopNumber,
    String? ipAddress,
    String? hostname,
    BigInt? latencyMs,
  }) {
    return TraceHop(
      hopNumber: hopNumber ?? this.hopNumber,
      ipAddress: ipAddress ?? this.ipAddress,
      hostname: hostname ?? this.hostname,
      latencyMs: latencyMs ?? this.latencyMs,
    );
  }
}

/// Extension on [LatencyStats] to provide an immutable way to copy stats.
extension LatencyStatsCopyWith on LatencyStats {
  /// Creates a copy of [LatencyStats] with the given fields replaced by the new values.
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
  /// Creates a copy of [NetworkReport] with the given fields replaced by the new values.
  NetworkReport copyWith({
    BigInt? timestampMs,
    NetworkStatus? status,
    ConnectionType? connectionType,
    SecurityFlags? securityFlags,
    List<TargetReport>? targetReports,
  }) {
    return NetworkReport(
      timestampMs: timestampMs ?? this.timestampMs,
      status: status ?? this.status,
      connectionType: connectionType ?? this.connectionType,
      securityFlags: securityFlags ?? this.securityFlags,
      targetReports: targetReports ?? this.targetReports,
    );
  }
}

/// Extension on [NetworkStatus] to provide an immutable way to copy the status summary.
extension NetworkStatusCopyWith on NetworkStatus {
  /// Creates a copy of [NetworkStatus] with the given fields replaced by the new values.
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
