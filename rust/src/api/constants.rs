pub struct AppConstants;

impl AppConstants {
    pub const CLOUDFLARE_NAME: &'static str = "Cloudflare";
    pub const GOOGLE_NAME: &'static str = "Google";

    pub const DEFAULT_PORT: u16 = 53;
    pub const DEFAULT_TIMEOUT_MS: u32 = 1000;
    pub const DEFAULT_CHECK_INTERVAL_MS: u32 = 5000;

    pub const CLOUDFLARE_DNS: &'static str = "1.1.1.1";
    pub const GOOGLE_DNS: &'static str = "8.8.8.8";

    pub const DEFAULT_EXCELLENT_THRESHOLD: u32 = 50;
    pub const DEFAULT_GREAT_THRESHOLD: u32 = 100;
    pub const DEFAULT_GOOD_THRESHOLD: u32 = 200;
    pub const DEFAULT_MODERATE_THRESHOLD: u32 = 400;
    pub const DEFAULT_POOR_THRESHOLD: u32 = 1000;
}
