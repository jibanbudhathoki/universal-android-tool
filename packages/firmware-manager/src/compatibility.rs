use crate::catalog::FirmwareMetadata;
use serde::{Deserialize, Serialize};
use uat_shared_types::{DeviceInfo, FirmwareCatalogItem};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CompatibilityResult {
    pub is_compatible: bool,
    pub match_confidence: u8, // 0 - 100
    pub model_match: bool,
    pub region_compatible: bool,
    pub anti_rollback_safe: bool,
    pub notes: Vec<String>,
    pub warnings: Vec<String>,
}

pub struct CompatibilityChecker;

impl CompatibilityChecker {
    /// Basic backward-compatible compatibility verification
    pub fn check(device: &DeviceInfo, firmware: &FirmwareCatalogItem) -> CompatibilityResult {
        Self::check_with_metadata(device, firmware, None, None)
    }

    /// Comprehensive compatibility verification evaluating OEM, model aliases, and Anti-Rollback (ARB) safety
    pub fn check_with_metadata(
        device: &DeviceInfo,
        firmware: &FirmwareCatalogItem,
        metadata: Option<&FirmwareMetadata>,
        current_device_arb: Option<u32>,
    ) -> CompatibilityResult {
        let mut notes = Vec::new();
        let mut warnings = Vec::new();
        let mut confidence = 0u8;

        let dev_mfg = device.manufacturer.to_lowercase();
        let fw_mfg = firmware.manufacturer.to_lowercase();

        // 1. Manufacturer verification
        let mfg_matches = dev_mfg == fw_mfg
            || (dev_mfg.contains("google") && fw_mfg.contains("google"))
            || (dev_mfg.contains("pixel") && fw_mfg.contains("pixel"))
            || (dev_mfg.contains("samsung") && fw_mfg.contains("samsung"))
            || (dev_mfg.contains("xiaomi") && fw_mfg.contains("xiaomi"))
            || (dev_mfg.contains("poco") && (fw_mfg.contains("poco") || fw_mfg.contains("xiaomi")))
            || (dev_mfg.contains("redmi") && (fw_mfg.contains("redmi") || fw_mfg.contains("xiaomi")));

        if mfg_matches {
            confidence += 40;
            notes.push(format!("Manufacturer match verified: {}", device.manufacturer));
        } else {
            warnings.push(format!(
                "CRITICAL MANUFACTURER MISMATCH: Device is '{}', but firmware is designed for '{}'. Flashing blocked.",
                device.manufacturer, firmware.manufacturer
            ));
            return CompatibilityResult {
                is_compatible: false,
                match_confidence: 0,
                model_match: false,
                region_compatible: false,
                anti_rollback_safe: false,
                notes,
                warnings,
            };
        }

        // 2. Model and Alias verification
        let dev_model = device.model.to_lowercase();
        let fw_model = firmware.model.to_lowercase();

        let mut model_matched = dev_model == fw_model || fw_model.contains(&dev_model) || dev_model.contains(&fw_model);

        if !model_matched {
            if let Some(meta) = metadata {
                if meta.model_aliases.iter().any(|alias| {
                    let a = alias.to_lowercase();
                    dev_model.contains(&a) || a.contains(&dev_model)
                }) {
                    model_matched = true;
                    notes.push("Hardware model verified via authorized OEM model code alias".to_string());
                }
            }
        }

        if model_matched {
            confidence += 50;
            notes.push(format!("Model compatibility verified: {}", firmware.model));
        } else {
            warnings.push(format!(
                "MODEL MISMATCH: Target device reports '{}', while firmware specifies '{}'. Verify exact hardware SKU.",
                device.model, firmware.model
            ));
        }

        // 3. Region / CSC compatibility
        let region_compatible = firmware.region.eq_ignore_ascii_case("global")
            || firmware.region.eq_ignore_ascii_case("eux")
            || firmware.region.contains("Global");

        if region_compatible {
            confidence += 10;
            notes.push(format!("Region '{}' is compatible with this variant", firmware.region));
        } else {
            notes.push(format!("Region note: Firmware region is designated as '{}'", firmware.region));
        }

        // 4. Anti-Rollback (ARB) Protection Check
        let mut anti_rollback_safe = true;
        if let (Some(meta), Some(dev_arb)) = (metadata, current_device_arb) {
            if meta.anti_rollback_index < dev_arb {
                anti_rollback_safe = false;
                warnings.push(format!(
                    "ANTI-ROLLBACK (ARB) CONFLICT: Device has hardware ARB index {}, but target firmware has ARB index {}. Flashing older build will hard-brick the device.",
                    dev_arb, meta.anti_rollback_index
                ));
            } else {
                notes.push(format!(
                    "Anti-Rollback check passed (Device ARB: {}, Firmware ARB: {})",
                    dev_arb, meta.anti_rollback_index
                ));
            }
        }

        let is_compatible = confidence >= 90 && model_matched && anti_rollback_safe;

        CompatibilityResult {
            is_compatible,
            match_confidence: confidence,
            model_match: model_matched,
            region_compatible,
            anti_rollback_safe,
            notes,
            warnings,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exact_compatibility_match() {
        let device = DeviceInfo {
            id: "pixel8-test".to_string(),
            vendor_id: 6353,
            product_id: 20193,
            manufacturer: "Google Pixel".to_string(),
            model: "Pixel 8".to_string(),
            android_version: Some("14.0".to_string()),
            build_number: None,
            serial_number: None,
            connection_mode: uat_shared_types::ConnectionMode::NativeUsb,
            supported_protocols: vec![],
            capabilities: Default::default(),
            is_authorized: false,
        };

        let fw = FirmwareCatalogItem {
            id: "fw-pixel-8".to_string(),
            manufacturer: "Google Pixel".to_string(),
            model: "Pixel 8".to_string(),
            build_id: "UP1A.231005.007".to_string(),
            android_version: "14.0".to_string(),
            release_date: "2023-10-04".to_string(),
            region: "Global".to_string(),
            verification_checksum: "sha256:e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855".to_string(),
            download_url: None,
        };

        let res = CompatibilityChecker::check(&device, &fw);
        assert!(res.is_compatible);
        assert!(res.match_confidence >= 90);
        assert!(res.warnings.is_empty());
    }

    #[test]
    fn test_anti_rollback_downgrade_prevention() {
        let device = DeviceInfo {
            id: "s23-test".to_string(),
            vendor_id: 1256,
            product_id: 26720,
            manufacturer: "Samsung".to_string(),
            model: "Galaxy S23 (SM-S911B)".to_string(),
            android_version: Some("14.0".to_string()),
            build_number: None,
            serial_number: None,
            connection_mode: uat_shared_types::ConnectionMode::NativeUsb,
            supported_protocols: vec![],
            capabilities: Default::default(),
            is_authorized: false,
        };

        let fw = FirmwareCatalogItem {
            id: "fw-s23-v1".to_string(),
            manufacturer: "Samsung".to_string(),
            model: "Galaxy S23 (SM-S911B)".to_string(),
            build_id: "S911BXXU1AWA6".to_string(),
            android_version: "13.0".to_string(),
            release_date: "2023-02-01".to_string(),
            region: "EUX".to_string(),
            verification_checksum: "sha256:0123456789".to_string(),
            download_url: None,
        };

        let meta = FirmwareMetadata {
            security_patch_level: "2023-02-01".to_string(),
            anti_rollback_index: 1, // Firmware has ARB 1
            chipset: Some("Snapdragon 8 Gen 2".to_string()),
            model_aliases: vec!["SM-S911B".to_string()],
        };

        // Device currently has bootloader SW REV 3 (ARB 3)
        let res = CompatibilityChecker::check_with_metadata(&device, &fw, Some(&meta), Some(3));
        assert!(!res.is_compatible);
        assert!(!res.anti_rollback_safe);
        assert!(res.warnings.iter().any(|w| w.contains("ANTI-ROLLBACK")));
    }
}
