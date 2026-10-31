# Universal Android Tool (UAT) — Hardware & Driver Setup Guide

This document provides step-by-step instructions for configuring USB drivers, operating system permissions, and hardware connections across **Linux**, **Windows**, and **macOS**.

---

## 1. Operating System USB Driver Configuration

### 1.1 Linux: UDEV Rules Configuration

By default, Linux limits raw USB communication with non-root processes. To allow standard users and browsers (WebUSB) to communicate with Android devices, install UDEV rules:

Create file `/etc/udev/rules.d/51-android-uat.rules`:

```udev
# Universal Android Tool (UAT) - Comprehensive Android Device UDEV Rules

# Google Pixel & Fastboot
SUBSYSTEM=="usb", ATTR{idVendor}=="18d1", MODE="0666", GROUP="plugdev"

# Samsung Electronics (MTP, ADB, Odin Download Mode)
SUBSYSTEM=="usb", ATTR{idVendor}=="04e8", MODE="0666", GROUP="plugdev"

# Xiaomi, Redmi, POCO
SUBSYSTEM=="usb", ATTR{idVendor}=="2717", MODE="0666", GROUP="plugdev"

# OPPO
SUBSYSTEM=="usb", ATTR{idVendor}=="22d9", MODE="0666", GROUP="plugdev"

# Vivo
SUBSYSTEM=="usb", ATTR{idVendor}=="2e04", MODE="0666", GROUP="plugdev"

# Realme
SUBSYSTEM=="usb", ATTR{idVendor}=="2a70", MODE="0666", GROUP="plugdev"

# Motorola
SUBSYSTEM=="usb", ATTR{idVendor}=="22b8", MODE="0666", GROUP="plugdev"

# OnePlus
SUBSYSTEM=="usb", ATTR{idVendor}=="2a70", MODE="0666", GROUP="plugdev"

# Nokia / HMD Global
SUBSYSTEM=="usb", ATTR{idVendor}=="0421", MODE="0666", GROUP="plugdev"

# HTC
SUBSYSTEM=="usb", ATTR{idVendor}=="0bb4", MODE="0666", GROUP="plugdev"

# Huawei
SUBSYSTEM=="usb", ATTR{idVendor}=="12d1", MODE="0666", GROUP="plugdev"

# Sony
SUBSYSTEM=="usb", ATTR{idVendor}=="0fce", MODE="0666", GROUP="plugdev"

# LG Electronics
SUBSYSTEM=="usb", ATTR{idVendor}=="1004", MODE="0666", GROUP="plugdev"

# Qualcomm EDL 9008 Diagnostic Mode
SUBSYSTEM=="usb", ATTR{idVendor}=="05c6", ATTR{idProduct}=="9008", MODE="0666", GROUP="plugdev"

# MediaTek BROM / Preloader Mode
SUBSYSTEM=="usb", ATTR{idVendor}=="0e8d", ATTR{idProduct}=="0003", MODE="0666", GROUP="plugdev"
```

#### Apply UDEV Rules:
```bash
sudo usermod -aG plugdev $USER
sudo udevadm control --reload-rules
sudo udevadm trigger
```
*Note: Log out and log back in for group membership to take effect.*

---

### 1.2 Windows: WinUSB / UsbDK Driver Setup

On Windows, standard Android composite drivers bind endpoints exclusively to `adb.exe`. To allow **WebUSB** or direct native USB access:

1. Download **Zadig** from [https://zadig.akeo.ie/](https://zadig.akeo.ie/).
2. Connect your Android device via USB.
3. In Zadig:
   - Select **Options** > **List All Devices**.
   - Select your Android device (or composite interface).
   - Change target driver to **WinUSB (v6.1.7600.16385)**.
   - Click **Replace Driver**.
4. To revert to standard Google ADB drivers, open Windows **Device Manager**, right-click the device, and choose **Uninstall device** > check **Delete the driver software for this device**.

---

### 1.3 macOS: IOKit Permissions

macOS requires no driver installation for standard USB access:
- Browser-based WebUSB works directly in Chromium-based browsers (Chrome, Edge, Brave).
- Tauri 2 desktop app communicates directly with macOS IOKit direct access.
- Ensure Android File Transfer or other background syncing daemons are closed to avoid claiming exclusive USB interfaces.

---

## 2. Hardware Boot Modes & Key Combinations

Universal Android Tool detects and guides devices across their operational modes:

| Manufacturer | Mode | Hardware Key Sequence |
|---|---|---|
| **Samsung** | Recovery Mode | Connect USB cable. Hold `[Volume Up]` + `[Power/Side]` until Samsung recovery logo appears. |
| **Samsung** | Odin Download Mode | Power off. Hold `[Volume Down]` + `[Volume Up]` simultaneously while plugging in USB cable. |
| **Google Pixel** | Fastboot Mode | Power off. Hold `[Volume Down]` + `[Power]` until Fastboot menu appears. |
| **Google Pixel** | Stock Recovery | From Fastboot menu, scroll with Volume keys to select "Recovery Mode" and press Power. |
| **Xiaomi / POCO** | Fastboot Mode | Power off. Hold `[Volume Down]` + `[Power]` until FASTBOOT bunny or logo appears. |
| **Xiaomi / POCO** | Mi Recovery | Power off. Hold `[Volume Up]` + `[Power]` until Mi recovery menu appears. |
| **Motorola** | Fastboot Mode | Power off. Hold `[Volume Down]` + `[Power]` for 3 seconds, then release. |
| **OnePlus** | Fastboot Mode | Power off. Hold `[Volume Up]` + `[Volume Down]` + `[Power]` until Fastboot appears. |
| **Generic Qualcomm**| EDL Mode (9008) | Power off. Hold both `[Volume Up]` + `[Volume Down]` while inserting USB cable, or short test points. |

---

## 3. Physical Hardware & Cable Recommendations

1. **Cables**: Always use a high-quality data-rated USB cable (USB 2.0 High-Speed 480 Mbps or USB 3.x SuperSpeed). Charge-only cables will not expose data pins (D+/D-) and will fail discovery.
2. **Ports**: Connect directly to host motherboard ports (rear I/O on desktop towers). Avoid unpowered USB hubs or keyboard pass-through ports.
3. **USB 3.0 vs USB 2.0**: Fastboot on certain AMD chipsets experiences handshake timing issues over USB 3.1/3.2 ports. If Fastboot fails to respond, connect via a native USB 2.0 port or through an active hub.
