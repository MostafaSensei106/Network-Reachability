pub struct LibConstants;

impl LibConstants {
    pub const CLOUDFLARE_NAME: &'static str = "Cloudflare";
    pub const GOOGLE_NAME: &'static str = "Google";

    pub const DEFAULT_INTERFACE_NAME: &'static str = "unknown";

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

    pub const DEFAULT_STABILITY_THRESHOLD: u8 = 40;
    /// 45%

    pub const DEFAULT_CRITICAL_PACKET_LOSS_PRECENT: f32 = 5.0;

    pub const DEFAULT_MAX_TRACEROUTE_HOPS: u8 = 30;

    // ── Connection Type Detection Prefixes ──────────────────────────────────

    pub const VPN_PREFIXES: &'static [&'static str] = &[
        "tun",
        "tap",
        "ppp",
        "vpn",
        "pptp",
        "pppoe",
        "tunnel",
        "ipsec",
        "l2tp",
        "wg",
        "wireguard",
        "gre",
        "sit",
        "ipip",
        "vti",
        "utun", // macOS VPN
        "svpn",
        "cvpn",
        "openvpn",
        "zerotier",
        "zt",
        "tailscale",
        "ts",
        "mullvad",
        "proton",
    ];

    pub const WIFI_PREFIXES: &'static [&'static str] = &[
        "wlan", "wlp", "wlx", "wlm", "wifi", "ath", // Atheros
        "ra", "rausb", // Ralink
        "rtw", "rtl",   // Realtek
        "brcmf", // Broadcom
        "mlan", "uap", // Marvell
        "iwl", // Intel
        "mt7", // MediaTek
        "qca", "wcn", // Qualcomm
        "ap0", // Access Point mode
    ];

    pub const ETHERNET_PREFIXES: &'static [&'static str] = &[
        "eth", "en", "em", "enp", "eno", "enx", "ens", "lan", "vlan", "bond", // Bonding
        "team", // Teaming
        "br", "virbr", // Bridge
        "veth",  // Virtual Ethernet (Docker/LXC)
        "docker", "lxcbr", "lxdbr", "vmnet", "vmk",     // VMware
        "vboxnet", // VirtualBox
        "xenbr",   // Xen
        "mlx",     // Mellanox/NVIDIA
        "bnxt",    // Broadcom server
        "i40e", "ice", // Intel server
        "tb",  // Thunderbolt
        "usb", "usbnet", // USB Ethernet
        "ecm", "ncm", "rndis", // USB protocols
    ];

    pub const CELLULAR_PREFIXES: &'static [&'static str] = &[
        "rmnet",
        "wwan",
        "lte",
        "3g",
        "4g",
        "5g",
        "nr",
        "gsm",
        "edge",
        "gprs",
        "ccmni", // MediaTek
        "mbim",
        "qmi", // USB modem protocols
        "cdc", // CDC devices
        "hso", // Option modems
        "qcrmnet",
        "rmnet_data", // Qualcomm
        "seth_lte",   // Samsung
    ];

    pub const LOOPBACK_PREFIXES: &'static [&'static str] = &["lo"];

    pub const BLUETOOTH_PREFIXES: &'static [&'static str] = &["bnep", "bt", "pan"];
}
