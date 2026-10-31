use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ConnectionMode {
    WebUsb,
    WebSerial,
    NativeUsb,
    Adb,
    Fastboot,
    Mtp,
    Disconnected,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SupportedProtocol {
    Adb,
    Fastboot,
    Mtp,
    UsbMassStorage,
    VendorSpecific(String),
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeviceCapabilities {
    pub adb_available: bool,
    pub fastboot_available: bool,
    pub mtp_available: bool,
    pub diagnostic_mode_supported: bool,
    pub recovery_mode_supported: bool,
    pub oem_lock_status_readable: bool,
    pub authorized_reset_supported: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceInfo {
    pub id: String,
    pub vendor_id: u16,
    pub product_id: u16,
    pub manufacturer: String,
    pub model: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub android_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
    pub connection_mode: ConnectionMode,
    pub supported_protocols: Vec<SupportedProtocol>,
    pub capabilities: DeviceCapabilities,
    pub is_authorized: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OperationStatus {
    Idle,
    Queued,
    Running,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationRecord {
    pub id: Uuid,
    pub device_id: String,
    pub operation_type: String,
    pub status: OperationStatus,
    pub started_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<DateTime<Utc>>,
    pub progress_percent: u8,
    pub logs: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FirmwareCatalogItem {
    pub id: String,
    pub manufacturer: String,
    pub model: String,
    pub build_id: String,
    pub android_version: String,
    pub release_date: String,
    pub region: String,
    pub verification_checksum: String,
    pub download_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecoveryGuide {
    pub id: String,
    pub manufacturer: String,
    pub title: String,
    pub category: String, // "frp_recovery" or "screen_lock"
    pub legal_notice: String,
    pub prerequisites: Vec<String>,
    pub official_steps: Vec<String>,
    pub official_support_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdbDiagnosticReport {
    pub is_authorized: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manufacturer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub android_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified_boot_state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crypto_state: Option<String>,
    pub frp_partition_present: bool,
    pub properties: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FastbootDiagnosticReport {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bootloader_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baseband_version: Option<String>,
    pub is_unlocked: bool,
    pub is_secure_boot: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_slot: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub battery_voltage: Option<String>,
    pub variables: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComprehensiveDiagnosticResult {
    pub device_id: String,
    pub timestamp: DateTime<Utc>,
    pub connection_mode: ConnectionMode,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adb_diagnostics: Option<AdbDiagnosticReport>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fastboot_diagnostics: Option<FastbootDiagnosticReport>,
    pub security_status: String,
}

