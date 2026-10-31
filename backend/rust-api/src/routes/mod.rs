use crate::handlers;
use crate::middleware::cors_layer;
use crate::services::AppState;
use axum::{
    routing::{get, post},
    Router,
};

pub fn app_router(state: AppState) -> Router {
    Router::new()
        // Health & Diagnostics
        .route("/api/health", get(handlers::health_check))
        // Devices
        .route("/api/devices", get(handlers::list_devices))
        .route("/api/devices/connect", post(handlers::connect_device))
        .route("/api/devices/:id", get(handlers::get_device))
        .route(
            "/api/devices/:id/capabilities",
            get(handlers::get_device_capabilities),
        )
        .route(
            "/api/devices/:id/diagnostics",
            post(handlers::run_device_diagnostics),
        )
        // Operations
        .route("/api/operations", get(handlers::list_operations))
        .route("/api/operations/:id", get(handlers::get_operation))
        // Firmware
        .route("/api/firmware/catalog", get(handlers::get_firmware_catalog))
        .route(
            "/api/firmware/verify-compatibility",
            post(handlers::verify_firmware_compatibility),
        )
        .route(
            "/api/firmware/verify-checksum",
            post(handlers::verify_firmware_checksum),
        )
        // Recovery & Audit
        .route("/api/recovery/guides", get(handlers::get_recovery_guides))
        .route(
            "/api/recovery/verify-ownership",
            post(handlers::verify_ownership),
        )
        .route("/api/recovery/audit", post(handlers::create_recovery_audit))
        .layer(cors_layer())
        .with_state(state)
}

