use crate::errors::DeviceEngineError;
use uat_shared_types::DeviceInfo;

pub struct InformationExtractor;

impl InformationExtractor {
    pub fn sanitize_descriptor_string(input: &str) -> String {
        input
            .chars()
            .filter(|c| c.is_alphanumeric() || c.is_ascii_punctuation() || *c == ' ')
            .collect::<String>()
            .trim()
            .to_string()
    }

    pub fn extract_hardware_summary(device: &DeviceInfo) -> String {
        format!(
            "{} {} (VID: 0x{:04x}, PID: 0x{:04x}, Mode: {:?})",
            device.manufacturer,
            device.model,
            device.vendor_id,
            device.product_id,
            device.connection_mode
        )
    }

    pub fn filter_sensitive_attributes(
        mut device: DeviceInfo,
        allow_serial_number: bool,
    ) -> DeviceInfo {
        if !allow_serial_number {
            device.serial_number = None;
        }
        device
    }

    pub fn parse_build_prop(
        &self,
        prop_output: &str,
    ) -> Result<(Option<String>, Option<String>), DeviceEngineError> {
        let mut android_version = None;
        let mut build_id = None;

        for line in prop_output.lines() {
            let line = line.trim();
            if let Some(stripped) = line.strip_prefix("ro.build.version.release=") {
                android_version = Some(stripped.trim().to_string());
            } else if let Some(stripped) = line.strip_prefix("ro.build.id=") {
                build_id = Some(stripped.trim().to_string());
            }
        }

        Ok((android_version, build_id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanitize_descriptor() {
        let dirty = "Pixel 8\u{0000}\n  ";
        let cleaned = InformationExtractor::sanitize_descriptor_string(dirty);
        assert_eq!(cleaned, "Pixel 8");
    }

    #[test]
    fn test_parse_build_prop() {
        let extractor = InformationExtractor;
        let output = "ro.build.version.release=14\nro.build.id=UP1A.231005.007\nro.product.model=Pixel 8";
        let (version, build_id) = extractor.parse_build_prop(output).unwrap();
        assert_eq!(version, Some("14".to_string()));
        assert_eq!(build_id, Some("UP1A.231005.007".to_string()));
    }
}
