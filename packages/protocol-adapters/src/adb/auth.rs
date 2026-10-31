use crate::adb::packet::{AdbMessageHeader, ADB_MAX_DATA_PAYLOAD, ADB_VERSION, A_AUTH, A_CNXN};
use crate::common::ProtocolError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AdbAuthType {
    Token = 1,
    Signature = 2,
    RsaPublicKey = 3,
}

impl TryFrom<u32> for AdbAuthType {
    type Error = ProtocolError;

    fn try_from(val: u32) -> Result<Self, Self::Error> {
        match val {
            1 => Ok(AdbAuthType::Token),
            2 => Ok(AdbAuthType::Signature),
            3 => Ok(AdbAuthType::RsaPublicKey),
            other => Err(ProtocolError::CommunicationError(format!(
                "Unknown ADB AUTH packet type: {}",
                other
            ))),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AdbAuthStatus {
    Unauthenticated,
    /// Device has prompted the user with the authorization dialog ("Allow USB debugging?")
    AwaitingUserPrompt {
        token: Vec<u8>,
    },
    /// Device has accepted connection and ADB debugging is authorized
    Authorized {
        device_banner: String,
    },
    /// Device rejected or connection closed
    Rejected(String),
}

pub struct AdbHandshakeManager {
    system_identity: String,
}

impl AdbHandshakeManager {
    pub fn new(system_identity: Option<&str>) -> Self {
        Self {
            system_identity: system_identity
                .unwrap_or("host::Universal-Android-Tool")
                .to_string(),
        }
    }

    /// Generates the initial A_CNXN handshake packet sent to the Android device
    pub fn create_connect_packet(&self) -> (AdbMessageHeader, Vec<u8>) {
        let mut payload = self.system_identity.as_bytes().to_vec();
        payload.push(0); // Null-terminated string

        let header = AdbMessageHeader::new(A_CNXN, ADB_VERSION, ADB_MAX_DATA_PAYLOAD, &payload);
        (header, payload)
    }

    /// Evaluates the device's reply to the initial connection packet
    pub fn handle_device_handshake_response(
        &self,
        header: &AdbMessageHeader,
        payload: &[u8],
    ) -> Result<AdbAuthStatus, ProtocolError> {
        header.validate_payload(payload)?;

        match header.command {
            A_CNXN => {
                // Device directly accepted without auth challenge (already authorized or user accepted previously)
                let banner = String::from_utf8_lossy(payload).trim_end_matches('\0').to_string();
                tracing::info!("ADB handshake authorized: banner='{}'", banner);
                Ok(AdbAuthStatus::Authorized {
                    device_banner: banner,
                })
            }
            A_AUTH => {
                let auth_type = AdbAuthType::try_from(header.arg0)?;
                match auth_type {
                    AdbAuthType::Token => {
                        // Device provided a 20-byte random challenge token and awaits RSA signature / public key acceptance
                        tracing::info!("Device requested ADB authentication challenge token");
                        Ok(AdbAuthStatus::AwaitingUserPrompt {
                            token: payload.to_vec(),
                        })
                    }
                    _ => Ok(AdbAuthStatus::AwaitingUserPrompt {
                        token: payload.to_vec(),
                    }),
                }
            }
            other => Err(ProtocolError::CommunicationError(format!(
                "Unexpected packet command during handshake: 0x{:08x}",
                other
            ))),
        }
    }
}

impl Default for AdbHandshakeManager {
    fn default() -> Self {
        Self::new(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_connect_packet() {
        let manager = AdbHandshakeManager::default();
        let (header, payload) = manager.create_connect_packet();

        assert_eq!(header.command, A_CNXN);
        assert_eq!(header.data_length, payload.len() as u32);
        assert!(payload.ends_with(&[0]));
    }

    #[test]
    fn test_handle_authorized_response() {
        let manager = AdbHandshakeManager::default();
        let banner = b"device::ro.product.name=Pixel8;ro.build.version.release=14\0";
        let header = AdbMessageHeader::new(A_CNXN, ADB_VERSION, ADB_MAX_DATA_PAYLOAD, banner);

        let status = manager
            .handle_device_handshake_response(&header, banner)
            .expect("Failed to handle authorized response");

        match status {
            AdbAuthStatus::Authorized { device_banner } => {
                assert!(device_banner.contains("Pixel8"));
            }
            _ => panic!("Expected Authorized status"),
        }
    }

    #[test]
    fn test_handle_unauthorized_token_challenge() {
        let manager = AdbHandshakeManager::default();
        let token = vec![0x11; 20];
        let header = AdbMessageHeader::new(A_AUTH, 1, 0, &token);

        let status = manager
            .handle_device_handshake_response(&header, &token)
            .expect("Failed to handle token response");

        match status {
            AdbAuthStatus::AwaitingUserPrompt { token: returned_token } => {
                assert_eq!(returned_token.len(), 20);
            }
            _ => panic!("Expected AwaitingUserPrompt status"),
        }
    }
}
