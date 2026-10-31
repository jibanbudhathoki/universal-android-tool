# Universal Android Tool (UAT)

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Zero-Bypass: Compliant](https://img.shields.io/badge/Zero--Bypass-Compliant-emerald.svg)](docs/SECURITY.md)
[![Rust: 1.80+](https://img.shields.io/badge/Rust-1.80%2B-orange.svg)](https://www.rust-lang.org/)
[![Next.js: 15+](https://img.shields.io/badge/Next.js-15%2B-black.svg)](https://nextjs.org/)
[![Tauri: 2.x](https://img.shields.io/badge/Tauri-2.x-cyan.svg)](https://tauri.app/)

**Universal Android Tool (UAT)** is an enterprise-grade cross-platform Android servicing platform designed to communicate with compatible Android devices through USB across **Web** (Chromium browsers via WebUSB) and **Desktop** (Windows, Linux, macOS via Tauri 2).

Built with a modular monorepo architecture, UAT combines a high-performance asynchronous Rust core engine with modern, responsive React user interfaces.

---

## 🛡️ Core Security Architecture: Strict Zero-Bypass Pledge

Commercial and legacy servicing tools often rely on unverified exploits, bootloader vulnerabilities, and unauthorized account circumvention. **UAT strictly rejects this approach.**

- **No Unauthorized FRP Bypasses**: Factory Reset Protection recovery is managed through official manufacturer enterprise workflows and Google Account recovery portals.
- **No Lock Screen Exploits**: Respects modern File-Based Encryption (FBE). No PIN bruteforcing or credential dumping.
- **Mandatory Ownership Verification**: Validates purchase receipts, technician credentials, and customer consent before exposing recovery tools.
- **Cryptographic Audit Ledger**: Records all actions in a tamper-evident append-only ledger secured by SHA-256 hash chaining.
- **Anti-Rollback (ARB) Downgrade Prevention**: Proactively validates firmware versions to prevent hardware bricking.

For more details, see [docs/SECURITY.md](docs/SECURITY.md).

---

## 🚀 Key Features

- **Dual-Surface Platform**:
  - **Web Dashboard**: Zero-install browser experience using the W3C WebUSB API.
  - **Desktop Application**: Standalone native client powered by Tauri 2 and direct OS USB drivers.
- **Multi-Brand Registry**: Profiles and boot-mode recognition for **Samsung, Google Pixel, Xiaomi, Redmi, POCO, OPPO, Vivo, Realme, Motorola, OnePlus, Nokia**, plus Qualcomm EDL (9008) and MediaTek BROM modes.
- **Diagnostic Adapters**:
  - **ADB Protocol Codec**: 24-byte message header codec with `A_CNXN` handshakes, `A_AUTH` token challenges, and whitelisted read-only diagnostic property inspection.
  - **Fastboot Protocol Codec**: `getvar:all` parsing with strict filtering against destructive partition writes (`flash`, `erase`, `unlock`).
- **Verified Firmware Catalog**: Hardware SKU alias matching (`husky`, `shiba`, `fuxi`, `SM-S911B`), region matching, and SHA-256 binary checksum validation.
- **Hardware Recovery Guidance**: Step-by-step physical key sequences for entering Stock Recovery and Download modes across all supported OEMs.

---

## 📁 Monorepo Architecture

```
universal-android-tools/
├── apps/
│   ├── web/                    # Next.js 15+ Web application (WebUSB, Zustand, Tailwind CSS)
│   └── desktop/                # Tauri 2 Desktop application (React, Vite, Rust core)
├── backend/
│   └── rust-api/               # Axum 0.8 REST API daemon (Local device orchestration)
├── packages/
│   ├── shared-types/           # Cross-language models (Rust structs & TypeScript interfaces)
│   ├── device-registry/        # 11 OEM hardware profiles, USB VIDs/PIDs, and boot modes
│   ├── protocol-adapters/      # ADB, Fastboot, MTP, and Generic USB packet codecs
│   ├── device-engine/          # Connection manager, capability detection, operation lifecycles
│   ├── recovery-workflows/     # Legitimate FRP/Screen-lock guidance, ownership check, audit log
│   └── firmware-manager/       # Stock firmware catalog, compatibility engine, SHA-256 verifier
├── docs/                       # Technical, architectural, and security documentation
└── tests/                      # Workspace integration test suite (17 automated tests)
```

---

## 🚦 Quickstart Guide

### Prerequisites
- **Node.js**: v20+ (tested on v24.20.0)
- **pnpm**: v10+ (tested on v12.16.1)
- **Rust Toolchain**: 1.80+ (tested on 1.94.1)

### 1. Installation
```bash
# Clone the repository
git clone https://github.com/your-org/universal-android-tools.git
cd universal-android-tools

# Install Node.js dependencies
pnpm install
```

### 2. Verify Codebase & Run Tests
```bash
# Run all 67 unit & integration tests across Rust crates
cargo test --workspace

# Run TypeScript typechecks across all projects
pnpm --recursive run typecheck
```

### 3. Launch Development Environments
You can run services independently:

```bash
# Start Next.js Web Application (runs on http://localhost:3000)
pnpm dev:web

# Start Axum REST API Daemon (runs on http://127.0.0.1:8080)
pnpm dev:backend

# Start Tauri 2 Desktop Application
pnpm dev:desktop
```

---

## 📚 Technical Documentation

Explore the detailed architecture and implementation manuals:

- [**docs/ARCHITECTURE.md**](docs/ARCHITECTURE.md): System topology, threading model, and protocol framing.
- [**docs/SECURITY.md**](docs/SECURITY.md): Threat model, zero-bypass enforcement, and audit hash-chaining.
- [**docs/API_REFERENCE.md**](docs/API_REFERENCE.md): Complete REST endpoints, Tauri IPC commands, and WebUSB methods.
- [**docs/DEPLOYMENT.md**](docs/DEPLOYMENT.md): Production builds, Docker containerization, and Tauri packaging.
- [**docs/HARDWARE_SETUP.md**](docs/HARDWARE_SETUP.md): Linux UDEV rules, Windows WinUSB setup, and hardware key guides.

---

## 📄 License

Universal Android Tool is released under the **MIT License**.
