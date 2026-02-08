export 'core/err/exceptions.dart';
export 'core/rust/api/models.dart'
    show
        CaptivePortalStatus,
        CheckStrategy,
        ConnectionQuality,
        ConnectionType,
        NetworkMetadata,
        NetworkReport,
        NetworkStatus,
        NetworkTarget,
        NetwrokConfiguration,
        QualityThresholds,
        TargetProtocol,
        TargetReport,
        TraceHop;

// The functions from engine.dart and utils.dart are intended to be used internally by NetworkReachability class
// or by Flutter Rust Bridge, but not directly exposed as top-level library functions for end-users,
// as per the README's developer flow which centralizes interaction through NetworkReachability class.
// export 'core/rust/api/engine.dart' show checkNetwork;
// export 'core/rust/api/utils.dart' show detectNetworkMetadata, evaluateQuality;
