import { invoke } from '@tauri-apps/api/core';
import { listen, type UnlistenFn } from '@tauri-apps/api/event';
import type {
  ChecksumVerificationResult,
  CompatibilityResult,
  DeviceCapabilities,
  DeviceInfo,
  FirmwareCatalogItem,
  OperationRecord,
  OwnershipVerification,
  RecoveryAuditRecord,
  RecoveryGuide,
  VerificationResult,
} from '@uat/shared-types';

export interface SystemInfo {
  os: string;
  arch: string;
  driver_stack: string;
  desktop_version: string;
  security_standard: string;
}

export interface DiagnosticEvent {
  device_id: string;
  progress_percent: number;
  status: string;
  message: string;
  timestamp: string;
}

export const tauriBridge = {
  async listDevices(): Promise<DeviceInfo[]> {
    return invoke<DeviceInfo[]>('list_devices');
  },

  async getDeviceCapabilities(deviceId: string): Promise<DeviceCapabilities> {
    return invoke<DeviceCapabilities>('get_device_capabilities', { deviceId });
  },

  async connectNativeDevice(
    vendorId: number,
    productId: number,
    manufacturer?: string,
    model?: string,
    serialNumber?: string
  ): Promise<DeviceInfo> {
    return invoke<DeviceInfo>('connect_native_device', {
      vendorId,
      productId,
      manufacturer,
      model,
      serialNumber,
    });
  },

  async disconnectDevice(deviceId: string): Promise<boolean> {
    return invoke<boolean>('disconnect_device', { deviceId });
  },

  async runDiagnostics(deviceId: string): Promise<OperationRecord> {
    return invoke<OperationRecord>('run_diagnostics', { deviceId });
  },

  async getSystemInfo(): Promise<SystemInfo> {
    return invoke<SystemInfo>('get_system_info');
  },
  
  async getFirmwareCatalog(): Promise<FirmwareCatalogItem[]> {
    return invoke<FirmwareCatalogItem[]>('get_firmware_catalog');
  },

  async getRecoveryGuidance(manufacturer: string): Promise<RecoveryGuide[]> {
    return invoke<RecoveryGuide[]>('get_recovery_guidance', { manufacturer });
  },

  async verifyOwnership(verification: OwnershipVerification): Promise<VerificationResult> {
    return invoke<VerificationResult>('verify_ownership', { verification });
  },

  async verifyFirmwareCompatibility(
    deviceId: string,
    firmwareId: string,
    currentDeviceArb?: number
  ): Promise<CompatibilityResult> {
    return invoke<CompatibilityResult>('verify_firmware_compatibility', {
      deviceId,
      firmwareId,
      currentDeviceArb,
    });
  },

  async verifyFirmwareChecksum(
    expectedChecksum: string,
    rawString?: string
  ): Promise<ChecksumVerificationResult> {
    return invoke<ChecksumVerificationResult>('verify_firmware_checksum', {
      expectedChecksum,
      rawString,
    });
  },

  async recordRecoveryAudit(params: {
    device_id: string;
    manufacturer: string;
    model: string;
    workflow_type: string;
    technician_id: string;
    outcome: string;
    notes?: string;
  }): Promise<RecoveryAuditRecord> {
    return invoke<RecoveryAuditRecord>('record_recovery_audit', { params });
  },

  async onDiagnosticEvent(callback: (event: DiagnosticEvent) => void): Promise<UnlistenFn> {
    return listen<DiagnosticEvent>('diagnostic_event', (e) => {
      callback(e.payload);
    });
  },
};

