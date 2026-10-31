use crate::common::ProtocolError;

pub const FASTBOOT_MAX_COMMAND_LEN: usize = 64;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FastbootResponse {
    /// Informational response stream line (can be sent repeatedly before OKAY/FAIL)
    Info(String),
    /// Command completed successfully with optional message
    Okay(String),
    /// Command failed with error message
    Fail(String),
    /// Data stage initiated with expected payload size
    Data(u32),
}

pub struct FastbootPacketCodec;

impl FastbootPacketCodec {
    /// Encodes a Fastboot command string for USB bulk OUT transfer
    pub fn format_command(cmd: &str) -> Result<Vec<u8>, ProtocolError> {
        let trimmed = cmd.trim();
        if trimmed.is_empty() {
            return Err(ProtocolError::CommunicationError(
                "Fastboot command cannot be empty".to_string(),
            ));
        }

        if trimmed.len() > FASTBOOT_MAX_COMMAND_LEN {
            return Err(ProtocolError::CommunicationError(format!(
                "Fastboot command exceeds max length of {} bytes",
                FASTBOOT_MAX_COMMAND_LEN
            )));
        }

        // Fastboot protocol requires raw ASCII string without trailing newline
        Ok(trimmed.as_bytes().to_vec())
    }

    /// Parses an incoming Fastboot response buffer from USB bulk IN
    pub fn parse_response(buf: &[u8]) -> Result<FastbootResponse, ProtocolError> {
        if buf.len() < 4 {
            return Err(ProtocolError::CommunicationError(format!(
                "Fastboot response packet too short: {} bytes",
                buf.len()
            )));
        }

        let prefix = &buf[0..4];
        let payload = String::from_utf8_lossy(&buf[4..]).trim().to_string();

        match prefix {
            b"INFO" => Ok(FastbootResponse::Info(payload)),
            b"OKAY" => Ok(FastbootResponse::Okay(payload)),
            b"FAIL" => Ok(FastbootResponse::Fail(payload)),
            b"DATA" => {
                let hex_str = payload.trim();
                let length = u32::from_str_radix(hex_str, 16).map_err(|e| {
                    ProtocolError::CommunicationError(format!(
                        "Invalid DATA response hex length '{}': {}",
                        hex_str, e
                    ))
                })?;
                Ok(FastbootResponse::Data(length))
            }
            other => Err(ProtocolError::CommunicationError(format!(
                "Unknown Fastboot response prefix: {:?}",
                String::from_utf8_lossy(other)
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_command() {
        let bytes = FastbootPacketCodec::format_command("getvar:all").unwrap();
        assert_eq!(bytes, b"getvar:all");
    }

    #[test]
    fn test_parse_info_response() {
        let raw = b"INFOversion-bootloader: 1.0.4\n";
        let resp = FastbootPacketCodec::parse_response(raw).unwrap();
        assert_eq!(resp, FastbootResponse::Info("version-bootloader: 1.0.4".to_string()));
    }

    #[test]
    fn test_parse_okay_response() {
        let raw = b"OKAYFinished.";
        let resp = FastbootPacketCodec::parse_response(raw).unwrap();
        assert_eq!(resp, FastbootResponse::Okay("Finished.".to_string()));
    }

    #[test]
    fn test_parse_fail_response() {
        let raw = b"FAILUnknown command";
        let resp = FastbootPacketCodec::parse_response(raw).unwrap();
        assert_eq!(resp, FastbootResponse::Fail("Unknown command".to_string()));
    }

    #[test]
    fn test_parse_data_response() {
        let raw = b"DATA00100000"; // 1MB hex
        let resp = FastbootPacketCodec::parse_response(raw).unwrap();
        assert_eq!(resp, FastbootResponse::Data(0x00100000));
    }
}
