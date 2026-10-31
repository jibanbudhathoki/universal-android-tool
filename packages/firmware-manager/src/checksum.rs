use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ChecksumVerificationResult {
    pub matches: bool,
    pub expected_checksum: String,
    pub calculated_checksum: String,
    pub status_message: String,
}

pub struct ChecksumVerifier;

impl ChecksumVerifier {
    /// Computes the standard lowercase hex SHA-256 digest of arbitrary payload bytes
    pub fn compute_sha256(data: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data);
        format!("{:x}", hasher.finalize())
    }

    /// Verifies binary payload against an expected SHA-256 checksum string (handles optional 'sha256:' prefix)
    pub fn verify_bytes(data: &[u8], expected_checksum: &str) -> ChecksumVerificationResult {
        let calculated = Self::compute_sha256(data);
        let normalized_expected = expected_checksum
            .trim()
            .trim_start_matches("sha256:")
            .to_lowercase();

        let matches = calculated == normalized_expected;
        let status_message = if matches {
            "Cryptographic integrity verified. SHA-256 checksum matches official catalog payload.".to_string()
        } else {
            format!(
                "CHECKSUM MISMATCH: Payload altered or corrupted. Expected '{}', calculated '{}'. Flashing strictly blocked.",
                normalized_expected, calculated
            )
        };

        ChecksumVerificationResult {
            matches,
            expected_checksum: normalized_expected,
            calculated_checksum: calculated,
            status_message,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_checksum_verifier_matching() {
        let data = b"official android stock payload header";
        let expected = ChecksumVerifier::compute_sha256(data);
        let formatted = format!("sha256:{}", expected);

        let res = ChecksumVerifier::verify_bytes(data, &formatted);
        assert!(res.matches);
        assert!(res.status_message.contains("verified"));
    }

    #[test]
    fn test_checksum_verifier_mismatch() {
        let data = b"clean payload";
        let tampered_data = b"tampered payload";
        let expected = ChecksumVerifier::compute_sha256(data);

        let res = ChecksumVerifier::verify_bytes(tampered_data, &expected);
        assert!(!res.matches);
        assert!(res.status_message.contains("MISMATCH"));
    }
}
