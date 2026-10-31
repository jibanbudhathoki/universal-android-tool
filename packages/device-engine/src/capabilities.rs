use uat_shared_types::{ConnectionMode, DeviceCapabilities, DeviceInfo, SupportedProtocol};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UsbInterfaceDescriptor {
    pub interface_number: u8,
    pub interface_class: u8,
    pub interface_subclass: u8,
    pub interface_protocol: u8,
}

pub struct CapabilityDetector;

impl CapabilityDetector {
    /// Detects capabilities based on USB interface classes and connection authorization.
    pub fn evaluate_from_interfaces(
        interfaces: &[UsbInterfaceDescriptor],
        is_authorized: bool,
        connection_mode: &ConnectionMode,
    ) -> DeviceCapabilities {
        let mut adb_found = false;
        let mut fastboot_found = false;
        let mut mtp_found = false;

        for iface in interfaces {
            // ADB Interface: Class 0xFF (Vendor Specific), Subclass 0x42, Protocol 0x01
            if iface.interface_class == 0xff
                && iface.interface_subclass == 0x42
                && iface.interface_protocol == 0x01
            {
                adb_found = true;
            }

            // Fastboot Interface: Class 0xFF (Vendor Specific), Subclass 0x42, Protocol 0x03
            if iface.interface_class == 0xff
                && iface.interface_subclass == 0x42
                && iface.interface_protocol == 0x03
            {
                fastboot_found = true;
            }

            // MTP Interface: Class 0x06 (Still Image) or Class 0xFF, Subclass 0xFF
            if iface.interface_class == 0x06
                || (iface.interface_class == 0xff && iface.interface_subclass == 0xff)
            {
                mtp_found = true;
            }
        }

        // Fastboot mode overrides interface heuristics
        if *connection_mode == ConnectionMode::Fastboot {
            fastboot_found = true;
        }

        DeviceCapabilities {
            adb_available: adb_found && is_authorized,
            fastboot_available: fastboot_found,
            mtp_available: mtp_found,
            diagnostic_mode_supported: adb_found || fastboot_found,
            recovery_mode_supported: fastboot_found,
            oem_lock_status_readable: fastboot_found,
            authorized_reset_supported: adb_found || fastboot_found,
        }
    }

    /// Evaluates capabilities from known device info and connection state.
    pub fn evaluate(device: &DeviceInfo) -> DeviceCapabilities {
        let has_adb = device.supported_protocols.contains(&SupportedProtocol::Adb);
        let has_fastboot = device.supported_protocols.contains(&SupportedProtocol::Fastboot);

        let is_fastboot_mode = device.connection_mode == ConnectionMode::Fastboot;

        DeviceCapabilities {
            adb_available: has_adb && device.is_authorized,
            fastboot_available: has_fastboot || is_fastboot_mode,
            mtp_available: true,
            diagnostic_mode_supported: has_adb || has_fastboot || is_fastboot_mode,
            recovery_mode_supported: is_fastboot_mode,
            oem_lock_status_readable: is_fastboot_mode,
            authorized_reset_supported: (has_adb && device.is_authorized) || is_fastboot_mode,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interface_evaluation() {
        let interfaces = vec![
            UsbInterfaceDescriptor {
                interface_number: 0,
                interface_class: 0xff,
                interface_subclass: 0x42,
                interface_protocol: 0x01, // ADB
            },
        ];

        // When unauthorized, ADB commands must NOT be exposed
        let caps_unauthorized = CapabilityDetector::evaluate_from_interfaces(
            &interfaces,
            false,
            &ConnectionMode::NativeUsb,
        );
        assert!(!caps_unauthorized.adb_available);
        assert!(caps_unauthorized.diagnostic_mode_supported);

        // When authorized, ADB commands are available
        let caps_authorized = CapabilityDetector::evaluate_from_interfaces(
            &interfaces,
            true,
            &ConnectionMode::NativeUsb,
        );
        assert!(caps_authorized.adb_available);
    }
}
