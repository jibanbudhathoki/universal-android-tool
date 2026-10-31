# Universal Android Tool (UAT)

Universal Android Tool (UAT) is a professional cross-platform Android servicing platform designed to communicate with compatible Android devices through USB across Web and Desktop (Windows, Linux, macOS).

## 🚀 Key Features

- **Cross-Platform**: Modern Web Application (WebUSB) & Native Desktop Application (Tauri 2).
- **Zero-Bypass Ethical Architecture**: Strictly manufacturer-authorized workflows, ownership verification, and transparent operation reporting. No unauthorized FRP or lock removal bypass exploits.
- **Unified Rust Device Engine**: High-performance asynchronous USB device discovery, capability detection, and protocol handling.
- **Manufacturer & Device Registry**: Structured hardware profiles for Samsung, Xiaomi, Redmi, POCO, OPPO, Vivo, Realme, Motorola, OnePlus, Google Pixel, and Nokia.
- **Accurate Diagnostic Pipelines**: Real-time inspection of device capabilities before exposing any operations.

---

## 📁 Monorepo Structure

```
universal-android-tool/
├── apps/
│   ├── web/                    # Next.js 15+ frontend (WebUSB, Zustand, Tailwind CSS)
│   └── desktop/                # Tauri 2 desktop frontend (React, Rust native USB)
├── backend/
│   └── rust-api/               # Axum REST & WebSocket service
├── packages/
│   ├── device-engine/          # USB lifecycle, capabilities probe, connection manager
│   ├── device-registry/        # Vendor IDs, Product IDs, and manufacturer profiles
│   ├── protocol-adapters/      # ADB, Fastboot, MTP, and Generic USB adapters
│   ├── recovery-workflows/     # Manufacturer-authorized FRP and Screen Lock workflows
│   ├── firmware-manager/       # Official firmware catalog & compatibility checking
│   └── shared-types/           # Shared models in Rust crate & TypeScript package
├── docs/                       # Architecture and security specifications
└── tests/                      # Workspace integration test suite
```

---

## 🛠️ Prerequisites

- **Node.js**: v20+ (tested on v24.20.0)
- **Package Manager**: `pnpm` (tested on v12.16.1)
- **Rust Toolchain**: `rustc` & `cargo` 1.80+ (tested on 1.94.1)

---

## 🚦 Getting Started

### 1. Install Node.js Dependencies

```bash
pnpm install
```

### 2. Verify Rust Crates & Build

```bash
cargo check --workspace
cargo test --workspace
```

### 3. Run Web Application

```bash
pnpm dev:web
```
Open [http://localhost:3000](http://localhost:3000) to view the WebUSB diagnostic dashboard.

### 4. Run Rust Backend API

```bash
pnpm dev:backend
```
API runs on `http://127.0.0.1:8080`.
- Health check: `GET http://127.0.0.1:8080/api/health`
- Device list: `GET http://127.0.0.1:8080/api/devices`
- Firmware catalog: `GET http://127.0.0.1:8080/api/firmware/catalog`
- Recovery guides: `GET http://127.0.0.1:8080/api/recovery/guides`

### 5. Run Desktop Application (Tauri 2)

```bash
pnpm dev:desktop
```

---

## 🔒 Security Compliance

Please review [docs/SECURITY.md](docs/SECURITY.md) for detailed information on authorized servicing procedures, device ownership verification, and security guarantees.
