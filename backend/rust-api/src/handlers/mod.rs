use crate::errors::ApiError;
use crate::services::AppState;
use axum::{
    extract::{Path, Query, State},
    response::Json,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use uat_device_engine::{CapabilityDetector, RawUsbDescriptor};
use uat_firmware_manager::{ChecksumVerifier, CompatibilityChecker};
use uat_recovery_workflows::{
    CreateAuditParams, FrpRecoveryWorkflow, OwnershipVerification, RecoveryAuditRecord,
    ScreenLockRecoveryWorkflow, VerificationResult,
};
use uat_shared_types::{
    ComprehensiveDiagnosticResult, DeviceInfo, FirmwareCatalogItem, RecoveryGuide,
};

#[derive(Debug, Deserialize)]
pub struct ConnectDevicePayload {
    pub vendor_id: u16,
    pub product_id: u16,
    pub manufacturer: Option<String>,
    pub model: Option<String>,
    pub serial_number: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct FirmwareQueryParams {
    pub model: Option<String>,
    pub manufacturer: Option<String>,
    pub region: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct VerifyCompatibilityPayload {
    pub device_id: String,
    pub firmware_id: String,
    pub current_device_arb: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct VerifyChecksumPayload {
    pub raw_string: Option<String>,
    pub expected_checksum: String,
}

#[derive(Debug, Serialize)]
pub struct CompatibilityResponse {
    pub is_compatible: bool,
    pub match_confidence: u8,
    pub model_match: bool,
    pub region_compatible: bool,
    pub anti_rollback_safe: bool,
    pub notes: Vec<String>,
    pub warnings: Vec<String>,
}

pub async fn health_check() -> Json<Value> {
    Json(json!({
        "status": "healthy",
        "service": "universal-android-tool-api",
        "version": env!("CARGO_PKG_VERSION"),
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}

pub async fn list_devices(State(state): State<AppState>) -> Json<Vec<DeviceInfo>> {
    let devices = state.device_engine.connection_manager.list_devices().await;
    Json(devices)
}

pub async fn connect_device(
    State(state): State<AppState>,
    Json(payload): Json<ConnectDevicePayload>,
) -> Result<Json<DeviceInfo>, ApiError> {
    let descriptor = RawUsbDescriptor::simple(
        payload.vendor_id,
        payload.product_id,
        payload.manufacturer,
        payload.model,
        payload.serial_number,
    );

    let device = state
        .device_engine
        .discover_and_connect(descriptor)
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;

    Ok(Json(device))
}

pub async fn get_device(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<DeviceInfo>, ApiError> {
    let device = state
        .device_engine
        .connection_manager
        .get_device(&id)
        .await
        .ok_or_else(|| ApiError::NotFound(format!("Device {} not found", id)))?;

    Ok(Json(device))
}

pub async fn get_device_capabilities(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Value>, ApiError> {
    let device = state
        .device_engine
        .connection_manager
        .get_device(&id)
        .await
        .ok_or_else(|| ApiError::NotFound(format!("Device {} not found", id)))?;

    let capabilities = CapabilityDetector::evaluate(&device);
    Ok(Json(json!({
        "device_id": id,
        "capabilities": capabilities
    })))
}

pub async fn run_device_diagnostics(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ComprehensiveDiagnosticResult>, ApiError> {
    let diag = state
        .device_engine
        .run_comprehensive_diagnostics(&id)
        .await
        .map_err(|e| ApiError::NotFound(e.to_string()))?;

    Ok(Json(diag))
}

pub async fn list_operations(State(state): State<AppState>) -> Json<Value> {
    let devices = state.device_engine.connection_manager.list_devices().await;
    Json(json!({
        "active_devices": devices.len(),
        "status": "idle"
    }))
}

pub async fn get_operation(Path(id): Path<String>) -> Result<Json<Value>, ApiError> {
    Err(ApiError::NotFound(format!("Operation {} not found", id)))
}

pub async fn get_firmware_catalog(
    State(state): State<AppState>,
    Query(params): Query<FirmwareQueryParams>,
) -> Json<Vec<FirmwareCatalogItem>> {
    if let Some(ref model) = params.model {
        let items = state.firmware_catalog.find_by_model(model);
        return Json(items.into_iter().cloned().collect());
    }
    if let Some(ref mfg) = params.manufacturer {
        let items = state.firmware_catalog.find_by_manufacturer(mfg);
        return Json(items.into_iter().cloned().collect());
    }
    if let Some(ref region) = params.region {
        let items = state.firmware_catalog.find_by_region(region);
        return Json(items.into_iter().cloned().collect());
    }

    let all_items = state.firmware_catalog.list().to_vec();
    Json(all_items)
}

pub async fn verify_firmware_compatibility(
    State(state): State<AppState>,
    Json(payload): Json<VerifyCompatibilityPayload>,
) -> Result<Json<CompatibilityResponse>, ApiError> {
    let device = state
        .device_engine
        .connection_manager
        .get_device(&payload.device_id)
        .await
        .ok_or_else(|| ApiError::NotFound(format!("Device {} not found", payload.device_id)))?;

    let firmware = state
        .firmware_catalog
        .find_by_id(&payload.firmware_id)
        .ok_or_else(|| ApiError::NotFound(format!("Firmware {} not found", payload.firmware_id)))?;

    let metadata = state.firmware_catalog.get_metadata(&firmware.id);

    let res = CompatibilityChecker::check_with_metadata(
        &device,
        firmware,
        metadata,
        payload.current_device_arb,
    );

    Ok(Json(CompatibilityResponse {
        is_compatible: res.is_compatible,
        match_confidence: res.match_confidence,
        model_match: res.model_match,
        region_compatible: res.region_compatible,
        anti_rollback_safe: res.anti_rollback_safe,
        notes: res.notes,
        warnings: res.warnings,
    }))
}

pub async fn verify_firmware_checksum(
    Json(payload): Json<VerifyChecksumPayload>,
) -> Json<Value> {
    let bytes = payload.raw_string.unwrap_or_default().into_bytes();
    let res = ChecksumVerifier::verify_bytes(&bytes, &payload.expected_checksum);
    Json(json!({
        "matches": res.matches,
        "expected_checksum": res.expected_checksum,
        "calculated_checksum": res.calculated_checksum,
        "status_message": res.status_message
    }))
}

pub async fn get_recovery_guides(State(state): State<AppState>) -> Json<Vec<RecoveryGuide>> {
    let devices = state.device_engine.connection_manager.list_devices().await;
    let mut guides = Vec::new();

    if let Some(dev) = devices.first() {
        guides.push(FrpRecoveryWorkflow::get_official_guidance(dev));
        guides.push(ScreenLockRecoveryWorkflow::get_official_guidance(dev));
    } else {
        // Default generic recovery guidance
        let generic_dev = DeviceInfo {
            id: "generic".to_string(),
            vendor_id: 0,
            product_id: 0,
            manufacturer: "Android".to_string(),
            model: "Generic Device".to_string(),
            android_version: None,
            build_number: None,
            serial_number: None,
            connection_mode: uat_shared_types::ConnectionMode::Unknown,
            supported_protocols: vec![],
            capabilities: Default::default(),
            is_authorized: false,
        };
        guides.push(FrpRecoveryWorkflow::get_official_guidance(&generic_dev));
        guides.push(ScreenLockRecoveryWorkflow::get_official_guidance(&generic_dev));
    }

    Json(guides)
}

pub async fn verify_ownership(
    Json(verification): Json<OwnershipVerification>,
) -> Json<VerificationResult> {
    let res = verification.verify_eligibility();
    Json(res)
}

pub async fn create_recovery_audit(
    State(state): State<AppState>,
    Json(params): Json<CreateAuditParams>,
) -> Json<RecoveryAuditRecord> {
    let record = state.audit_logger.record_workflow(params);
    Json(record)
}
