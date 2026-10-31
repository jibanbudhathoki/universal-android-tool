use uat_device_engine::{DeviceDiscovery, RawUsbDescriptor};
use uat_device_registry::DeviceRegistry;
use uat_firmware_manager::{CompatibilityChecker, FirmwareCatalog};
use uat_recovery_workflows::{FrpRecoveryWorkflow, OwnershipVerification, ScreenLockRecoveryWorkflow};

#[test]
fn test_manufacturer_registry_resolution() {
    let registry = DeviceRegistry::new();
    let identification = registry.identify(1204, 26720); // Samsung S23 VID:PID
    assert_eq!(identification.manufacturer, "Samsung");
    assert!(identification.is_known_product);
}

#[test]
fn test_all_11_manufacturers_registered() {
    let registry = DeviceRegistry::new();
    let required_brands = [
        "Samsung",
        "Google Pixel",
        "Xiaomi",
        "Redmi",
        "POCO",
        "OPPO",
        "Vivo",
        "Realme",
        "Motorola",
        "OnePlus",
        "Nokia",
    ];

    let all_profiles = registry.list_supported_manufacturers();
    for brand in &required_brands {
        let found = all_profiles.iter().any(|p| p.name == *brand);
        assert!(found, "Required brand '{}' must be in device registry", brand);
    }
}

#[test]
fn test_chipset_diagnostics_recognition() {
    let registry = DeviceRegistry::new();
    // Qualcomm 9008 EDL
    let qcom = registry.identify(1478, 36872);
    assert_eq!(qcom.manufacturer, "Qualcomm Diagnostic");
    assert!(qcom.is_known_product);
    assert!(qcom.detected_mode.contains("9008"));

    // MediaTek BROM
    let mtk = registry.identify(3725, 3);
    assert_eq!(mtk.manufacturer, "MediaTek Diagnostic");
    assert!(mtk.is_known_product);
    assert!(mtk.detected_mode.contains("BROM"));
}

#[test]
fn test_strict_zero_false_claims_on_unknown_product() {
    let registry = DeviceRegistry::new();
    let unk = registry.identify(1256, 0xbeef);
    assert_eq!(unk.manufacturer, "Samsung");
    assert!(!unk.is_known_product);
    assert!(unk.model_name.contains("Unknown Samsung Model (0xbeef)"));
    assert!(unk.official_support_portal.is_some());
}

#[test]
fn test_device_discovery_descriptor() {
    let discovery = DeviceDiscovery::new();
    let descriptor = RawUsbDescriptor::simple(
        6353, // Google Pixel VID
        20042,
        Some("Google".to_string()),
        Some("Pixel 8".to_string()),
        None,
    );

    let device = discovery.inspect_descriptor(descriptor).expect("Failed to inspect descriptor");
    assert_eq!(device.manufacturer, "Google");
    assert_eq!(device.model, "Pixel 8");
    assert_eq!(device.vendor_id, 6353);
}

#[test]
fn test_recovery_workflows_official_guidance() {
    let discovery = DeviceDiscovery::new();
    let descriptor = RawUsbDescriptor::simple(
        1204,
        26720,
        Some("Samsung".to_string()),
        Some("Galaxy S23".to_string()),
        None,
    );

    let device = discovery.inspect_descriptor(descriptor).unwrap();
    let frp_guide = FrpRecoveryWorkflow::get_official_guidance(&device);
    assert_eq!(frp_guide.category, "frp_recovery");
    assert!(frp_guide.legal_notice.contains("does NOT circumvent FRP"));

    let screen_guide = ScreenLockRecoveryWorkflow::get_official_guidance(&device);
    assert_eq!(screen_guide.category, "screen_lock");
    assert!(screen_guide.legal_notice.contains("All local data will be erased"));
}

#[test]
fn test_ownership_verification_authorization() {
    let unverified = OwnershipVerification {
        is_verified_owner: false,
        proof_of_purchase_confirmed: false,
        authorized_technician_id: None,
        disclaimer_accepted: false,
        ..Default::default()
    };
    assert!(!unverified.is_authorized_for_recovery());

    let verified = OwnershipVerification {
        is_verified_owner: true,
        proof_of_purchase_confirmed: true,
        authorized_technician_id: Some("TECH-123".to_string()),
        disclaimer_accepted: true,
        ..Default::default()
    };
    assert!(verified.is_authorized_for_recovery());
}

#[test]
fn test_firmware_compatibility() {
    let catalog = FirmwareCatalog::new();
    let pixel_firmware = catalog.find_by_model("Pixel 8");
    assert!(!pixel_firmware.is_empty());

    let discovery = DeviceDiscovery::new();
    let device = discovery
        .inspect_descriptor(RawUsbDescriptor::simple(
            6353,
            20042,
            Some("Google Pixel".to_string()),
            Some("Pixel 8".to_string()),
            None,
        ))
        .unwrap();

    let compat = CompatibilityChecker::check(&device, pixel_firmware[0]);
    assert!(compat.is_compatible);
    assert!(compat.match_confidence >= 90);
}

#[tokio::test]
async fn test_adb_packet_framing_and_diagnostics() {
    use uat_protocol_adapters::adb::packet::{AdbMessageHeader, A_CNXN};
    use uat_protocol_adapters::{AdbAdapter, AdbAuthStatus, AdbDiagnosticRunner};

    let mut adapter = AdbAdapter::new();
    let (header, _payload) = adapter.create_handshake_packet();
    assert_eq!(header.command, A_CNXN);

    // Verify wire byte serialization
    let bytes = header.to_bytes();
    let parsed_header = AdbMessageHeader::from_bytes(&bytes).unwrap();
    assert_eq!(parsed_header.command, A_CNXN);

    // Simulate device authorization response
    let banner = b"device::ro.product.model=Pixel8;ro.product.manufacturer=Google\0";
    let resp_hdr = AdbMessageHeader::new(A_CNXN, 0x01000000, 256 * 1024, banner);
    let auth = adapter.process_handshake_response(&resp_hdr, banner).unwrap();
    assert!(matches!(auth, AdbAuthStatus::Authorized { .. }));
    assert!(adapter.is_authorized());

    // Generate getprop diagnostic report
    let raw_props = r#"
[ro.product.manufacturer]: [Google]
[ro.product.model]: [Pixel 8]
[ro.build.version.release]: [14]
[ro.boot.verifiedbootstate]: [green]
[ro.crypto.state]: [encrypted]
[ro.frp.pst]: [/dev/block/by-name/frp]
"#;
    let report = adapter.parse_diagnostic_properties(raw_props);
    assert_eq!(report.manufacturer.as_deref(), Some("Google"));
    assert_eq!(report.model.as_deref(), Some("Pixel 8"));
    assert_eq!(report.android_version.as_deref(), Some("14"));
    assert!(report.frp_partition_present);

    // Security check: non-diagnostic command blocked
    assert!(AdbDiagnosticRunner::validate_safe_command("rm -rf /system").is_err());
}

#[tokio::test]
async fn test_fastboot_command_security_and_diagnostics() {
    use uat_protocol_adapters::{FastbootAdapter, FastbootDiagnosticRunner, FastbootResponse};

    let adapter = FastbootAdapter::new();

    // Whitelisted command
    let cmd = adapter.format_command("getvar:all").unwrap();
    assert_eq!(cmd, b"getvar:all");

    // Destructive command blocked by security policy
    let blocked = adapter.format_command("oem unlock");
    assert!(blocked.is_err());

    // Response decoding
    let resp = adapter.parse_response(b"INFOunlocked: no\n").unwrap();
    assert_eq!(resp, FastbootResponse::Info("unlocked: no".to_string()));

    // Diagnostics report generation from multi-line INFO lines
    let info_lines = vec![
        "product: husky".to_string(),
        "unlocked: no".to_string(),
        "secure: yes".to_string(),
        "version-bootloader: 1.0-test".to_string(),
    ];
    let report = adapter.collect_diagnostic_variables(&info_lines);
    assert_eq!(report.product.as_deref(), Some("husky"));
    assert!(!report.is_unlocked);
    assert!(report.is_secure_boot);

    assert!(FastbootDiagnosticRunner::validate_safe_command("erase userdata").is_err());
}

#[tokio::test]
async fn test_device_engine_comprehensive_diagnostics() {
    use uat_device_engine::DeviceEngine;

    let engine = DeviceEngine::new();
    let descriptor = RawUsbDescriptor::simple(
        6353,
        20042,
        Some("Google".to_string()),
        Some("Pixel 8".to_string()),
        None,
    );

    let device = engine.discover_and_connect(descriptor).await.unwrap();
    let diag = engine.run_comprehensive_diagnostics(&device.id).await.unwrap();

    assert_eq!(diag.device_id, device.id);
    assert!(diag.security_status.contains("User Authorization"));
}

#[test]
fn test_tailored_frp_guidance_for_oems() {
    let discovery = DeviceDiscovery::new();

    // Samsung device
    let sam_dev = discovery
        .inspect_descriptor(RawUsbDescriptor::simple(
            1204,
            26720,
            Some("Samsung".to_string()),
            Some("Galaxy S23".to_string()),
            None,
        ))
        .unwrap();
    let sam_guide = FrpRecoveryWorkflow::get_official_guidance(&sam_dev);
    assert!(sam_guide.title.contains("Samsung"));
    assert!(sam_guide.official_support_url.contains("samsung.com"));

    // Xiaomi device
    let mi_dev = discovery
        .inspect_descriptor(RawUsbDescriptor::simple(
            10007,
            65352,
            Some("Xiaomi".to_string()),
            Some("Redmi Note 12".to_string()),
            None,
        ))
        .unwrap();
    let mi_guide = FrpRecoveryWorkflow::get_official_guidance(&mi_dev);
    assert!(mi_guide.title.contains("Xiaomi"));
    assert!(mi_guide.official_steps.iter().any(|s| s.contains("i.mi.com")));
}

#[test]
fn test_ownership_eligibility_checkpoint() {
    let mut verification = OwnershipVerification::default();

    // 1. None checked -> not authorized
    let check1 = verification.verify_eligibility();
    assert!(!check1.is_authorized);
    assert_eq!(check1.missing_requirements.len(), 3);

    // 2. Owner identity confirmed
    verification.is_verified_owner = true;
    let check2 = verification.verify_eligibility();
    assert!(!check2.is_authorized);
    assert_eq!(check2.missing_requirements.len(), 2);

    // 3. Proof of purchase confirmed
    verification.proof_of_purchase_confirmed = true;
    let check3 = verification.verify_eligibility();
    assert!(!check3.is_authorized);
    assert_eq!(check3.missing_requirements.len(), 1);

    // 4. Disclaimer accepted -> fully authorized!
    verification.disclaimer_accepted = true;
    let check4 = verification.verify_eligibility();
    assert!(check4.is_authorized);
    assert!(check4.missing_requirements.is_empty());
}

#[test]
fn test_audit_record_tamper_evident_hashing() {
    use uat_recovery_workflows::RecoveryAuditLogger;

    let logger = RecoveryAuditLogger::new();
    let verification = OwnershipVerification {
        is_verified_owner: true,
        proof_of_purchase_confirmed: true,
        authorized_technician_id: Some("TECH-CERT-99".to_string()),
        disclaimer_accepted: true,
        device_imei_or_serial: Some("359281092817263".to_string()),
        customer_name: Some("Robert King".to_string()),
        notes: Some("Customer verified with government driver license".to_string()),
    };

    let record = logger.record_workflow(uat_recovery_workflows::CreateAuditParams {
        device_id: "dev-samsung-s23".to_string(),
        manufacturer: "Samsung".to_string(),
        model: "Galaxy S23".to_string(),
        serial_number: Some("359281092817263".to_string()),
        workflow_type: "frp_recovery_guidance".to_string(),
        verification,
        disclaimer_text: "Universal Android Tool Zero-Bypass Compliance Agreement".to_string(),
    });

    // Pristine record passes cryptographic hash verification
    assert!(record.verify_integrity());

    // Tampered record fails integrity verification
    let mut tampered = record.clone();
    tampered.workflow_type = "unauthorized_bypass_attempt".to_string();
    assert!(!tampered.verify_integrity());
}

#[test]
fn test_firmware_catalog_search_and_metadata() {
    let catalog = FirmwareCatalog::new();

    // Query by marketing model
    let pixel_results = catalog.find_by_model("Pixel 8 Pro");
    assert!(!pixel_results.is_empty());
    let fw = pixel_results[0];
    assert_eq!(fw.model, "Pixel 8 Pro");

    // Query by hardware codename alias
    let husky_results = catalog.find_by_model("husky");
    assert_eq!(husky_results.len(), 1);
    assert_eq!(husky_results[0].id, fw.id);

    // Retrieve ARB index from metadata
    let meta = catalog.get_metadata(&fw.id).expect("Metadata missing for Pixel 8 Pro");
    assert_eq!(meta.anti_rollback_index, 1);
    assert!(meta.chipset.as_deref().unwrap().contains("Tensor"));
}

#[test]
fn test_firmware_checksum_integrity_verification() {
    use uat_firmware_manager::ChecksumVerifier;

    let payload = b"Universal Android Tool Certified Stock Image 2024";
    let sha256_hash = ChecksumVerifier::compute_sha256(payload);

    // Exact checksum matches
    let res = ChecksumVerifier::verify_bytes(payload, &format!("sha256:{}", sha256_hash));
    assert!(res.matches);
    assert!(res.status_message.contains("Cryptographic integrity verified"));

    // Corrupted payload fails
    let corrupted = b"Altered boot.img bytes";
    let fail_res = ChecksumVerifier::verify_bytes(corrupted, &sha256_hash);
    assert!(!fail_res.matches);
    assert!(fail_res.status_message.contains("CHECKSUM MISMATCH"));
}

#[test]
fn test_firmware_anti_rollback_protection() {
    use uat_firmware_manager::{CompatibilityChecker, FirmwareCatalog};

    let catalog = FirmwareCatalog::new();
    let fw_s23 = catalog
        .find_by_id("fw-galaxy-s23-bwk4")
        .expect("S23 firmware missing");
    let meta_s23 = catalog.get_metadata(&fw_s23.id).expect("S23 metadata missing");

    let device = DeviceDiscovery::new()
        .inspect_descriptor(RawUsbDescriptor::simple(
            1204,
            26720,
            Some("Samsung".to_string()),
            Some("Galaxy S23 (SM-S911B)".to_string()),
            None,
        ))
        .unwrap();

    // Current device ARB = 3, firmware ARB = 3 -> Safe
    let safe_check = CompatibilityChecker::check_with_metadata(&device, fw_s23, Some(meta_s23), Some(3));
    assert!(safe_check.is_compatible);
    assert!(safe_check.anti_rollback_safe);

    // Current device updated to ARB = 4, trying to flash ARB = 3 -> Blocked to prevent hard-brick!
    let brick_prevent_check = CompatibilityChecker::check_with_metadata(&device, fw_s23, Some(meta_s23), Some(4));
    assert!(!brick_prevent_check.is_compatible);
    assert!(!brick_prevent_check.anti_rollback_safe);
    assert!(brick_prevent_check.warnings.iter().any(|w| w.contains("ANTI-ROLLBACK")));
}



