use crate::errors::DeviceEngineError;
use chrono::Utc;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uat_shared_types::{OperationRecord, OperationStatus};
use uuid::Uuid;

pub struct OperationManager {
    operations: Arc<RwLock<HashMap<Uuid, OperationRecord>>>,
}

impl OperationManager {
    pub fn new() -> Self {
        Self {
            operations: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn start_operation(
        &self,
        device_id: String,
        operation_type: String,
    ) -> OperationRecord {
        let op_id = Uuid::new_v4();
        let started_at = Utc::now();
        let initial_log = format!(
            "[{}] Operation '{}' queued for device '{}'",
            started_at.format("%H:%M:%S"),
            operation_type,
            device_id
        );

        let record = OperationRecord {
            id: op_id,
            device_id,
            operation_type,
            status: OperationStatus::Running,
            started_at,
            completed_at: None,
            progress_percent: 0,
            logs: vec![initial_log],
            error_message: None,
        };

        let mut ops = self.operations.write().await;
        ops.insert(op_id, record.clone());
        record
    }

    pub async fn update_progress(
        &self,
        op_id: Uuid,
        progress_percent: u8,
        log_message: Option<String>,
    ) -> Result<OperationRecord, DeviceEngineError> {
        let mut ops = self.operations.write().await;
        let record = ops.get_mut(&op_id).ok_or_else(|| {
            DeviceEngineError::CommunicationError(format!("Operation {} not found", op_id))
        })?;

        record.progress_percent = progress_percent.min(100);
        if let Some(msg) = log_message {
            let timestamped = format!("[{}] {}", Utc::now().format("%H:%M:%S"), msg);
            record.logs.push(timestamped);
        }

        Ok(record.clone())
    }

    pub async fn complete_operation(
        &self,
        op_id: Uuid,
        final_log: Option<String>,
    ) -> Result<OperationRecord, DeviceEngineError> {
        let mut ops = self.operations.write().await;
        let record = ops.get_mut(&op_id).ok_or_else(|| {
            DeviceEngineError::CommunicationError(format!("Operation {} not found", op_id))
        })?;

        record.status = OperationStatus::Completed;
        record.progress_percent = 100;
        record.completed_at = Some(Utc::now());

        if let Some(msg) = final_log {
            let timestamped = format!("[{}] {}", Utc::now().format("%H:%M:%S"), msg);
            record.logs.push(timestamped);
        }

        Ok(record.clone())
    }

    pub async fn fail_operation(
        &self,
        op_id: Uuid,
        error_message: String,
    ) -> Result<OperationRecord, DeviceEngineError> {
        let mut ops = self.operations.write().await;
        let record = ops.get_mut(&op_id).ok_or_else(|| {
            DeviceEngineError::CommunicationError(format!("Operation {} not found", op_id))
        })?;

        record.status = OperationStatus::Failed;
        record.completed_at = Some(Utc::now());
        record.error_message = Some(error_message.clone());

        let timestamped = format!(
            "[{}] [ERROR] Operation failed: {}",
            Utc::now().format("%H:%M:%S"),
            error_message
        );
        record.logs.push(timestamped);

        Ok(record.clone())
    }

    pub async fn cancel_operation(
        &self,
        op_id: Uuid,
        reason: String,
    ) -> Result<OperationRecord, DeviceEngineError> {
        let mut ops = self.operations.write().await;
        let record = ops.get_mut(&op_id).ok_or_else(|| {
            DeviceEngineError::CommunicationError(format!("Operation {} not found", op_id))
        })?;

        record.status = OperationStatus::Cancelled;
        record.completed_at = Some(Utc::now());

        let timestamped = format!(
            "[{}] [CANCELLED] Operation cancelled: {}",
            Utc::now().format("%H:%M:%S"),
            reason
        );
        record.logs.push(timestamped);

        Ok(record.clone())
    }

    pub async fn get_operation(&self, op_id: Uuid) -> Option<OperationRecord> {
        let ops = self.operations.read().await;
        ops.get(&op_id).cloned()
    }

    pub async fn list_all_operations(&self) -> Vec<OperationRecord> {
        let ops = self.operations.read().await;
        let mut list: Vec<OperationRecord> = ops.values().cloned().collect();
        list.sort_by(|a, b| b.started_at.cmp(&a.started_at));
        list
    }

    pub async fn list_operations_for_device(&self, device_id: &str) -> Vec<OperationRecord> {
        let ops = self.operations.read().await;
        let mut list: Vec<OperationRecord> = ops
            .values()
            .filter(|o| o.device_id == device_id)
            .cloned()
            .collect();
        list.sort_by(|a, b| b.started_at.cmp(&a.started_at));
        list
    }
}

impl Default for OperationManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_operation_lifecycle() {
        let manager = OperationManager::new();
        let op = manager
            .start_operation("dev-1".to_string(), "Diagnostics".to_string())
            .await;
        assert_eq!(op.status, OperationStatus::Running);
        assert_eq!(op.progress_percent, 0);

        let updated = manager
            .update_progress(op.id, 50, Some("Halfway done".to_string()))
            .await
            .unwrap();
        assert_eq!(updated.progress_percent, 50);

        let completed = manager
            .complete_operation(op.id, Some("All tests passed".to_string()))
            .await
            .unwrap();
        assert_eq!(completed.status, OperationStatus::Completed);
        assert_eq!(completed.progress_percent, 100);
        assert!(completed.completed_at.is_some());
    }
}
