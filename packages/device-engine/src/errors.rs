use thiserror::Error;

#[derive(Error, Debug)]
pub enum DeviceEngineError {
    #[error("Device not found: {0}")]
    DeviceNotFound(String),
    #[error("Device communication error: {0}")]
    CommunicationError(String),
    #[error("Permission denied to access USB device: {0}")]
    PermissionDenied(String),
    #[error("Platform unsupported: {0}")]
    UnsupportedPlatform(String),
    #[error("Device capability missing: {0}")]
    CapabilityMissing(String),
    #[error("Protocol error: {0}")]
    ProtocolError(String),
    #[error("Security violation: {0}")]
    SecurityViolation(String),
}
