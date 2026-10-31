use crate::models::{DeviceIdentification, KnownProduct, ManufacturerProfile};

pub struct DeviceRegistry {
    profiles: Vec<ManufacturerProfile>,
}

impl DeviceRegistry {
    pub fn new() -> Self {
        let json_data = include_str!("../profiles/manufacturers.json");
        let profiles: Vec<ManufacturerProfile> =
            serde_json::from_str(json_data).unwrap_or_else(|e| {
                tracing::error!("Failed to parse manufacturers.json: {}", e);
                Vec::new()
            });
        Self { profiles }
    }

    /// Identify device from USB VID and PID
    pub fn identify(&self, vendor_id: u16, product_id: u16) -> DeviceIdentification {
        for profile in &self.profiles {
            if profile.vendor_ids.contains(&vendor_id) {
                if let Some(product) = profile
                    .known_products
                    .iter()
                    .find(|p| p.product_id == product_id)
                {
                    let detected_mode = Self::classify_mode(&product.supported_modes);
                    return DeviceIdentification {
                        manufacturer: profile.name.clone(),
                        model_name: product.model_name.clone(),
                        is_known_product: true,
                        official_support_portal: Some(profile.recovery_support_portal.clone()),
                        detected_mode,
                        supported_modes: product.supported_modes.clone(),
                        recovery_notes: profile.support_notes.clone(),
                        fastboot_supported: profile.fastboot_supported,
                    };
                }

                // Known manufacturer, unknown model - strictly avoid false claims!
                return DeviceIdentification {
                    manufacturer: profile.name.clone(),
                    model_name: format!("Unknown {} Model (0x{:04x})", profile.name, product_id),
                    is_known_product: false,
                    official_support_portal: Some(profile.recovery_support_portal.clone()),
                    detected_mode: "Standard USB (Unmatched Model PID)".to_string(),
                    supported_modes: vec!["generic_usb".to_string()],
                    recovery_notes: profile.support_notes.clone(),
                    fastboot_supported: profile.fastboot_supported,
                };
            }
        }

        DeviceIdentification {
            manufacturer: format!("Generic Vendor (0x{:04x})", vendor_id),
            model_name: format!("Generic Device (0x{:04x})", product_id),
            is_known_product: false,
            official_support_portal: None,
            detected_mode: "Unknown USB Device".to_string(),
            supported_modes: vec!["generic_usb".to_string()],
            recovery_notes: None,
            fastboot_supported: false,
        }
    }

    pub fn find_by_vid(&self, vid: u16) -> Option<&ManufacturerProfile> {
        self.profiles.iter().find(|p| p.vendor_ids.contains(&vid))
    }

    pub fn find_product(&self, vid: u16, pid: u16) -> Option<(&ManufacturerProfile, &KnownProduct)> {
        for profile in &self.profiles {
            if profile.vendor_ids.contains(&vid) {
                if let Some(prod) = profile.known_products.iter().find(|p| p.product_id == pid) {
                    return Some((profile, prod));
                }
            }
        }
        None
    }

    pub fn list_supported_manufacturers(&self) -> Vec<&ManufacturerProfile> {
        self.profiles.iter().collect()
    }

    pub fn get_known_products_for_manufacturer(&self, mfg_id: &str) -> Vec<KnownProduct> {
        self.profiles
            .iter()
            .find(|p| p.id.eq_ignore_ascii_case(mfg_id) || p.name.eq_ignore_ascii_case(mfg_id))
            .map(|p| p.known_products.clone())
            .unwrap_or_default()
    }

    pub fn detect_modes_from_pid(&self, vid: u16, pid: u16) -> Vec<String> {
        self.find_product(vid, pid)
            .map(|(_, prod)| prod.supported_modes.clone())
            .unwrap_or_default()
    }

    pub fn is_known_vendor(&self, vid: u16) -> bool {
        self.profiles.iter().any(|p| p.vendor_ids.contains(&vid))
    }

    pub fn is_known_product(&self, vid: u16, pid: u16) -> bool {
        self.find_product(vid, pid).is_some()
    }

    pub fn get_recovery_guidance(&self, mfg_id_or_name: &str) -> Option<String> {
        self.profiles
            .iter()
            .find(|p| {
                p.id.eq_ignore_ascii_case(mfg_id_or_name)
                    || p.name.eq_ignore_ascii_case(mfg_id_or_name)
            })
            .and_then(|p| p.support_notes.clone())
    }

    pub fn all_profiles(&self) -> &[ManufacturerProfile] {
        &self.profiles
    }

    fn classify_mode(modes: &[String]) -> String {
        if modes.iter().any(|m| m == "download") {
            "Samsung Odin Download Mode".to_string()
        } else if modes.iter().any(|m| m == "edl") {
            "Qualcomm Emergency Download (EDL 9008)".to_string()
        } else if modes.iter().any(|m| m == "brom") {
            "MediaTek BootROM (BROM) Mode".to_string()
        } else if modes.iter().any(|m| m == "fastboot") {
            "Fastboot Bootloader Mode".to_string()
        } else if modes.iter().any(|m| m == "recovery") {
            "Recovery / Sideload Mode".to_string()
        } else if modes.iter().any(|m| m == "adb") && modes.iter().any(|m| m == "mtp") {
            "MTP + ADB Debugging Enabled".to_string()
        } else if modes.iter().any(|m| m == "adb") {
            "ADB Debugging Interface".to_string()
        } else if modes.iter().any(|m| m == "mtp") {
            "MTP File Transfer Mode".to_string()
        } else if modes.iter().any(|m| m == "ptp") {
            "PTP Camera Transfer Mode".to_string()
        } else {
            "Standard USB Interface".to_string()
        }
    }
}

impl Default for DeviceRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samsung_identification() {
        let registry = DeviceRegistry::new();
        // Samsung VID = 1256 (or 1204 alias), PID = 26720
        let id1 = registry.identify(1204, 26720);
        assert_eq!(id1.manufacturer, "Samsung");
        assert!(id1.is_known_product);
        assert!(id1.is_adb_capable());

        let id2 = registry.identify(1256, 26736); // Odin Download mode
        assert_eq!(id2.manufacturer, "Samsung");
        assert_eq!(id2.detected_mode, "Samsung Odin Download Mode");
    }

    #[test]
    fn test_pixel_fastboot_identification() {
        let registry = DeviceRegistry::new();
        let id = registry.identify(6353, 53249);
        assert_eq!(id.manufacturer, "Google Pixel");
        assert!(id.is_known_product);
        assert_eq!(id.detected_mode, "Fastboot Bootloader Mode");
        assert!(id.is_fastboot_capable());
    }

    #[test]
    fn test_xiaomi_and_subbrands() {
        let registry = DeviceRegistry::new();
        let id_mi = registry.identify(10007, 65296);
        assert_eq!(id_mi.manufacturer, "Xiaomi");
        assert!(id_mi.is_known_product);
        assert_eq!(id_mi.detected_mode, "Fastboot Bootloader Mode");

        assert!(registry.is_known_vendor(10007));
        assert!(registry.is_known_vendor(10991));
    }

    #[test]
    fn test_oppo_vivo_realme() {
        let registry = DeviceRegistry::new();
        let id_oppo = registry.identify(8907, 10085);
        assert_eq!(id_oppo.manufacturer, "OPPO");
        assert!(id_oppo.is_known_product);

        let id_vivo = registry.identify(11222, 24578);
        assert_eq!(id_vivo.manufacturer, "Vivo");
        assert!(id_vivo.is_known_product);

        let id_realme = registry.identify(8907, 10089);
        assert!(id_realme.is_known_product);
    }

    #[test]
    fn test_motorola_oneplus_nokia() {
        let registry = DeviceRegistry::new();
        let id_moto = registry.identify(8888, 11905);
        assert_eq!(id_moto.manufacturer, "Motorola");
        assert!(id_moto.is_known_product);
        assert_eq!(id_moto.detected_mode, "Fastboot Bootloader Mode");

        let id_oneplus = registry.identify(10864, 61443);
        assert_eq!(id_oneplus.manufacturer, "OnePlus");
        assert!(id_oneplus.is_known_product);

        let id_nokia = registry.identify(11780, 49189);
        assert_eq!(id_nokia.manufacturer, "Nokia");
        assert!(id_nokia.is_known_product);
        assert_eq!(id_nokia.detected_mode, "Fastboot Bootloader Mode");
    }

    #[test]
    fn test_chipset_diagnostics() {
        let registry = DeviceRegistry::new();
        let qcom = registry.identify(1478, 36872);
        assert_eq!(qcom.manufacturer, "Qualcomm Diagnostic");
        assert_eq!(qcom.detected_mode, "Qualcomm Emergency Download (EDL 9008)");

        let mtk = registry.identify(3725, 3);
        assert_eq!(mtk.manufacturer, "MediaTek Diagnostic");
        assert_eq!(mtk.detected_mode, "MediaTek BootROM (BROM) Mode");
    }

    #[test]
    fn test_known_vendor_unknown_product_graceful_handling() {
        let registry = DeviceRegistry::new();
        let id = registry.identify(6353, 0x9999);
        assert_eq!(id.manufacturer, "Google Pixel");
        assert!(!id.is_known_product);
        assert!(id.model_name.contains("Unknown Google Pixel Model (0x9999)"));
        assert!(id.official_support_portal.is_some());
    }

    #[test]
    fn test_unknown_device_graceful_handling() {
        let registry = DeviceRegistry::new();
        let id = registry.identify(0x9999, 0x1234);
        assert!(!id.is_known_product);
        assert!(id.manufacturer.contains("Generic Vendor (0x9999)"));
        assert!(id.model_name.contains("Generic Device (0x1234)"));
        assert!(id.official_support_portal.is_none());
    }

    #[test]
    fn test_query_apis() {
        let registry = DeviceRegistry::new();
        assert!(registry.list_supported_manufacturers().len() >= 11);

        let mfg = registry.find_by_vid(6353).expect("Pixel profile not found");
        assert_eq!(mfg.id, "google");

        let (p_mfg, p_prod) = registry.find_product(6353, 53249).expect("Product not found");
        assert_eq!(p_mfg.name, "Google Pixel");
        assert_eq!(p_prod.model_name, "Pixel Fastboot Bootloader Mode");

        let modes = registry.detect_modes_from_pid(6353, 53249);
        assert_eq!(modes, vec!["fastboot"]);

        let guidance = registry.get_recovery_guidance("google");
        assert!(guidance.is_some());
    }
}
