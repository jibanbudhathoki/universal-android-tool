use std::sync::Arc;
use uat_device_engine::DeviceEngine;
use uat_firmware_manager::FirmwareCatalog;
use uat_recovery_workflows::RecoveryAuditLogger;

#[derive(Clone)]
pub struct AppState {
    pub device_engine: Arc<DeviceEngine>,
    pub firmware_catalog: Arc<FirmwareCatalog>,
    pub audit_logger: Arc<RecoveryAuditLogger>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            device_engine: Arc::new(DeviceEngine::new()),
            firmware_catalog: Arc::new(FirmwareCatalog::new()),
            audit_logger: Arc::new(RecoveryAuditLogger::new()),
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}
