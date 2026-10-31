use async_trait::async_trait;
use crate::common::{ProtocolAdapter, ProtocolError};
use uat_shared_types::SupportedProtocol;

pub struct GenericUsbAdapter {
    connected: bool,
}

impl GenericUsbAdapter {
    pub fn new() -> Self {
        Self { connected: false }
    }
}

impl Default for GenericUsbAdapter {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl ProtocolAdapter for GenericUsbAdapter {
    fn protocol(&self) -> SupportedProtocol {
        SupportedProtocol::UsbMassStorage
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
