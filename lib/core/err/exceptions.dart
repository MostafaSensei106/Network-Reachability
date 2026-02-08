import 'package:network_reachability/core/rust/api/models.dart';

/// Custom exception for poor network quality.
class PoorConnectionException implements Exception {
  final NetworkReport report;
  PoorConnectionException(this.report);

  @override
  String toString() =>
      'PoorConnectionException: '
      'timestampMs: ${report.timestampMs}, '
      'status: ${report.status.quality}, latency ${report.status.latencyMs}ms, '
      'connectionType: ${report.connectionType}, '
      'metadata: ${report.metadata}, '
      'targetReports: ${report.targetReports.map((e) => e.toString()).join(', ')}';
}
