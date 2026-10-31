# Universal Android Tool (UAT) — API Reference

This document provides complete documentation for the **Axum REST API**, **Tauri 2 Desktop IPC Commands**, and **WebUSB Service Interface**.

---

## 1. Axum REST API Daemon (`backend/rust-api`)

Default Base URL: `http://127.0.0.1:8080`

### 1.1 Health & Diagnostics

#### `GET /api/health`
Returns the status, version, and server timestamp.
- **Response**: `200 OK`
```json
{
  "status": "healthy",
  "service": "universal-android-tool-api",
  "version": "0.1.0",
  "timestamp": "2026-10-31T06:00:00Z"
}
```

---

### 1.2 Devices

#### `GET /api/devices`
Returns all currently connected and registered Android devices.
- **Response**: `200 OK`
```json
[
  {
    "id": "usb-04e8:6860",
    "vendor_id": 1256,
    "product_id": 26720,
    "manufacturer": "Samsung",
    "model": "Galaxy S23 (SM-S911B)",
    "android_version": "14.0",
    "build_number": "S911BXXU3BWK4",
    "serial_number": null,
    "connection_mode": "nativeusb",
    "supported_protocols": ["adb", "mtp"],
    "capabilities": {
      "adb_available": true,
      "fastboot_available": false,
      "mtp_available": true,
      "diagnostic_mode_supported": true,
      "recovery_mode_supported": false,
      "oem_lock_status_readable": false,
      "authorized_reset_supported": true
    },
    "is_authorized": true
  }
]
```

#### `POST /api/devices/connect`
Manually register or probe a connected USB descriptor.
- **Request Body**:
```json
{
  "vendor_id": 6353,
  "product_id": 20042,
  "manufacturer": "Google Pixel",
  "model": "Pixel 8 (shiba)",
  "serial_number": null
}
```
- **Response**: `200 OK` (returns full `DeviceInfo` struct)

#### `GET /api/devices/:id`
Retrieves information for a specific registered device by ID.
- **Response**: `200 OK` or `404 Not Found`

#### `GET /api/devices/:id/capabilities`
Evaluates and returns physical hardware capabilities for the target device.
- **Response**: `200 OK`
```json
{
  "device_id": "usb-18d1:4ee2",
  "capabilities": {
    "adb_available": true,
    "fastboot_available": false,
    "mtp_available": true,
    "diagnostic_mode_supported": true,
    "recovery_mode_supported": false,
    "oem_lock_status_readable": false,
    "authorized_reset_supported": true
  }
}
```

#### `POST /api/devices/:id/diagnostics`
Executes comprehensive, non-destructive ADB and Fastboot diagnostic checks.
- **Response**: `200 OK`
```json
{
  "device_id": "usb-18d1:4ee2",
  "timestamp": "2026-10-31T06:01:00Z",
  "connection_mode": "nativeusb",
  "adb_diagnostics": {
    "is_authorized": true,
    "manufacturer": "Google Pixel",
    "model": "Pixel 8",
    "android_version": "14.0",
    "build_id": "UP1A.231005.007",
    "verified_boot_state": "green",
    "crypto_state": "encrypted",
    "frp_partition_present": true,
    "properties": {
      "ro.product.model": "Pixel 8",
      "ro.build.version.release": "14"
    }
  },
  "fastboot_diagnostics": null,
  "security_status": "Verified (Official Hardware Profile)"
}
```

---

### 1.3 Firmware Management

#### `GET /api/firmware/catalog`
Query the official stock firmware catalog.
- **Query Parameters**:
  - `model` (optional string): e.g. `SM-S911B`, `shiba`, `Pixel 8`
  - `manufacturer` (optional string): e.g. `Samsung`, `Google Pixel`
  - `region` (optional string): e.g. `EUX`, `Global`
- **Response**: `200 OK` (list of `FirmwareCatalogItem`)

#### `POST /api/firmware/verify-compatibility`
Validates hardware compatibility and anti-rollback safety between a device and firmware image.
- **Request Body**:
```json
{
  "device_id": "usb-04e8:6860",
  "firmware_id": "fw-galaxy-s23-bwk4",
  "current_device_arb": 3
}
```
- **Response**: `200 OK`
```json
{
  "is_compatible": true,
  "match_confidence": 100,
  "model_match": true,
  "region_compatible": true,
  "anti_rollback_safe": true,
  "notes": ["Hardware profile exact match", "Anti-rollback safe (Equal ARB version: 3)"],
  "warnings": []
}
```

#### `POST /api/firmware/verify-checksum`
Validates cryptographic SHA-256 binary hash.
- **Request Body**:
```json
{
  "raw_string": "firmware-payload-binary-bytes",
  "expected_checksum": "sha256:e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
}
```
- **Response**: `200 OK`

---

### 1.4 Recovery & Verification

#### `GET /api/recovery/guides`
Returns official manufacturer recovery procedures and portal URLs for active devices.

#### `POST /api/recovery/verify-ownership`
Performs verification check against proof-of-purchase and technician credentials.
- **Request Body**:
```json
{
  "device_serial": "R5CW10ABCDE",
  "proof_of_purchase_id": "INV-2026-9812",
  "technician_id": "TECH-0481",
  "customer_id_type": "Driver License",
  "reason_for_service": "Customer forgotten PIN",
  "consent_acknowledged": true
}
```
- **Response**: `200 OK`
```json
{
  "is_eligible": true,
  "compliance_status": "Ownership Verified & Authorized",
  "timestamp": "2026-10-31T06:02:00Z",
  "failure_reasons": []
}
```

#### `POST /api/recovery/audit`
Creates an immutable, cryptographically hash-chained audit record.
- **Request Body**:
```json
{
  "device_id": "usb-04e8:6860",
  "manufacturer": "Samsung",
  "model": "Galaxy S23",
  "workflow_type": "FRP Recovery Guidance",
  "technician_id": "TECH-0481",
  "outcome": "Guidance Provided & Verified",
  "notes": "Customer presented matching purchase receipt"
}
```
- **Response**: `200 OK` (returns full `RecoveryAuditRecord` including SHA-256 hash chaining)

---

## 2. Tauri 2 Desktop IPC API (`apps/desktop`)

Desktop frontend invokes native Rust commands via `@tauri-apps/api/core::invoke`.

| Command | Arguments | Return Type | Description |
|---|---|---|---|
| `list_devices` | none | `DeviceInfo[]` | Lists all detected native USB devices |
| `get_device_capabilities` | `{ deviceId: string }` | `DeviceCapabilities` | Evaluates physical interface protocols |
| `connect_native_device` | `{ vendorId, productId, ... }` | `DeviceInfo` | Manually attaches a device descriptor |
| `disconnect_device` | `{ deviceId: string }` | `boolean` | Unregisters a device from the session |
| `run_diagnostics` | `{ deviceId: string }` | `OperationRecord` | Executes progressive diagnostic probe |
| `get_system_info` | none | `SystemInfo` | Returns host OS, arch, and USB driver stack |
| `get_firmware_catalog` | none | `FirmwareCatalogItem[]` | Retrieves full stock firmware catalog |
| `get_recovery_guidance` | `{ manufacturer: string }` | `RecoveryGuide[]` | Returns tailored OEM recovery guides |
| `verify_ownership` | `{ verification: OwnershipVerification }` | `VerificationResult` | Validates service eligibility |
| `verify_firmware_compatibility` | `{ deviceId, firmwareId, currentDeviceArb? }` | `CompatibilityResult` | Checks model, region, and ARB safety |
| `verify_firmware_checksum` | `{ expectedChecksum, rawString? }` | `ChecksumVerificationResult` | Verifies SHA-256 digest |
| `record_recovery_audit` | `{ params: CreateAuditParams }` | `RecoveryAuditRecord` | Appends hash-chained audit record |

### Desktop Event Emitter:
- **`diagnostic_event`**: Emitted during diagnostic execution with `{ device_id, progress_percent, status, message, timestamp }`.

---

## 3. WebUSB Service Interface (`apps/web/src/services/webusb.ts`)

| Method | Parameters | Return Type | Description |
|---|---|---|---|
| `isSupported()` | none | `boolean` | Checks if `navigator.usb` is supported in browser |
| `getPairedDevices()` | `allowSerialNumber?: boolean` | `Promise<DeviceInfo[]>` | Enumerates previously authorized USB devices |
| `requestDevice()` | `allowSerialNumber?: boolean` | `Promise<DeviceInfo>` | Triggers user gesture permission prompt |
| `openDevice()` | `deviceId: string` | `Promise<boolean>` | Opens USB session to claim endpoints |
| `closeDevice()` | `deviceId: string` | `Promise<void>` | Closes active USB session |
| `mapUsbDeviceToDeviceInfo()` | `device: USBDevice` | `DeviceInfo` | Parses USB configs & interface classes |
