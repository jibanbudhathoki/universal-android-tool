export type ConnectionMode =
  | 'webusb'
  | 'webserial'
  | 'nativeusb'
  | 'adb'
  | 'fastboot'
  | 'mtp'
  | 'disconnected'
  | 'unknown';

export type SupportedProtocol =
  | 'adb'
  | 'fastboot'
  | 'mtp'
  | 'usbmassstorage'
  | { vendor_specific: string };

export interface DeviceCapabilities {
  adb_available: boolean;
  fastboot_available: boolean;
  mtp_available: boolean;
  diagnostic_mode_supported: boolean;
  recovery_mode_supported: boolean;
  oem_lock_status_readable: boolean;
  authorized_reset_supported: boolean;
}

export interface DeviceInfo {
  id: string;
  vendor_id: number;
  product_id: number;
  manufacturer: string;
  model: string;
  android_version?: string;
  build_number?: string;
  serial_number?: string;
  connection_mode: ConnectionMode;
  supported_protocols: SupportedProtocol[];
  capabilities: DeviceCapabilities;
  is_authorized: boolean;
}

export type OperationStatus =
  | 'idle'
  | 'queued'
  | 'running'
  | 'completed'
  | 'failed'
  | 'cancelled';

export interface OperationRecord {
  id: string;
  device_id: string;
  operation_type: string;
  status: OperationStatus;
  started_at: string;
  completed_at?: string;
  progress_percent: number;
  logs: string[];
  error_message?: string;
}

export interface FirmwareCatalogItem {
  id: string;
  manufacturer: string;
  model: string;
  build_id: string;
  android_version: string;
  release_date: string;
  region: string;
  verification_checksum: string;
  download_url?: string;
}

export interface RecoveryGuide {
  id: string;
  manufacturer: string;
  title: string;
  category: 'frp_recovery' | 'screen_lock';
  legal_notice: string;
  prerequisites: string[];
  official_steps: string[];
  official_support_url: string;
}

export interface ApiResponse<T> {
  success: boolean;
  data?: T;
  error?: string;
  timestamp: string;
}

export interface AdbDiagnosticReport {
  is_authorized: boolean;
  manufacturer?: string;
  model?: string;
  android_version?: string;
  build_id?: string;
  verified_boot_state?: string;
  crypto_state?: string;
  frp_partition_present: boolean;
  properties: Record<string, string>;
}

export interface FastbootDiagnosticReport {
  product?: string;
  serial_number?: string;
  bootloader_version?: string;
  baseband_version?: string;
  is_unlocked: boolean;
  is_secure_boot: boolean;
  current_slot?: string;
  battery_voltage?: string;
  variables: Record<string, string>;
}

export interface ComprehensiveDiagnosticResult {
  device_id: string;
  timestamp: string;
  connection_mode: ConnectionMode;
  adb_diagnostics?: AdbDiagnosticReport;
  fastboot_diagnostics?: FastbootDiagnosticReport;
  security_status: string;
}

export interface OwnershipVerification {
  device_serial: string;
  proof_of_purchase_id?: string;
  technician_id: string;
  customer_id_type?: string;
  reason_for_service: string;
  consent_acknowledged: boolean;
}

export interface VerificationResult {
  is_eligible: boolean;
  compliance_status: string;
  timestamp: string;
  failure_reasons: string[];
}

export interface CompatibilityResult {
  is_compatible: boolean;
  match_confidence: number;
  model_match: boolean;
  region_compatible: boolean;
  anti_rollback_safe: boolean;
  notes: string[];
  warnings: string[];
}

export interface ChecksumVerificationResult {
  matches: boolean;
  expected_checksum: string;
  calculated_checksum: string;
  status_message: string;
}

export interface RecoveryAuditRecord {
  audit_id: string;
  device_id: string;
  manufacturer: string;
  model: string;
  workflow_type: string;
  technician_id: string;
  outcome: string;
  timestamp: string;
  previous_record_hash: string;
  record_hash: string;
}

