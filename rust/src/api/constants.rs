pub struct AppConstants;

impl AppConstants {
    pub const CLOUDFLARE_NAME: &'static str = "Cloudflare";
    pub const GOOGLE_NAME: &'static str = "Google";

    pub const DEFAULT_PORT: u16 = 53;
    pub const DEFAULT_TIMEOUT_MS: u64 = 1000;
    pub const DEFAULT_CHECK_INTERVAL_MS: u64 = 5000;

    pub const CLOUDFLARE_DNS: &'static str = "1.1.1.1";
    pub const GOOGLE_DNS: &'static str = "8.8.8.8";

    pub const DEFAULT_EXCELLENT_THRESHOLD: u64 = 50;
    pub const DEFAULT_GREAT_THRESHOLD: u64 = 100;
    pub const DEFAULT_GOOD_THRESHOLD: u64 = 200;
    pub const DEFAULT_MODERATE_THRESHOLD: u64 = 400;
    pub const DEFAULT_POOR_THRESHOLD: u64 = 1000;

    pub const DEFAULT_JITTER_SAMPLES: u8 = 5;
    pub const DEFAULT_JITTER_THRESHOLD_PERCENT: f64 = 0.2;

    pub const CAPTIVE_PORTAL_DETECTION_URL: &'static str = "http://neverssl.com";

    pub const DEFAULT_STABILITY_THRESHOLD: u8 = 70;
    /// 70%

    pub const DEFAULT_CRITICAL_PACKET_LOSS: f32 = 15.0;
    pub const DEFAULT_MAX_TRACEROUTE_HOPS: u8 = 30;
}
