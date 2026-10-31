use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uat_shared_types::{DeviceInfo, FirmwareCatalogItem};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FirmwareMetadata {
    pub security_patch_level: String,
    pub anti_rollback_index: u32,
    pub chipset: Option<String>,
    pub model_aliases: Vec<String>,
}

pub struct FirmwareCatalog {
    items: Vec<FirmwareCatalogItem>,
    metadata_map: HashMap<String, FirmwareMetadata>,
}

impl FirmwareCatalog {
    pub fn new() -> Self {
        let mut items = Vec::new();
        let mut metadata_map = HashMap::new();

        // 1. Google Pixel 8 Pro
        let id_p8p = "fw-pixel-8-pro-uq1a".to_string();
        items.push(FirmwareCatalogItem {
            id: id_p8p.clone(),
            manufacturer: "Google Pixel".to_string(),
            model: "Pixel 8 Pro".to_string(),
            build_id: "UQ1A.240205.004".to_string(),
            android_version: "14.0".to_string(),
            release_date: "2024-02-05".to_string(),
            region: "Global".to_string(),
            verification_checksum: "sha256:7f83b1657ff1fc53b92dc18148a1d65dfc2d4b1fa3d677284addd200126d9069".to_string(),
            download_url: Some("https://flash.android.com/build/UQ1A.240205.004".to_string()),
        });
        metadata_map.insert(
            id_p8p,
            FirmwareMetadata {
                security_patch_level: "2024-02-01".to_string(),
                anti_rollback_index: 1,
                chipset: Some("Google Tensor G3".to_string()),
                model_aliases: vec!["husky".to_string(), "GC3VE".to_string(), "G1MNW".to_string()],
            },
        );

        // 2. Google Pixel 8
        let id_p8 = "fw-pixel-8-up1a".to_string();
        items.push(FirmwareCatalogItem {
            id: id_p8.clone(),
            manufacturer: "Google Pixel".to_string(),
            model: "Pixel 8".to_string(),
            build_id: "UP1A.231005.007".to_string(),
            android_version: "14.0".to_string(),
            release_date: "2023-10-04".to_string(),
            region: "Global".to_string(),
            verification_checksum: "sha256:e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855".to_string(),
            download_url: Some("https://flash.android.com/build/UP1A.231005.007".to_string()),
        });
        metadata_map.insert(
            id_p8,
            FirmwareMetadata {
                security_patch_level: "2023-10-01".to_string(),
                anti_rollback_index: 1,
                chipset: Some("Google Tensor G3".to_string()),
                model_aliases: vec!["shiba".to_string(), "GKWS6".to_string(), "G9BQD".to_string()],
            },
        );

        // 3. Samsung Galaxy S24 Ultra
        let id_s24u = "fw-galaxy-s24u-axb5".to_string();
        items.push(FirmwareCatalogItem {
            id: id_s24u.clone(),
            manufacturer: "Samsung".to_string(),
            model: "Galaxy S24 Ultra (SM-S928B)".to_string(),
            build_id: "S928BXXU1AXB5".to_string(),
            android_version: "14.0".to_string(),
            release_date: "2024-02-15".to_string(),
            region: "EUX".to_string(),
            verification_checksum: "sha256:a1b2c3d4e5f60718293a4b5c6d7e8f901234567890abcdef1234567890abcdef".to_string(),
            download_url: Some("https://www.samsung.com/support".to_string()),
        });
        metadata_map.insert(
            id_s24u,
            FirmwareMetadata {
                security_patch_level: "2024-02-01".to_string(),
                anti_rollback_index: 1,
                chipset: Some("Snapdragon 8 Gen 3 for Galaxy".to_string()),
                model_aliases: vec!["SM-S928B".to_string(), "SM-S928U".to_string(), "Galaxy S24 Ultra".to_string()],
            },
        );

        // 4. Samsung Galaxy S23
        let id_s23 = "fw-galaxy-s23-bwk4".to_string();
        items.push(FirmwareCatalogItem {
            id: id_s23.clone(),
            manufacturer: "Samsung".to_string(),
            model: "Galaxy S23 (SM-S911B)".to_string(),
            build_id: "S911BXXU3BWK4".to_string(),
            android_version: "14.0".to_string(),
            release_date: "2023-11-20".to_string(),
            region: "EUX".to_string(),
            verification_checksum: "sha256:d41d8cd98f00b204e9800998ecf8427e0123456789abcdef0123456789abcdef".to_string(),
            download_url: Some("https://www.samsung.com/support".to_string()),
        });
        metadata_map.insert(
            id_s23,
            FirmwareMetadata {
                security_patch_level: "2023-11-01".to_string(),
                anti_rollback_index: 3, // Bootloader SW REV 3
                chipset: Some("Snapdragon 8 Gen 2 for Galaxy".to_string()),
                model_aliases: vec!["SM-S911B".to_string(), "SM-S911U".to_string(), "Galaxy S23".to_string(), "s911".to_string()],
            },
        );

        // 5. Xiaomi POCO F5
        let id_poco = "fw-poco-f5-umrmixm".to_string();
        items.push(FirmwareCatalogItem {
            id: id_poco.clone(),
            manufacturer: "POCO".to_string(),
            model: "POCO F5".to_string(),
            build_id: "OS1.0.2.0.UMRMIXM".to_string(),
            android_version: "14.0".to_string(),
            release_date: "2024-01-18".to_string(),
            region: "Global".to_string(),
            verification_checksum: "sha256:0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef".to_string(),
            download_url: Some("https://www.po.co/global/support".to_string()),
        });
        metadata_map.insert(
            id_poco,
            FirmwareMetadata {
                security_patch_level: "2024-01-01".to_string(),
                anti_rollback_index: 1,
                chipset: Some("Snapdragon 7+ Gen 2".to_string()),
                model_aliases: vec!["marble".to_string(), "23049PCD8G".to_string()],
            },
        );

        // 6. Redmi Note 12
        let id_redmi = "fw-redmi-note12-tmgmixm".to_string();
        items.push(FirmwareCatalogItem {
            id: id_redmi.clone(),
            manufacturer: "Redmi".to_string(),
            model: "Redmi Note 12 4G".to_string(),
            build_id: "V14.0.8.0.TMGMIXM".to_string(),
            android_version: "13.0".to_string(),
            release_date: "2023-09-12".to_string(),
            region: "Global".to_string(),
            verification_checksum: "sha256:fedcba9876543210fedcba9876543210fedcba9876543210fedcba9876543210".to_string(),
            download_url: Some("https://www.mi.com/global/support".to_string()),
        });
        metadata_map.insert(
            id_redmi,
            FirmwareMetadata {
                security_patch_level: "2023-09-01".to_string(),
                anti_rollback_index: 2,
                chipset: Some("Snapdragon 685".to_string()),
                model_aliases: vec!["tapas".to_string(), "23021RAAEG".to_string(), "Redmi Note 12".to_string()],
            },
        );

        // 7. OnePlus 12
        let id_op12 = "fw-oneplus-12-cph2581".to_string();
        items.push(FirmwareCatalogItem {
            id: id_op12.clone(),
            manufacturer: "OnePlus".to_string(),
            model: "OnePlus 12".to_string(),
            build_id: "CPH2581_14.0.0.604(EX01)".to_string(),
            android_version: "14.0".to_string(),
            release_date: "2024-04-10".to_string(),
            region: "Global / EU".to_string(),
            verification_checksum: "sha256:11223344556677889900aabbccddeeff11223344556677889900aabbccddeeff".to_string(),
            download_url: Some("https://www.oneplus.com/support".to_string()),
        });
        metadata_map.insert(
            id_op12,
            FirmwareMetadata {
                security_patch_level: "2024-04-01".to_string(),
                anti_rollback_index: 1,
                chipset: Some("Snapdragon 8 Gen 3".to_string()),
                model_aliases: vec!["CPH2581".to_string(), "CPH2583".to_string(), "OnePlus 12".to_string()],
            },
        );

        Self {
            items,
            metadata_map,
        }
    }

    pub fn list(&self) -> &[FirmwareCatalogItem] {
        &self.items
    }

    pub fn find_by_id(&self, id: &str) -> Option<&FirmwareCatalogItem> {
        self.items.iter().find(|i| i.id == id)
    }

    pub fn find_by_model(&self, model: &str) -> Vec<&FirmwareCatalogItem> {
        let q = model.to_lowercase();
        self.items
            .iter()
            .filter(|item| {
                if item.model.to_lowercase().contains(&q) {
                    return true;
                }
                if let Some(meta) = self.metadata_map.get(&item.id) {
                    if meta.model_aliases.iter().any(|a| a.to_lowercase().contains(&q)) {
                        return true;
                    }
                }
                false
            })
            .collect()
    }

    pub fn find_by_manufacturer(&self, mfg: &str) -> Vec<&FirmwareCatalogItem> {
        let q = mfg.to_lowercase();
        self.items
            .iter()
            .filter(|item| item.manufacturer.to_lowercase().contains(&q))
            .collect()
    }

    pub fn find_by_region(&self, region: &str) -> Vec<&FirmwareCatalogItem> {
        let q = region.to_lowercase();
        self.items
            .iter()
            .filter(|item| item.region.to_lowercase().contains(&q))
            .collect()
    }

    pub fn get_metadata(&self, firmware_id: &str) -> Option<&FirmwareMetadata> {
        self.metadata_map.get(firmware_id)
    }

    pub fn find_latest_compatible(&self, device: &DeviceInfo) -> Option<&FirmwareCatalogItem> {
        let matches = self.find_by_model(&device.model);
        matches.into_iter().max_by_key(|item| &item.release_date)
    }
}

impl Default for FirmwareCatalog {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_catalog_query_by_model_alias() {
        let catalog = FirmwareCatalog::new();
        // Searching for Pixel 8 Pro codename "husky"
        let results = catalog.find_by_model("husky");
        assert!(!results.is_empty());
        assert_eq!(results[0].model, "Pixel 8 Pro");
    }

    #[test]
    fn test_catalog_query_by_samsung_model_number() {
        let catalog = FirmwareCatalog::new();
        let results = catalog.find_by_model("SM-S911B");
        assert!(!results.is_empty());
        assert!(results[0].model.contains("Galaxy S23"));
    }
}
