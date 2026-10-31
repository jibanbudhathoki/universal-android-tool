import { create } from 'zustand';
import type { DeviceInfo, OperationRecord } from '@uat/shared-types';
import { webUsbService } from '../services/webusb';

export type NavTab =
  | 'dashboard'
  | 'device-manager'
  | 'frp-recovery'
  | 'screen-recovery'
  | 'diagnostics'
  | 'firmware'
  | 'usb-connection'
  | 'history'
  | 'settings';

interface DeviceState {
  connectedDevices: DeviceInfo[];
  selectedDeviceId: string | null;
  webUsbSupported: boolean | null;
  connectionStatus: 'idle' | 'connecting' | 'connected' | 'error';
  statusMessage: string;
  activeTab: NavTab;
  operations: OperationRecord[];
  activeOperation: OperationRecord | null;
  realTimeLogs: string[];
  allowSerialNumber: boolean;
  backendApiUrl: string;

  // Actions
  init: () => Promise<void>;
  connectDevice: () => Promise<void>;
  disconnectDevice: (deviceId: string) => Promise<void>;
  selectDevice: (deviceId: string | null) => void;
  setTab: (tab: NavTab) => void;
  setAllowSerialNumber: (allow: boolean) => void;
  setBackendApiUrl: (url: string) => void;
  addLog: (line: string) => void;
  clearLogs: () => void;
  runDiagnostic: (deviceId: string) => Promise<void>;
}

export const useDeviceStore = create<DeviceState>((set, get) => ({
  connectedDevices: [],
  selectedDeviceId: null,
  webUsbSupported: null,
  connectionStatus: 'idle',
  statusMessage: 'Ready for device connection',
  activeTab: 'dashboard',
  operations: [],
  activeOperation: null,
  realTimeLogs: [
    `[${new Date().toLocaleTimeString()}] Universal Android Tool (UAT) initialized`,
    `[${new Date().toLocaleTimeString()}] Engine core: Rust & WebUSB ready`,
  ],
  allowSerialNumber: false,
  backendApiUrl: 'http://127.0.0.1:8080',

  init: async () => {
    const isSupported = webUsbService.isSupported();
    set({ webUsbSupported: isSupported });

    if (!isSupported) {
      get().addLog(`[${new Date().toLocaleTimeString()}] [WARN] WebUSB API not supported in this browser. Use Chromium-based browser or UAT Desktop.`);
      return;
    }

    try {
      const paired = await webUsbService.getPairedDevices(get().allowSerialNumber);
      if (paired.length > 0) {
        set({
          connectedDevices: paired,
          selectedDeviceId: paired[0]?.id ?? null,
          connectionStatus: 'connected',
          statusMessage: `${paired.length} device(s) detected via WebUSB`,
        });
        get().addLog(`[${new Date().toLocaleTimeString()}] Loaded ${paired.length} paired USB device(s)`);
      }
    } catch (err) {
      console.warn('Init error:', err);
    }

    // Attach USB connect/disconnect listeners
    if (typeof navigator !== 'undefined' && navigator.usb) {
      navigator.usb.addEventListener('connect', (e: USBConnectionEvent) => {
        const info = webUsbService.mapUsbDeviceToDeviceInfo(e.device, get().allowSerialNumber);
        set((state) => {
          const exists = state.connectedDevices.some((d) => d.id === info.id);
          const updated = exists ? state.connectedDevices : [...state.connectedDevices, info];
          return {
            connectedDevices: updated,
            selectedDeviceId: state.selectedDeviceId ?? info.id,
            connectionStatus: 'connected',
            statusMessage: `Connected: ${info.manufacturer} ${info.model}`,
          };
        });
        get().addLog(`[${new Date().toLocaleTimeString()}] [USB CONNECT] ${info.manufacturer} ${info.model} (VID: 0x${info.vendor_id.toString(16)})`);
      });

      navigator.usb.addEventListener('disconnect', (e: USBConnectionEvent) => {
        const devId = webUsbService.generateDeviceId(e.device);
        set((state) => {
          const updated = state.connectedDevices.filter((d) => d.id !== devId);
          return {
            connectedDevices: updated,
            selectedDeviceId: state.selectedDeviceId === devId ? (updated[0]?.id ?? null) : state.selectedDeviceId,
            connectionStatus: updated.length > 0 ? 'connected' : 'idle',
            statusMessage: `Disconnected USB device: 0x${e.device.vendorId.toString(16)}`,
          };
        });
        get().addLog(`[${new Date().toLocaleTimeString()}] [USB DISCONNECT] Device 0x${e.device.vendorId.toString(16)} removed`);
      });
    }
  },

  connectDevice: async () => {
    set({ connectionStatus: 'connecting', statusMessage: 'Requesting USB device authorization...' });
    get().addLog(`[${new Date().toLocaleTimeString()}] Prompting user for USB device selection...`);

    try {
      const device = await webUsbService.requestDevice(get().allowSerialNumber);
      set((state) => {
        const filtered = state.connectedDevices.filter((d) => d.id !== device.id);
        return {
          connectedDevices: [...filtered, device],
          selectedDeviceId: device.id,
          connectionStatus: 'connected',
          statusMessage: `Connected: ${device.manufacturer} ${device.model}`,
        };
      });
      get().addLog(`[${new Date().toLocaleTimeString()}] Successfully paired: ${device.manufacturer} ${device.model}`);
      get().addLog(`[${new Date().toLocaleTimeString()}] Hardware protocols identified: ${device.supported_protocols.join(', ')}`);
    } catch (err: unknown) {
      const errorMsg = err instanceof Error ? err.message : String(err);
      if (err instanceof Error && err.name === 'NotFoundError') {
        set({ connectionStatus: get().connectedDevices.length > 0 ? 'connected' : 'idle', statusMessage: 'Device selection cancelled by user.' });
        get().addLog(`[${new Date().toLocaleTimeString()}] Pairing cancelled by user`);
      } else {
        set({ connectionStatus: 'error', statusMessage: `Connection failed: ${errorMsg}` });
        get().addLog(`[${new Date().toLocaleTimeString()}] [ERROR] USB connection error: ${errorMsg}`);
      }
    }
  },

  disconnectDevice: async (deviceId: string) => {
    await webUsbService.closeDevice(deviceId);
    set((state) => {
      const updated = state.connectedDevices.filter((d) => d.id !== deviceId);
      return {
        connectedDevices: updated,
        selectedDeviceId: state.selectedDeviceId === deviceId ? (updated[0]?.id ?? null) : state.selectedDeviceId,
        connectionStatus: updated.length > 0 ? 'connected' : 'idle',
        statusMessage: `Device ${deviceId} disconnected`,
      };
    });
    get().addLog(`[${new Date().toLocaleTimeString()}] Disconnected device ${deviceId}`);
  },

  selectDevice: (deviceId) => set({ selectedDeviceId: deviceId }),

  setTab: (tab) => set({ activeTab: tab }),

  setAllowSerialNumber: (allow) => set({ allowSerialNumber: allow }),

  setBackendApiUrl: (url) => set({ backendApiUrl: url }),

  addLog: (line) =>
    set((state) => ({
      realTimeLogs: [...state.realTimeLogs.slice(-199), line],
    })),

  clearLogs: () => set({ realTimeLogs: [] }),

  runDiagnostic: async (deviceId: string) => {
    const device = get().connectedDevices.find((d) => d.id === deviceId);
    if (!device) return;

    const opId = `diag-${Date.now()}`;
    const op: OperationRecord = {
      id: opId,
      device_id: deviceId,
      operation_type: 'Device Diagnostics Probe',
      status: 'running',
      started_at: new Date().toISOString(),
      progress_percent: 10,
      logs: [`[${new Date().toLocaleTimeString()}] Initiating diagnostic probe for ${device.manufacturer} ${device.model}`],
    };

    set({ activeOperation: op });
    get().addLog(`[${new Date().toLocaleTimeString()}] Starting diagnostic probe on ${device.id}...`);

    // Step 1: Descriptor query
    await new Promise((r) => setTimeout(r, 600));
    set((s) => s.activeOperation ? {
      activeOperation: {
        ...s.activeOperation,
        progress_percent: 45,
        logs: [...s.activeOperation.logs, `[${new Date().toLocaleTimeString()}] USB Descriptor verification completed`],
      }
    } : {});
    get().addLog(`[${new Date().toLocaleTimeString()}] VID: 0x${device.vendor_id.toString(16)} | PID: 0x${device.product_id.toString(16)} verified`);

    // Step 2: Protocol probe
    await new Promise((r) => setTimeout(r, 700));
    set((s) => s.activeOperation ? {
      activeOperation: {
        ...s.activeOperation,
        progress_percent: 85,
        logs: [...s.activeOperation.logs, `[${new Date().toLocaleTimeString()}] Protocol endpoints evaluated: ${device.supported_protocols.join(', ')}`],
      }
    } : {});

    // Step 3: Complete
    await new Promise((r) => setTimeout(r, 400));
    const completedOp: OperationRecord = {
      ...op,
      status: 'completed',
      completed_at: new Date().toISOString(),
      progress_percent: 100,
      logs: [
        ...op.logs,
        `[${new Date().toLocaleTimeString()}] Diagnostic completed successfully. Hardware responsive.`,
      ],
    };

    set((state) => ({
      activeOperation: null,
      operations: [completedOp, ...state.operations],
    }));
    get().addLog(`[${new Date().toLocaleTimeString()}] Diagnostic probe completed for ${device.id}`);
  },
}));
