pub mod auth;
pub mod diagnostics;
pub mod packet;

use async_trait::async_trait;
use crate::common::{ProtocolAdapter, ProtocolError};
use auth::{AdbAuthStatus, AdbHandshakeManager};
use diagnostics::AdbDiagnosticRunner;
use packet::AdbMessageHeader;
use uat_shared_types::{AdbDiagnosticReport, SupportedProtocol};

pub struct AdbAdapter {
    connected: bool,
    auth_status: AdbAuthStatus,
    handshake_manager: AdbHandshakeManager,
}

impl AdbAdapter {
    pub fn new() -> Self {
        Self {
            connected: false,
            auth_status: AdbAuthStatus::Unauthenticated,
            handshake_manager: AdbHandshakeManager::default(),
        }
    }

    pub fn auth_status(&self) -> &AdbAuthStatus {
        &self.auth_status
    }

    pub fn is_authorized(&self) -> bool {
        matches!(self.auth_status, AdbAuthStatus::Authorized { .. })
    }

    /// Generates initial A_CNXN handshake packet
    pub fn create_handshake_packet(&self) -> (AdbMessageHeader, Vec<u8>) {
        self.handshake_manager.create_connect_packet()
    }

    /// Updates internal authentication status according to device response
    pub fn process_handshake_response(
        &mut self,
        header: &AdbMessageHeader,
        payload: &[u8],
    ) -> Result<AdbAuthStatus, ProtocolError> {
        let status = self.handshake_manager.handle_device_handshake_response(header, payload)?;
        self.auth_status = status.clone();
        if self.is_authorized() {
            self.connected = true;
        }
        Ok(status)
    }

    /// Generates safe diagnostic shell command packet
    pub fn create_diagnostic_query(
        &self,
        command: &str,
    ) -> Result<(AdbMessageHeader, Vec<u8>), ProtocolError> {
        if !self.is_authorized() {
            return Err(ProtocolError::Unauthorized);
        }
        AdbDiagnosticRunner::create_shell_open_packet(1, command)
    }

    /// Parses raw getprop output into an AdbDiagnosticReport
    pub fn parse_diagnostic_properties(&self, raw_getprop: &str) -> AdbDiagnosticReport {
        let props = AdbDiagnosticRunner::parse_getprop_output(raw_getprop);
        AdbDiagnosticRunner::build_diagnostic_report(self.is_authorized(), props)
    }
}

impl Default for AdbAdapter {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl ProtocolAdapter for AdbAdapter {
    fn protocol(&self) -> SupportedProtocol {
        SupportedProtocol::Adb
    }

    async fn is_available(&self) -> bool {
        true
    }

    async fn connect(&mut self) -> Result<(), ProtocolError> {
        // Initial state before handshake exchange
        self.connected = true;
        Ok(())
    }

    async fn disconnect(&mut self) -> Result<(), ProtocolError> {
        self.connected = false;
        self.auth_status = AdbAuthStatus::Unauthenticated;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use packet::{A_CNXN, ADB_MAX_DATA_PAYLOAD, ADB_VERSION};

    #[tokio::test]
    async fn test_adb_adapter_lifecycle() {
        let mut adapter = AdbAdapter::new();
        assert_eq!(adapter.protocol(), SupportedProtocol::Adb);
        assert!(!adapter.is_authorized());

        let (header, _payload) = adapter.create_handshake_packet();
        assert_eq!(header.command, A_CNXN);

        // Simulate authorized device response
        let resp_banner = b"device::ro.product.model=Pixel8\0";
        let resp_header = AdbMessageHeader::new(A_CNXN, ADB_VERSION, ADB_MAX_DATA_PAYLOAD, resp_banner);
        let status = adapter.process_handshake_response(&resp_header, resp_banner).unwrap();

        assert!(matches!(status, AdbAuthStatus::Authorized { .. }));
        assert!(adapter.is_authorized());

        // Now authorized diagnostic command succeeds
        let (diag_header, diag_payload) = adapter.create_diagnostic_query("getprop").unwrap();
        assert_eq!(diag_header.command, packet::A_OPEN);
        assert_eq!(diag_payload, b"shell:getprop\0");
    }

    #[tokio::test]
    async fn test_unauthorized_query_rejected() {
        let adapter = AdbAdapter::new();
        // Unauthorized adapter rejects queries immediately
        let res = adapter.create_diagnostic_query("getprop");
        assert!(matches!(res, Err(ProtocolError::Unauthorized)));
    }
}
