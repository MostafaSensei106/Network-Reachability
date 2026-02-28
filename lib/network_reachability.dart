/// A comprehensive networking library for Dart and Flutter, built on a powerful Rust core.
///
/// This library provides advanced network monitoring, reachability checks, and security features
/// following Clean Architecture principles.
library;

// --- Application Layer ---
export 'src/application/network_reachability_service.dart';

// --- Domain Layer ---
export 'src/domain/entities/entities.dart';

// --- Core ---
export 'src/core/constants/enums.dart';
export 'src/core/exceptions/exceptions.dart';
export 'src/core/extensions/model_extensions.dart';

// --- Rust Generated (Internal use usually, but exported for flexibility) ---
export 'rust/frb_generated.dart';
