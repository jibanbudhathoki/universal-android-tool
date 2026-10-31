use async_trait::async_trait;
use crate::common::{ProtocolAdapter, ProtocolError};
use uat_shared_types::SupportedProtocol;

pub struct MtpAdapter {
    connected: bool,
}

impl MtpAdapter {
    pub fn new() -> Self {
        Self { connected: false }
    }
}

impl Default for MtpAdapter {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl ProtocolAdapter for MtpAdapter {
    fn protocol(&self) -> SupportedProtocol {
        SupportedProtocol::Mtp
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
