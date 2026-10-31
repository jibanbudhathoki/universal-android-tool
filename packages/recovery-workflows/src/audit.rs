use crate::verification::OwnershipVerification;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::sync::{Arc, Mutex};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RecoveryAuditRecord {
    pub id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub device_id: String,
    pub manufacturer: String,
    pub model: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
    pub workflow_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub technician_id: Option<String>,
    pub verification_summary: OwnershipVerification,
    pub disclaimer_text: String,
    pub audit_hash: String,
}

impl RecoveryAuditRecord {
    pub fn compute_hash(
        id: &Uuid,
        timestamp: &DateTime<Utc>,
        device_id: &str,
        workflow_type: &str,
        technician_id: Option<&str>,
        disclaimer_text: &str,
    ) -> String {
        let mut hasher = Sha256::new();
        hasher.update(id.as_bytes());
        hasher.update(timestamp.to_rfc3339().as_bytes());
        hasher.update(device_id.as_bytes());
        hasher.update(workflow_type.as_bytes());
        if let Some(tech) = technician_id {
            hasher.update(tech.as_bytes());
        }
        hasher.update(disclaimer_text.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    pub fn verify_integrity(&self) -> bool {
        let calculated = Self::compute_hash(
            &self.id,
            &self.timestamp,
            &self.device_id,
            &self.workflow_type,
            self.technician_id.as_deref(),
            &self.disclaimer_text,
        );
        calculated == self.audit_hash
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAuditParams {
    pub device_id: String,
    pub manufacturer: String,
    pub model: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
    pub workflow_type: String,
    pub verification: OwnershipVerification,
    pub disclaimer_text: String,
}

#[derive(Clone, Default)]
pub struct RecoveryAuditLogger {
    records: Arc<Mutex<Vec<RecoveryAuditRecord>>>,
}

impl RecoveryAuditLogger {
    pub fn new() -> Self {
        Self {
            records: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn record_workflow(&self, params: CreateAuditParams) -> RecoveryAuditRecord {
        let id = Uuid::new_v4();
        let timestamp = Utc::now();
        let audit_hash = RecoveryAuditRecord::compute_hash(
            &id,
            &timestamp,
            &params.device_id,
            &params.workflow_type,
            params.verification.authorized_technician_id.as_deref(),
            &params.disclaimer_text,
        );

        let record = RecoveryAuditRecord {
            id,
            timestamp,
            device_id: params.device_id,
            manufacturer: params.manufacturer,
            model: params.model,
            serial_number: params.serial_number,
            workflow_type: params.workflow_type,
            technician_id: params.verification.authorized_technician_id.clone(),
            verification_summary: params.verification,
            disclaimer_text: params.disclaimer_text,
            audit_hash,
        };

        if let Ok(mut lock) = self.records.lock() {
            lock.push(record.clone());
        }

        record
    }

    pub fn get_records(&self) -> Vec<RecoveryAuditRecord> {
        self.records.lock().map(|lock| lock.clone()).unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audit_record_creation_and_integrity_check() {
        let logger = RecoveryAuditLogger::new();
        let verification = OwnershipVerification {
            is_verified_owner: true,
            proof_of_purchase_confirmed: true,
            authorized_technician_id: Some("TECH-42".to_string()),
            disclaimer_accepted: true,
            device_imei_or_serial: Some("SN123456789".to_string()),
            customer_name: Some("Alice Smith".to_string()),
            notes: None,
        };

        let record = logger.record_workflow(CreateAuditParams {
            device_id: "dev-pixel8".to_string(),
            manufacturer: "Google".to_string(),
            model: "Pixel 8".to_string(),
            serial_number: Some("SN123456789".to_string()),
            workflow_type: "frp_recovery_guidance".to_string(),
            verification,
            disclaimer_text: "Legal disclaimer for authorized FRP guidance".to_string(),
        });

        assert!(record.verify_integrity());
        assert_eq!(record.technician_id.as_deref(), Some("TECH-42"));
        assert_eq!(logger.get_records().len(), 1);
    }
}
