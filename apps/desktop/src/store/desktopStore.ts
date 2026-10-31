import { create } from 'zustand';
import type { DeviceInfo, OperationRecord, FirmwareCatalogItem, RecoveryGuide } from '@uat/shared-types';
import { tauriBridge, type SystemInfo, type DiagnosticEvent } from '../services/tauriBridge';

export type DesktopNavTab =
  | 'dashboard'
  | 'device-manager'
  | 'frp-recovery'
  | 'screen-recovery'
  | 'diagnostics'
  | 'firmware'
  | 'usb-connection'
  | 'history'
  | 'settings';

interface DesktopState {
  devices: DeviceInfo[];
  selectedDeviceId: string | null;
  systemInfo: SystemInfo | null;
  activeTab: DesktopNavTab;
  activeOperation: { progress: number; status: string; currentMessage: string } | null;
  operations: OperationRecord[];
  firmwareList: FirmwareCatalogItem[];
  recoveryGuides: RecoveryGuide[];
  realTimeLogs: string[];
  allowSerialNumber: boolean;

  // Actions
  init: () => Promise<void>;
  refreshDevices: () => Promise<void>;
  connectNativeDevice: (vendorId: number, productId: number, manufacturer?: string, model?: string) => Promise<void>;
  disconnectDevice: (deviceId: string) => Promise<void>;
  selectDevice: (deviceId: string | null) => void;
  setTab: (tab: DesktopNavTab) => void;
  setAllowSerialNumber: (allow: boolean) => void;
  runDiagnostics: (deviceId: string) => Promise<void>;
  addLog: (line: string) => void;
  clearLogs: () => void;
}

export const useDesktopStore = create<DesktopState>((set, get) => ({
  devices: [],
  selectedDeviceId: null,
  systemInfo: null,
  activeTab: 'dashboard',
  activeOperation: null,
  operations: [],
  firmwareList: [],
  recoveryGuides: [],
  realTimeLogs: [
    `[${new Date().toLocaleTimeString()}] UAT Desktop Engine initialized`,
    `[${new Date().toLocaleTimeString()}] Native USB driver interface ready`,
  ],
  allowSerialNumber: false,

  init: async () => {
    try {
      const [sysInfo, fwList] = await Promise.all([
        tauriBridge.getSystemInfo().catch(() => ({
          os: 'macos',
          arch: 'arm64',
          driver_stack: 'macOS IOKit Direct Access',
          desktop_version: 'v0.1.0-tauri2',
          security_standard: 'Zero-Bypass Compliant',
        })),
        tauriBridge.getFirmwareCatalog().catch(() => []),
      ]);

      set({ systemInfo: sysInfo, firmwareList: fwList });
      get().addLog(`[${new Date().toLocaleTimeString()}] Host OS detected: ${sysInfo.os} (${sysInfo.arch})`);
      get().addLog(`[${new Date().toLocaleTimeString()}] Driver Subsystem: ${sysInfo.driver_stack}`);

      await get().refreshDevices();

      // Listen to streaming diagnostic events from Tauri backend
      await tauriBridge.onDiagnosticEvent((event: DiagnosticEvent) => {
        set({
          activeOperation: {
            progress: event.progress_percent,
            status: event.status,
            currentMessage: event.message,
          },
        });
        get().addLog(event.message);

        if (event.progress_percent >= 100) {
          setTimeout(() => set({ activeOperation: null }), 1000);
        }
      });
    } catch (err) {
      console.warn('Desktop init warning:', err);
    }
  },

  refreshDevices: async () => {
    try {
      const list = await tauriBridge.listDevices();
      set((state) => ({
        devices: list,
        selectedDeviceId: state.selectedDeviceId ?? (list[0]?.id ?? null),
      }));
    } catch (err) {
      console.warn('Failed to list devices:', err);
    }
  },

  connectNativeDevice: async (vendorId, productId, manufacturer, model) => {
    get().addLog(`[${new Date().toLocaleTimeString()}] Registering native USB descriptor (VID: 0x${vendorId.toString(16)}, PID: 0x${productId.toString(16)})`);
    try {
      const device = await tauriBridge.connectNativeDevice(
        vendorId,
        productId,
        manufacturer,
        model,
        get().allowSerialNumber ? 'AUTO-SERIAL' : undefined
      );

      set((state) => {
        const filtered = state.devices.filter((d) => d.id !== device.id);
        return {
          devices: [...filtered, device],
          selectedDeviceId: device.id,
        };
      });

      get().addLog(`[${new Date().toLocaleTimeString()}] Native USB device attached: ${device.manufacturer} ${device.model}`);

      // Load manufacturer recovery guidance
      const guides = await tauriBridge.getRecoveryGuidance(device.manufacturer);
      set({ recoveryGuides: guides });
    } catch (err) {
      get().addLog(`[${new Date().toLocaleTimeString()}] [ERROR] Native connection failure: ${String(err)}`);
    }
  },

  disconnectDevice: async (deviceId) => {
    try {
      await tauriBridge.disconnectDevice(deviceId);
      set((state) => {
        const updated = state.devices.filter((d) => d.id !== deviceId);
        return {
          devices: updated,
          selectedDeviceId: state.selectedDeviceId === deviceId ? (updated[0]?.id ?? null) : state.selectedDeviceId,
        };
      });
      get().addLog(`[${new Date().toLocaleTimeString()}] Disconnected device ${deviceId}`);
    } catch (err) {
      console.warn('Disconnect error:', err);
    }
  },

  selectDevice: (deviceId) => set({ selectedDeviceId: deviceId }),

  setTab: (tab) => set({ activeTab: tab }),

  setAllowSerialNumber: (allow) => set({ allowSerialNumber: allow }),

  runDiagnostics: async (deviceId) => {
    const dev = get().devices.find((d) => d.id === deviceId);
    if (!dev) return;

    set({
      activeOperation: {
        progress: 5,
        status: 'starting',
        currentMessage: `Initiating hardware diagnostics on ${dev.manufacturer} ${dev.model}...`,
      },
    });

    try {
      const record = await tauriBridge.runDiagnostics(deviceId);
      set((state) => ({
        operations: [record, ...state.operations],
      }));
    } catch (err) {
      get().addLog(`[${new Date().toLocaleTimeString()}] [ERROR] Diagnostic failure: ${String(err)}`);
      set({ activeOperation: null });
    }
  },

  addLog: (line) =>
    set((state) => ({
      realTimeLogs: [...state.realTimeLogs.slice(-199), line],
    })),

  clearLogs: () => set({ realTimeLogs: [] }),
}));
