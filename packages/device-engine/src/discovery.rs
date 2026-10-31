use crate::capabilities::{CapabilityDetector, UsbInterfaceDescriptor};
use crate::errors::DeviceEngineError;
use crate::information_extractor::InformationExtractor;
use uat_device_registry::DeviceRegistry;
use uat_shared_types::{ConnectionMode, DeviceInfo, SupportedProtocol};

#[derive(Debug, Clone)]
pub struct RawUsbDescriptor {
    pub vendor_id: u16,
    pub product_id: u16,
    pub manufacturer_string: Option<String>,
    pub product_string: Option<String>,
    pub serial_number: Option<String>,
    pub interfaces: Vec<UsbInterfaceDescriptor>,
}

impl RawUsbDescriptor {
    pub fn simple(
        vendor_id: u16,
        product_id: u16,
        manufacturer: Option<String>,
        product: Option<String>,
        serial: Option<String>,
    ) -> Self {
        Self {
            vendor_id,
            product_id,
            manufacturer_string: manufacturer,
            product_string: product,
            serial_number: serial,
            interfaces: Vec::new(),
        }
    }
}

pub struct DeviceDiscovery {
    registry: DeviceRegistry,
}

impl DeviceDiscovery {
    pub fn new() -> Self {
        Self {
            registry: DeviceRegistry::new(),
        }
    }

    pub fn registry(&self) -> &DeviceRegistry {
        &self.registry
    }

    pub fn inspect_descriptor(
        &self,
        descriptor: RawUsbDescriptor,
    ) -> Result<DeviceInfo, DeviceEngineError> {
        let identification = self
            .registry
            .identify(descriptor.vendor_id, descriptor.product_id);

        let raw_mfg = descriptor
            .manufacturer_string
            .unwrap_or_else(|| identification.manufacturer.clone());

        let raw_model = descriptor
            .product_string
            .unwrap_or_else(|| identification.model_name.clone());

        let manufacturer = InformationExtractor::sanitize_descriptor_string(&raw_mfg);
        let model = InformationExtractor::sanitize_descriptor_string(&raw_model);

        let id = format!(
            "usb-{:04x}:{:04x}",
            descriptor.vendor_id, descriptor.product_id
        );

        let connection_mode = ConnectionMode::NativeUsb;

        // Evaluate capabilities based on parsed interface descriptors if present
        let capabilities = if !descriptor.interfaces.is_empty() {
            CapabilityDetector::evaluate_from_interfaces(
                &descriptor.interfaces,
                false, // Initial probe is unauthorized until user grants permission
                &connection_mode,
            )
        } else {
            // Fallback heuristics: check if vendor profile indicates known modes
            let mut caps = uat_shared_types::DeviceCapabilities {
                mtp_available: identification.supported_modes.iter().any(|m| m == "mtp"),
                recovery_mode_supported: identification.is_recovery_mode(),
                diagnostic_mode_supported: true,
                ..Default::default()
            };
            if identification.is_adb_capable() {
                caps.adb_available = true;
            }
            if identification.is_fastboot_capable() {
                caps.fastboot_available = true;
            }
            caps
        };

        let mut supported_protocols = vec![SupportedProtocol::UsbMassStorage, SupportedProtocol::Mtp];
        if (capabilities.adb_available || identification.is_adb_capable())
            && !supported_protocols.contains(&SupportedProtocol::Adb)
        {
            supported_protocols.push(SupportedProtocol::Adb);
        }
        if (capabilities.fastboot_available || identification.is_fastboot_capable())
            && !supported_protocols.contains(&SupportedProtocol::Fastboot)
        {
            supported_protocols.push(SupportedProtocol::Fastboot);
        }

        Ok(DeviceInfo {
            id,
            vendor_id: descriptor.vendor_id,
            product_id: descriptor.product_id,
            manufacturer,
            model,
            android_version: None,
            build_number: None,
            // Sensitive serial number is sanitized if present
            serial_number: descriptor
                .serial_number
                .map(|s| InformationExtractor::sanitize_descriptor_string(&s)),
            connection_mode,
            supported_protocols,
            capabilities,
            is_authorized: false,
        })
    }
}

impl Default for DeviceDiscovery {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inspect_descriptor_with_interfaces() {
        let discovery = DeviceDiscovery::new();
        let descriptor = RawUsbDescriptor {
            vendor_id: 1204, // Samsung
            product_id: 26720,
            manufacturer_string: Some("Samsung Inc.".to_string()),
            product_string: Some("Galaxy S23".to_string()),
            serial_number: Some("RF8M123456\u{0000}".to_string()),
            interfaces: vec![UsbInterfaceDescriptor {
                interface_number: 0,
                interface_class: 0xff,
                interface_subclass: 0x42,
                interface_protocol: 0x01,
            }],
        };

        let device = discovery.inspect_descriptor(descriptor).unwrap();
        assert_eq!(device.manufacturer, "Samsung Inc.");
        assert_eq!(device.model, "Galaxy S23");
        assert_eq!(device.serial_number, Some("RF8M123456".to_string()));
    }

    #[test]
    fn test_inspect_descriptor_fallback_to_registry() {
        let discovery = DeviceDiscovery::new();
        // Google Pixel Fastboot mode (PID 53249) with no interfaces parsed yet
        let descriptor = RawUsbDescriptor::simple(6353, 53249, None, None, None);
        let device = discovery.inspect_descriptor(descriptor).unwrap();
        assert_eq!(device.manufacturer, "Google Pixel");
        assert_eq!(device.model, "Pixel Fastboot Bootloader Mode");
        assert!(device.supported_protocols.contains(&SupportedProtocol::Fastboot));
        assert!(device.capabilities.fastboot_available);
    }
}
