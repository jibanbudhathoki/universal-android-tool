use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct OwnershipVerification {
    pub is_verified_owner: bool,
    pub proof_of_purchase_confirmed: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorized_technician_id: Option<String>,
    pub disclaimer_accepted: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_imei_or_serial: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct VerificationResult {
    pub is_authorized: bool,
    pub missing_requirements: Vec<String>,
    pub checkpoint_timestamp: DateTime<Utc>,
}

impl OwnershipVerification {
    pub fn is_authorized_for_recovery(&self) -> bool {
        self.is_verified_owner && self.proof_of_purchase_confirmed && self.disclaimer_accepted
    }

    pub fn verify_eligibility(&self) -> VerificationResult {
        let mut missing = Vec::new();

        if !self.is_verified_owner {
            missing.push("Device owner identity must be verified by official photo ID.".to_string());
        }
        if !self.proof_of_purchase_confirmed {
            missing.push("Valid invoice or authorized proof of purchase documentation must be verified.".to_string());
        }
        if !self.disclaimer_accepted {
            missing.push("Manufacturer compliance and legal recovery disclaimer must be acknowledged and accepted.".to_string());
        }

        let is_authorized = missing.is_empty();

        VerificationResult {
            is_authorized,
            missing_requirements: missing,
            checkpoint_timestamp: Utc::now(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unauthorized_when_proof_missing() {
        let verification = OwnershipVerification {
            is_verified_owner: true,
            proof_of_purchase_confirmed: false,
            authorized_technician_id: Some("TECH-88".to_string()),
            disclaimer_accepted: true,
            device_imei_or_serial: None,
            customer_name: None,
            notes: None,
        };

        assert!(!verification.is_authorized_for_recovery());
        let res = verification.verify_eligibility();
        assert!(!res.is_authorized);
        assert_eq!(res.missing_requirements.len(), 1);
        assert!(res.missing_requirements[0].contains("proof of purchase"));
    }

    #[test]
    fn test_authorized_when_all_criteria_met() {
        let verification = OwnershipVerification {
            is_verified_owner: true,
            proof_of_purchase_confirmed: true,
            authorized_technician_id: Some("TECH-88".to_string()),
            disclaimer_accepted: true,
            device_imei_or_serial: Some("358921094829102".to_string()),
            customer_name: Some("Jane Doe".to_string()),
            notes: Some("Original carrier receipt inspected".to_string()),
        };

        assert!(verification.is_authorized_for_recovery());
        let res = verification.verify_eligibility();
        assert!(res.is_authorized);
        assert!(res.missing_requirements.is_empty());
    }
}
