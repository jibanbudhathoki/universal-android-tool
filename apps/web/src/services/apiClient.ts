import type {
  ChecksumVerificationResult,
  CompatibilityResult,
  DeviceInfo,
  FirmwareCatalogItem,
  OwnershipVerification,
  RecoveryAuditRecord,
  RecoveryGuide,
  VerificationResult,
} from '@uat/shared-types';

export class ApiClient {
  private baseUrl: string;

  constructor(baseUrl = 'http://127.0.0.1:8080') {
    this.baseUrl = baseUrl;
  }

  public setBaseUrl(url: string) {
    this.baseUrl = url.replace(/\/$/, '');
  }

  public async getHealth(): Promise<{ status: string; service: string; version: string }> {
    const res = await fetch(`${this.baseUrl}/api/health`);
    if (!res.ok) throw new Error(`Health check failed: ${res.statusText}`);
    return res.json();
  }

  public async listDevices(): Promise<DeviceInfo[]> {
    const res = await fetch(`${this.baseUrl}/api/devices`);
    if (!res.ok) throw new Error(`Failed to list devices: ${res.statusText}`);
    return res.json();
  }

  public async connectDevice(payload: {
    vendor_id: number;
    product_id: number;
    manufacturer?: string;
    model?: string;
    serial_number?: string;
  }): Promise<DeviceInfo> {
    const res = await fetch(`${this.baseUrl}/api/devices/connect`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(payload),
    });
    if (!res.ok) throw new Error(`Failed to connect device: ${res.statusText}`);
    return res.json();
  }

  public async getFirmwareCatalog(params?: {
    model?: string;
    manufacturer?: string;
    region?: string;
  }): Promise<FirmwareCatalogItem[]> {
    const query = new URLSearchParams();
    if (params?.model) query.set('model', params.model);
    if (params?.manufacturer) query.set('manufacturer', params.manufacturer);
    if (params?.region) query.set('region', params.region);

    const url = `${this.baseUrl}/api/firmware/catalog${query.toString() ? `?${query.toString()}` : ''}`;
    const res = await fetch(url);
    if (!res.ok) throw new Error(`Failed to fetch firmware catalog: ${res.statusText}`);
    return res.json();
  }

  public async verifyFirmwareCompatibility(payload: {
    device_id: string;
    firmware_id: string;
    current_device_arb?: number;
  }): Promise<CompatibilityResult> {
    const res = await fetch(`${this.baseUrl}/api/firmware/verify-compatibility`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(payload),
    });
    if (!res.ok) throw new Error(`Failed to verify firmware compatibility: ${res.statusText}`);
    return res.json();
  }

  public async verifyFirmwareChecksum(payload: {
    raw_string?: string;
    expected_checksum: string;
  }): Promise<ChecksumVerificationResult> {
    const res = await fetch(`${this.baseUrl}/api/firmware/verify-checksum`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(payload),
    });
    if (!res.ok) throw new Error(`Failed to verify firmware checksum: ${res.statusText}`);
    return res.json();
  }

  public async getRecoveryGuides(): Promise<RecoveryGuide[]> {
    const res = await fetch(`${this.baseUrl}/api/recovery/guides`);
    if (!res.ok) throw new Error(`Failed to fetch recovery guides: ${res.statusText}`);
    return res.json();
  }

  public async verifyOwnership(payload: OwnershipVerification): Promise<VerificationResult> {
    const res = await fetch(`${this.baseUrl}/api/recovery/verify-ownership`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(payload),
    });
    if (!res.ok) throw new Error(`Failed to verify ownership: ${res.statusText}`);
    return res.json();
  }

  public async createRecoveryAudit(payload: {
    device_id: string;
    manufacturer: string;
    model: string;
    workflow_type: string;
    technician_id: string;
    outcome: string;
    notes?: string;
  }): Promise<RecoveryAuditRecord> {
    const res = await fetch(`${this.baseUrl}/api/recovery/audit`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(payload),
    });
    if (!res.ok) throw new Error(`Failed to record recovery audit: ${res.statusText}`);
    return res.json();
  }
}

export const apiClient = new ApiClient();
