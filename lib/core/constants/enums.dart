/// Enum representing specific security issues.
///
/// This enum is used by the [guard] function to throw
/// [SecurityException]s when specific security issues are
/// detected.
///
/// The possible values of this enum are:
///
/// * [vpnDetected]: the VPN connection is not allowed.
/// * [dnsHijackDetected]: DNS hijacking was detected.
/// * [proxyDetected]: a proxy server was detected.
/// * [unallowedInterface]: the active interface is not allowed.
///
/// See also [guard].
enum SecurityAlert {
  /// The VPN connection is not allowed.
  vpnDetected,

  /// DNS hijacking was detected.
  dnsHijackDetected,

  /// A proxy server was detected.
  proxyDetected,

  /// The active interface is not allowed.
  unallowedInterface,
}
