use crate::adb::packet::{AdbMessageHeader, A_OPEN};
use crate::common::ProtocolError;
use std::collections::HashMap;
use uat_shared_types::AdbDiagnosticReport;

pub struct AdbDiagnosticRunner;

impl AdbDiagnosticRunner {
    /// Strict security enforcement: ensures only non-destructive diagnostic commands can be executed
    pub fn validate_safe_command(cmd: &str) -> Result<(), ProtocolError> {
        let trimmed = cmd.trim();

        // Disallow dangerous command chaining or shell injection characters
        if trimmed.contains(';') || trimmed.contains('&') || trimmed.contains('|') || trimmed.contains('`') {
            return Err(ProtocolError::SecurityPolicyViolation(
                "Command contains forbidden shell metacharacters".to_string(),
            ));
        }

        // Whitelisted read-only diagnostic prefixes
        let safe_prefixes = [
            "getprop",
            "dumpsys battery",
            "dumpsys usb",
            "cat /proc/version",
            "uname -a",
            "pm list features",
        ];

        let is_safe = safe_prefixes.iter().any(|&prefix| trimmed.starts_with(prefix));
        if !is_safe {
            return Err(ProtocolError::SecurityPolicyViolation(format!(
                "Command '{}' rejected. Universal Android Tool allows strictly read-only diagnostics.",
                trimmed
            )));
        }

        Ok(())
    }

    /// Formats an A_OPEN packet to spawn an authorized diagnostic shell service
    pub fn create_shell_open_packet(local_id: u32, command: &str) -> Result<(AdbMessageHeader, Vec<u8>), ProtocolError> {
        Self::validate_safe_command(command)?;

        let destination = format!("shell:{}\0", command);
        let payload = destination.as_bytes().to_vec();
        let header = AdbMessageHeader::new(A_OPEN, local_id, 0, &payload);
        Ok((header, payload))
    }

    /// Parses Android `getprop` output into a key-value property map
    pub fn parse_getprop_output(raw_output: &str) -> HashMap<String, String> {
        let mut map = HashMap::new();

        for line in raw_output.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            // Standard format: [key]: [value]
            if let Some(colon_pos) = line.find("]: [") {
                let key_part = &line[..colon_pos];
                let val_part = &line[colon_pos + 4..];

                let key = key_part.trim_start_matches('[').trim();
                let val = val_part.trim_end_matches(']').trim();

                if !key.is_empty() {
                    map.insert(key.to_string(), val.to_string());
                }
            }
        }

        map
    }

    /// Generates a standardized AdbDiagnosticReport from extracted system properties
    pub fn build_diagnostic_report(
        is_authorized: bool,
        properties: HashMap<String, String>,
    ) -> AdbDiagnosticReport {
        let manufacturer = properties
            .get("ro.product.manufacturer")
            .cloned()
            .or_else(|| properties.get("ro.product.brand").cloned());

        let model = properties
            .get("ro.product.model")
            .cloned()
            .or_else(|| properties.get("ro.product.name").cloned());

        let android_version = properties.get("ro.build.version.release").cloned();

        let build_id = properties
            .get("ro.build.display.id")
            .cloned()
            .or_else(|| properties.get("ro.build.id").cloned());

        let verified_boot_state = properties
            .get("ro.boot.verifiedbootstate")
            .cloned()
            .or_else(|| properties.get("ro.boot.vbmeta.device_state").cloned());

        let crypto_state = properties.get("ro.crypto.state").cloned();

        // FRP partition presence check: inspect standard FRP property ro.frp.pst
        let frp_partition_present = properties.contains_key("ro.frp.pst")
            || properties.values().any(|v| v.contains("frp") || v.contains("persistent"));

        AdbDiagnosticReport {
            is_authorized,
            manufacturer,
            model,
            android_version,
            build_id,
            verified_boot_state,
            crypto_state,
            frp_partition_present,
            properties,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_security_validation_safe_commands() {
        assert!(AdbDiagnosticRunner::validate_safe_command("getprop").is_ok());
        assert!(AdbDiagnosticRunner::validate_safe_command("getprop ro.product.model").is_ok());
        assert!(AdbDiagnosticRunner::validate_safe_command("dumpsys battery").is_ok());
    }

    #[test]
    fn test_security_validation_blocks_dangerous_commands() {
        assert!(AdbDiagnosticRunner::validate_safe_command("rm -rf /data").is_err());
        assert!(AdbDiagnosticRunner::validate_safe_command("reboot bootloader").is_err());
        assert!(AdbDiagnosticRunner::validate_safe_command("getprop; rm -rf /").is_err());
        assert!(AdbDiagnosticRunner::validate_safe_command("getprop | grep model").is_err());
    }

    #[test]
    fn test_parse_getprop_output() {
        let raw = r#"
[ro.product.manufacturer]: [Google]
[ro.product.model]: [Pixel 8]
[ro.build.version.release]: [14]
[ro.boot.verifiedbootstate]: [green]
[ro.crypto.state]: [encrypted]
[ro.frp.pst]: [/dev/block/by-name/frp]
"#;

        let props = AdbDiagnosticRunner::parse_getprop_output(raw);
        assert_eq!(props.get("ro.product.manufacturer").unwrap(), "Google");
        assert_eq!(props.get("ro.product.model").unwrap(), "Pixel 8");
        assert_eq!(props.get("ro.build.version.release").unwrap(), "14");
        assert_eq!(props.get("ro.boot.verifiedbootstate").unwrap(), "green");

        let report = AdbDiagnosticRunner::build_diagnostic_report(true, props);
        assert!(report.is_authorized);
        assert_eq!(report.manufacturer.as_deref(), Some("Google"));
        assert_eq!(report.model.as_deref(), Some("Pixel 8"));
        assert_eq!(report.verified_boot_state.as_deref(), Some("green"));
        assert_eq!(report.crypto_state.as_deref(), Some("encrypted"));
        assert!(report.frp_partition_present);
    }
}
