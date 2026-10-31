use crate::common::ProtocolError;

// Standard ADB protocol command magic constants (little-endian ASCII)
pub const A_SYNC: u32 = 0x434e5953; // "SYNC"
pub const A_CNXN: u32 = 0x4e584e43; // "CNXN"
pub const A_AUTH: u32 = 0x48545541; // "AUTH"
pub const A_OPEN: u32 = 0x4e45504f; // "OPEN"
pub const A_OKAY: u32 = 0x59414b4f; // "OKAY"
pub const A_CLSE: u32 = 0x45534c43; // "CLSE"
pub const A_WRTE: u32 = 0x45545257; // "WRTE"

pub const ADB_VERSION: u32 = 0x01000000; // ADB v1.0.0
pub const ADB_MAX_DATA_PAYLOAD: u32 = 256 * 1024; // 256KB modern payload buffer

/// Represents the standard 24-byte ADB USB message header
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdbMessageHeader {
    pub command: u32,
    pub arg0: u32,
    pub arg1: u32,
    pub data_length: u32,
    pub data_check: u32,
    pub magic: u32,
}

impl AdbMessageHeader {
    /// Constructs a message header with auto-calculated checksum and magic value
    pub fn new(command: u32, arg0: u32, arg1: u32, payload: &[u8]) -> Self {
        let data_check = Self::calculate_checksum(payload);
        Self {
            command,
            arg0,
            arg1,
            data_length: payload.len() as u32,
            data_check,
            magic: command ^ 0xFFFFFFFF,
        }
    }

    /// Sum of all bytes in the data payload
    pub fn calculate_checksum(data: &[u8]) -> u32 {
        data.iter().map(|&b| b as u32).sum()
    }

    /// Encodes header into exact 24-byte little-endian wire representation
    pub fn to_bytes(&self) -> [u8; 24] {
        let mut buf = [0u8; 24];
        buf[0..4].copy_from_slice(&self.command.to_le_bytes());
        buf[4..8].copy_from_slice(&self.arg0.to_le_bytes());
        buf[8..12].copy_from_slice(&self.arg1.to_le_bytes());
        buf[12..16].copy_from_slice(&self.data_length.to_le_bytes());
        buf[16..20].copy_from_slice(&self.data_check.to_le_bytes());
        buf[20..24].copy_from_slice(&self.magic.to_le_bytes());
        buf
    }

    /// Decodes and validates a 24-byte ADB header from raw bytes
    pub fn from_bytes(bytes: &[u8; 24]) -> Result<Self, ProtocolError> {
        let command = u32::from_le_bytes(bytes[0..4].try_into().unwrap());
        let arg0 = u32::from_le_bytes(bytes[4..8].try_into().unwrap());
        let arg1 = u32::from_le_bytes(bytes[8..12].try_into().unwrap());
        let data_length = u32::from_le_bytes(bytes[12..16].try_into().unwrap());
        let data_check = u32::from_le_bytes(bytes[16..20].try_into().unwrap());
        let magic = u32::from_le_bytes(bytes[20..24].try_into().unwrap());

        // Validate command magic
        if magic != (command ^ 0xFFFFFFFF) {
            return Err(ProtocolError::CommunicationError(format!(
                "Invalid ADB message magic: expected 0x{:08x}, got 0x{:08x}",
                command ^ 0xFFFFFFFF,
                magic
            )));
        }

        if data_length > ADB_MAX_DATA_PAYLOAD {
            return Err(ProtocolError::CommunicationError(format!(
                "ADB payload length 0x{:08x} exceeds maximum supported 0x{:08x}",
                data_length, ADB_MAX_DATA_PAYLOAD
            )));
        }

        Ok(Self {
            command,
            arg0,
            arg1,
            data_length,
            data_check,
            magic,
        })
    }

    /// Verifies that an incoming payload matches the header's declared length and checksum
    pub fn validate_payload(&self, payload: &[u8]) -> Result<(), ProtocolError> {
        if payload.len() as u32 != self.data_length {
            return Err(ProtocolError::CommunicationError(format!(
                "Payload size mismatch: expected {} bytes, got {}",
                self.data_length,
                payload.len()
            )));
        }

        let actual_check = Self::calculate_checksum(payload);
        if actual_check != self.data_check {
            return Err(ProtocolError::CommunicationError(format!(
                "Payload checksum mismatch: expected 0x{:08x}, calculated 0x{:08x}",
                self.data_check, actual_check
            )));
        }

        Ok(())
    }

    pub fn command_name(&self) -> &'static str {
        match self.command {
            A_SYNC => "SYNC",
            A_CNXN => "CNXN",
            A_AUTH => "AUTH",
            A_OPEN => "OPEN",
            A_OKAY => "OKAY",
            A_CLSE => "CLSE",
            A_WRTE => "WRTE",
            _ => "UNKNOWN",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adb_header_serialization() {
        let payload = b"host::Universal-Android-Tool\0";
        let header = AdbMessageHeader::new(A_CNXN, ADB_VERSION, ADB_MAX_DATA_PAYLOAD, payload);

        let encoded = header.to_bytes();
        assert_eq!(encoded.len(), 24);

        let decoded = AdbMessageHeader::from_bytes(&encoded).expect("Failed to parse valid header");
        assert_eq!(decoded.command, A_CNXN);
        assert_eq!(decoded.arg0, ADB_VERSION);
        assert_eq!(decoded.arg1, ADB_MAX_DATA_PAYLOAD);
        assert_eq!(decoded.data_length, payload.len() as u32);
        assert_eq!(decoded.command_name(), "CNXN");

        assert!(decoded.validate_payload(payload).is_ok());
    }

    #[test]
    fn test_corrupt_magic_rejection() {
        let mut bytes = [0u8; 24];
        bytes[0..4].copy_from_slice(&A_CNXN.to_le_bytes());
        // Deliberately incorrect magic
        bytes[20..24].copy_from_slice(&0x12345678u32.to_le_bytes());

        let res = AdbMessageHeader::from_bytes(&bytes);
        assert!(res.is_err());
    }
}
