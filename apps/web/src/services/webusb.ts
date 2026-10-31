import type { DeviceCapabilities, DeviceInfo, SupportedProtocol } from '@uat/shared-types';

export const KNOWN_VENDORS: Record<number, string> = {
  0x04e8: 'Samsung',
  0x18d1: 'Google Pixel',
  0x2717: 'Xiaomi',
  0x22d9: 'OPPO',
  0x2e04: 'Vivo',
  0x22b8: 'Motorola',
  0x2a70: 'OnePlus',
  0x0421: 'Nokia',
  0x0bb4: 'HTC',
  0x12d1: 'Huawei',
  0x0fce: 'Sony',
  0x1004: 'LG',
  0x05c6: 'Qualcomm',
};

export class WebUsbService {
  private static instance: WebUsbService;
  private connectedUsbDevices: Map<string, USBDevice> = new Map();

  private constructor() {}

  public static getInstance(): WebUsbService {
    if (!WebUsbService.instance) {
      WebUsbService.instance = new WebUsbService();
    }
    return WebUsbService.instance;
  }

  public isSupported(): boolean {
    return typeof navigator !== 'undefined' && 'usb' in navigator && Boolean(navigator.usb);
  }

  public async getPairedDevices(allowSerialNumber = false): Promise<DeviceInfo[]> {
    if (!this.isSupported() || !navigator.usb) return [];

    try {
      const devices = await navigator.usb.getDevices();
      return devices.map((d) => this.mapUsbDeviceToDeviceInfo(d, allowSerialNumber));
    } catch (err) {
      console.warn('Error reading paired WebUSB devices:', err);
      return [];
    }
  }

  public async requestDevice(allowSerialNumber = false): Promise<DeviceInfo> {
    if (!this.isSupported() || !navigator.usb) {
      throw new Error('WebUSB API is not supported in this browser. Please use a Chromium-based browser or the desktop client.');
    }

    // Explicit user gesture prompt
    const usbDevice = await navigator.usb.requestDevice({
      filters: [], // Allow user to select any USB device
    });

    const deviceId = this.generateDeviceId(usbDevice);
    this.connectedUsbDevices.set(deviceId, usbDevice);

    return this.mapUsbDeviceToDeviceInfo(usbDevice, allowSerialNumber);
  }

  public async openDevice(deviceId: string): Promise<boolean> {
    const dev = this.connectedUsbDevices.get(deviceId);
    if (!dev) return false;

    try {
      if (!dev.opened) {
        await dev.open();
      }
      return true;
    } catch (err) {
      console.warn('Failed to open USB device:', err);
      return false;
    }
  }

  public async closeDevice(deviceId: string): Promise<void> {
    const dev = this.connectedUsbDevices.get(deviceId);
    if (dev && dev.opened) {
      try {
        await dev.close();
      } catch (err) {
        console.warn('Error closing USB device:', err);
      }
    }
    this.connectedUsbDevices.delete(deviceId);
  }

  public generateDeviceId(device: USBDevice): string {
    return `usb-${device.vendorId.toString(16).padStart(4, '0')}:${device.productId.toString(16).padStart(4, '0')}`;
  }

  public mapUsbDeviceToDeviceInfo(device: USBDevice, allowSerialNumber = false): DeviceInfo {
    const vendorName = KNOWN_VENDORS[device.vendorId] || device.manufacturerName || `Vendor (0x${device.vendorId.toString(16).padStart(4, '0')})`;
    const modelName = device.productName || `Device (0x${device.productId.toString(16).padStart(4, '0')})`;
    const id = this.generateDeviceId(device);

    // Protocol detection based on USB interface classes
    const supportedProtocols: SupportedProtocol[] = [];
    let adbFound = false;
    let fastbootFound = false;

    if (device.configurations && device.configurations.length > 0) {
      for (const config of device.configurations) {
        for (const iface of config.interfaces) {
          for (const alternate of iface.alternates) {
            // Android Debug Bridge USB interface class: 0xFF, subclass: 0x42, protocol: 0x01
            if (alternate.interfaceClass === 0xff && alternate.interfaceSubclass === 0x42 && alternate.interfaceProtocol === 0x01) {
              adbFound = true;
            }
            // Fastboot USB interface class: 0xFF, subclass: 0x42, protocol: 0x03
            if (alternate.interfaceClass === 0xff && alternate.interfaceSubclass === 0x42 && alternate.interfaceProtocol === 0x03) {
              fastbootFound = true;
            }
          }
        }
      }
    }

    if (adbFound) supportedProtocols.push('adb');
    if (fastbootFound) supportedProtocols.push('fastboot');
    supportedProtocols.push('mtp');

    const capabilities: DeviceCapabilities = {
      adb_available: adbFound,
      fastboot_available: fastbootFound,
      mtp_available: true,
      diagnostic_mode_supported: adbFound || fastbootFound,
      recovery_mode_supported: fastbootFound,
      oem_lock_status_readable: fastbootFound,
      authorized_reset_supported: adbFound || fastbootFound,
    };

    return {
      id,
      vendor_id: device.vendorId,
      product_id: device.productId,
      manufacturer: vendorName,
      model: modelName,
      serial_number: allowSerialNumber && device.serialNumber ? device.serialNumber : undefined,
      connection_mode: 'webusb',
      supported_protocols: supportedProtocols,
      capabilities,
      is_authorized: false,
    };
  }
}

export const webUsbService = WebUsbService.getInstance();
