pub mod diagnostics;
pub mod packet;

use async_trait::async_trait;
use crate::common::{ProtocolAdapter, ProtocolError};
use diagnostics::FastbootDiagnosticRunner;
use packet::{FastbootPacketCodec, FastbootResponse};
use uat_shared_types::{FastbootDiagnosticReport, SupportedProtocol};

pub struct FastbootAdapter {
    connected: bool,
}

impl FastbootAdapter {
    pub fn new() -> Self {
        Self { connected: false }
    }

    pub fn is_connected(&self) -> bool {
        self.connected
    }

    /// Prepares safe, validated command bytes for transmission over USB bulk OUT
    pub fn format_command(&self, cmd: &str) -> Result<Vec<u8>, ProtocolError> {
        FastbootDiagnosticRunner::validate_safe_command(cmd)?;
        FastbootPacketCodec::format_command(cmd)
    }

    /// Decodes a response packet from USB bulk IN
    pub fn parse_response(&self, buf: &[u8]) -> Result<FastbootResponse, ProtocolError> {
        FastbootPacketCodec::parse_response(buf)
    }

    /// Builds a diagnostic report from collected getvar output lines
    pub fn collect_diagnostic_variables(&self, info_lines: &[String]) -> FastbootDiagnosticReport {
        let vars = FastbootDiagnosticRunner::parse_getvar_lines(info_lines);
        FastbootDiagnosticRunner::build_diagnostic_report(vars)
    }
}

impl Default for FastbootAdapter {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl ProtocolAdapter for FastbootAdapter {
    fn protocol(&self) -> SupportedProtocol {
        SupportedProtocol::Fastboot
    }

    async fn is_available(&self) -> bool {
        true
    }

    async fn connect(&mut self) -> Result<(), ProtocolError> {
        self.connected = true;
        Ok(())
    }

    async fn disconnect(&mut self) -> Result<(), ProtocolError> {
        self.connected = false;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_fastboot_adapter_command_and_response() {
        let mut adapter = FastbootAdapter::new();
        assert_eq!(adapter.protocol(), SupportedProtocol::Fastboot);

        adapter.connect().await.unwrap();
        assert!(adapter.is_connected());

        // Safe command formatting
        let cmd = adapter.format_command("getvar:all").unwrap();
        assert_eq!(cmd, b"getvar:all");

        // Parse response
        let resp = adapter.parse_response(b"OKAY").unwrap();
        assert_eq!(resp, FastbootResponse::Okay("".to_string()));

        // Destructive command blocked
        let flash_err = adapter.format_command("flash bootloader boot.img");
        assert!(flash_err.is_err());
    }
}
