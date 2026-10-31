use crate::errors::DeviceEngineError;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uat_protocol_adapters::adb::AdbAdapter;
use uat_protocol_adapters::fastboot::FastbootAdapter;
use uat_protocol_adapters::mtp::MtpAdapter;
use uat_protocol_adapters::usb::GenericUsbAdapter;
use uat_protocol_adapters::ProtocolAdapter;
use uat_shared_types::{DeviceInfo, SupportedProtocol};

pub struct ProtocolManager {
    adapters: Arc<RwLock<HashMap<String, Vec<SupportedProtocol>>>>,
}

impl ProtocolManager {
    pub fn new() -> Self {
        Self {
            adapters: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn detect_protocols_for_device(
        &self,
        device: &DeviceInfo,
    ) -> Vec<SupportedProtocol> {
        let mut supported = Vec::new();

        // 1. Generic USB Mass Storage is always probed first for standard endpoints
        let generic = GenericUsbAdapter::new();
        if generic.is_available().await {
            supported.push(SupportedProtocol::UsbMassStorage);
        }

        // 2. MTP Adapter
        let mtp = MtpAdapter::new();
        if mtp.is_available().await {
            supported.push(SupportedProtocol::Mtp);
        }

        // 3. Fastboot Adapter - only when in fastboot connection mode
        if device.connection_mode == uat_shared_types::ConnectionMode::Fastboot {
            let fastboot = FastbootAdapter::new();
            if fastboot.is_available().await {
                supported.push(SupportedProtocol::Fastboot);
            }
        }

        // 4. ADB Adapter - only when authorized or in ADB connection mode
        if device.is_authorized || device.connection_mode == uat_shared_types::ConnectionMode::Adb {
            let adb = AdbAdapter::new();
            if adb.is_available().await {
                supported.push(SupportedProtocol::Adb);
            }
        }

        let mut lock = self.adapters.write().await;
        lock.insert(device.id.clone(), supported.clone());
        supported
    }

    pub async fn verify_protocol_capability(
        &self,
        device_id: &str,
        protocol: &SupportedProtocol,
    ) -> Result<(), DeviceEngineError> {
        let lock = self.adapters.read().await;
        let protocols = lock.get(device_id).ok_or_else(|| {
            DeviceEngineError::CapabilityMissing(format!(
                "No protocol profile evaluated for device {}",
                device_id
            ))
        })?;

        if protocols.contains(protocol) {
            Ok(())
        } else {
            Err(DeviceEngineError::CapabilityMissing(format!(
                "Device {} does not support protocol {:?}",
                device_id, protocol
            )))
        }
    }
}

impl Default for ProtocolManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uat_shared_types::ConnectionMode;

    #[tokio::test]
    async fn test_protocol_detection_unauthorized() {
        let manager = ProtocolManager::new();
        let device = DeviceInfo {
            id: "dev-test".to_string(),
            vendor_id: 1204,
            product_id: 26720,
            manufacturer: "Samsung".to_string(),
            model: "Galaxy S23".to_string(),
            android_version: None,
            build_number: None,
            serial_number: None,
            connection_mode: ConnectionMode::NativeUsb,
            supported_protocols: vec![],
            capabilities: Default::default(),
            is_authorized: false,
        };

        let detected = manager.detect_protocols_for_device(&device).await;
        assert!(detected.contains(&SupportedProtocol::UsbMassStorage));
        assert!(detected.contains(&SupportedProtocol::Mtp));
        // ADB is not assumed on unauthorized generic USB
        assert!(!detected.contains(&SupportedProtocol::Adb));
    }
}
