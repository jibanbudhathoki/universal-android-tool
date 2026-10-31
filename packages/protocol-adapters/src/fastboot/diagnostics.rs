use crate::common::ProtocolError;
use std::collections::HashMap;
use uat_shared_types::FastbootDiagnosticReport;

pub struct FastbootDiagnosticRunner;

impl FastbootDiagnosticRunner {
    /// Strict security enforcement: forbids non-diagnostic or destructive Fastboot commands
    pub fn validate_safe_command(cmd: &str) -> Result<(), ProtocolError> {
        let trimmed = cmd.trim();

        // Strictly disallow destructive or modifying Fastboot actions
        let forbidden_substrings = [
            "flash",
            "erase",
            "format",
            "unlock",
            "lock",
            "boot",
            "continue",
            "reboot",
        ];

        for forbidden in &forbidden_substrings {
            if trimmed.contains(forbidden) && !trimmed.starts_with("getvar:") {
                return Err(ProtocolError::SecurityPolicyViolation(format!(
                    "Command '{}' contains restricted keyword '{}'. Only diagnostic queries are permitted.",
                    trimmed, forbidden
                )));
            }
        }

        // Whitelist allowed diagnostic read commands
        let allowed = trimmed.starts_with("getvar:")
            || trimmed == "getvar:all"
            || trimmed == "oem device-info"
            || trimmed == "oem get-bootinfo";

        if !allowed {
            return Err(ProtocolError::SecurityPolicyViolation(format!(
                "Fastboot command '{}' rejected. Universal Android Tool allows strictly read-only diagnostics.",
                trimmed
            )));
        }

        Ok(())
    }

    /// Parses aggregated INFO lines into variable key-value pairs
    pub fn parse_getvar_lines(info_lines: &[String]) -> HashMap<String, String> {
        let mut map = HashMap::new();

        for line in info_lines {
            let mut cleaned = line.trim();

            // Strip common bootloader prefixes like "(bootloader) "
            if let Some(pos) = cleaned.find("(bootloader)") {
                cleaned = cleaned[pos + "(bootloader)".len()..].trim();
            }

            // Standard format: "key: value"
            if let Some(colon_pos) = cleaned.find(':') {
                let key = cleaned[..colon_pos].trim();
                let val = cleaned[colon_pos + 1..].trim();

                if !key.is_empty() {
                    map.insert(key.to_string(), val.to_string());
                }
            }
        }

        map
    }

    /// Generates a standardized FastbootDiagnosticReport from parsed variables
    pub fn build_diagnostic_report(variables: HashMap<String, String>) -> FastbootDiagnosticReport {
        let product = variables.get("product").cloned();
        let serial_number = variables
            .get("serialno")
            .cloned()
            .or_else(|| variables.get("serial").cloned());

        let bootloader_version = variables
            .get("version-bootloader")
            .cloned()
            .or_else(|| variables.get("bootloader").cloned());

        let baseband_version = variables.get("version-baseband").cloned();

        let is_unlocked = variables
            .get("unlocked")
            .map(|v| v.eq_ignore_ascii_case("yes") || v == "1" || v.eq_ignore_ascii_case("true"))
            .unwrap_or(false);

        let is_secure_boot = variables
            .get("secure")
            .map(|v| v.eq_ignore_ascii_case("yes") || v == "1" || v.eq_ignore_ascii_case("true"))
            .unwrap_or(true); // Default to secure if unspecified

        let current_slot = variables
            .get("current-slot")
            .cloned()
            .or_else(|| variables.get("slot-current").cloned());

        let battery_voltage = variables.get("battery-voltage").cloned();

        FastbootDiagnosticReport {
            product,
            serial_number,
            bootloader_version,
            baseband_version,
            is_unlocked,
            is_secure_boot,
            current_slot,
            battery_voltage,
            variables,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fastboot_command_security_whitelist() {
        assert!(FastbootDiagnosticRunner::validate_safe_command("getvar:all").is_ok());
        assert!(FastbootDiagnosticRunner::validate_safe_command("getvar:version-bootloader").is_ok());
        assert!(FastbootDiagnosticRunner::validate_safe_command("getvar:unlocked").is_ok());
        assert!(FastbootDiagnosticRunner::validate_safe_command("oem device-info").is_ok());
    }

    #[test]
    fn test_fastboot_command_security_blocks_destructive() {
        assert!(FastbootDiagnosticRunner::validate_safe_command("flash bootloader bootloader.img").is_err());
        assert!(FastbootDiagnosticRunner::validate_safe_command("erase userdata").is_err());
        assert!(FastbootDiagnosticRunner::validate_safe_command("oem unlock").is_err());
        assert!(FastbootDiagnosticRunner::validate_safe_command("flashing unlock").is_err());
    }

    #[test]
    fn test_parse_getvar_lines_and_report() {
        let lines = vec![
            "(bootloader) product: husky".to_string(),
            "(bootloader) serialno: 34121FD982".to_string(),
            "(bootloader) version-bootloader: husky-1.0-103829".to_string(),
            "(bootloader) unlocked: yes".to_string(),
            "(bootloader) secure: yes".to_string(),
            "(bootloader) current-slot: a".to_string(),
            "(bootloader) battery-voltage: 4210mV".to_string(),
        ];

        let vars = FastbootDiagnosticRunner::parse_getvar_lines(&lines);
        assert_eq!(vars.get("product").unwrap(), "husky");
        assert_eq!(vars.get("unlocked").unwrap(), "yes");

        let report = FastbootDiagnosticRunner::build_diagnostic_report(vars);
        assert_eq!(report.product.as_deref(), Some("husky"));
        assert_eq!(report.serial_number.as_deref(), Some("34121FD982"));
        assert_eq!(report.bootloader_version.as_deref(), Some("husky-1.0-103829"));
        assert!(report.is_unlocked);
        assert!(report.is_secure_boot);
        assert_eq!(report.current_slot.as_deref(), Some("a"));
        assert_eq!(report.battery_voltage.as_deref(), Some("4210mV"));
    }
}
