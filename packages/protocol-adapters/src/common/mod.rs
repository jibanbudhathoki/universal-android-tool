use async_trait::async_trait;
use thiserror::Error;
use uat_shared_types::SupportedProtocol;

#[derive(Error, Debug)]
pub enum ProtocolError {
    #[error("Device not connected or disconnected unexpectedly")]
    Disconnected,
    #[error("Protocol error: {0}")]
    CommunicationError(String),
    #[error("Device authorization required")]
    Unauthorized,
    #[error("Operation timeout")]
    Timeout,
    #[error("Protocol unsupported on this device: {0:?}")]
    Unsupported(SupportedProtocol),
    #[error("Operation rejected by security policy: {0}")]
    SecurityPolicyViolation(String),
}

#[async_trait]
pub trait ProtocolAdapter: Send + Sync {
    fn protocol(&self) -> SupportedProtocol;
    async fn is_available(&self) -> bool;
    async fn connect(&mut self) -> Result<(), ProtocolError>;
    async fn disconnect(&mut self) -> Result<(), ProtocolError>;
}
