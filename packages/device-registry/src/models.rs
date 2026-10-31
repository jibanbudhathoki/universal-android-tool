use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct KnownProduct {
    pub product_id: u16,
    pub model_name: String,
    pub supported_modes: Vec<String>,
    #[serde(default)]
    pub hardware_platform: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ManufacturerProfile {
    pub id: String,
    pub name: String,
    pub vendor_ids: Vec<u16>,
    pub known_products: Vec<KnownProduct>,
    pub recovery_support_portal: String,
    #[serde(default)]
    pub support_notes: Option<String>,
    #[serde(default)]
    pub fastboot_supported: bool,
    #[serde(default)]
    pub edl_or_download_mode: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DeviceIdentification {
    pub manufacturer: String,
    pub model_name: String,
    pub is_known_product: bool,
    pub official_support_portal: Option<String>,
    pub detected_mode: String,
    pub supported_modes: Vec<String>,
    pub recovery_notes: Option<String>,
    pub fastboot_supported: bool,
}

impl DeviceIdentification {
    pub fn is_adb_capable(&self) -> bool {
        self.supported_modes.iter().any(|m| m == "adb")
    }

    pub fn is_fastboot_capable(&self) -> bool {
        self.fastboot_supported || self.supported_modes.iter().any(|m| m == "fastboot")
    }

    pub fn is_recovery_mode(&self) -> bool {
        self.supported_modes.iter().any(|m| m == "recovery")
    }
}
