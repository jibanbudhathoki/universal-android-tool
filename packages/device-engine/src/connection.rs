use crate::errors::DeviceEngineError;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uat_shared_types::DeviceInfo;

pub struct ConnectionManager {
    connected_devices: Arc<RwLock<HashMap<String, DeviceInfo>>>,
}

impl ConnectionManager {
    pub fn new() -> Self {
        Self {
            connected_devices: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn register_device(&self, device: DeviceInfo) -> Result<(), DeviceEngineError> {
        let mut devices = self.connected_devices.write().await;
        devices.insert(device.id.clone(), device);
        Ok(())
    }

    pub async fn unregister_device(&self, device_id: &str) -> Option<DeviceInfo> {
        let mut devices = self.connected_devices.write().await;
        devices.remove(device_id)
    }

    pub async fn get_device(&self, device_id: &str) -> Option<DeviceInfo> {
        let devices = self.connected_devices.read().await;
        devices.get(device_id).cloned()
    }

    pub async fn list_devices(&self) -> Vec<DeviceInfo> {
        let devices = self.connected_devices.read().await;
        devices.values().cloned().collect()
    }
}

impl Default for ConnectionManager {
    fn default() -> Self {
        Self::new()
    }
}
