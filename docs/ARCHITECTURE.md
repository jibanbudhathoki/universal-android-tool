# Universal Android Tool (UAT) Architecture

## Overview
Universal Android Tool (UAT) is an enterprise-grade, modular, cross-platform Android device servicing system designed for both browser environments (via WebUSB) and native desktop environments (via Tauri 2 and Rust native USB adapters).

## Monorepo Layout

```
universal-android-tool/
├── apps/
│   ├── web/                    # Next.js 15+ Web application (WebUSB, Zustand, Tailwind)
│   └── desktop/                # Tauri 2 Desktop application (Native USB via Rust)
├── backend/
│   └── rust-api/               # Axum REST & WebSocket service
├── packages/
│   ├── device-engine/          # Core Rust device lifecycle, capabilities, discovery
│   ├── device-registry/        # Manufacturer profiles, USB VIDs, PIDs, portal links
│   ├── protocol-adapters/      # ADB, Fastboot, MTP, USB abstraction layers
│   ├── recovery-workflows/     # Legitimate FRP & Screen lock recovery workflows
│   ├── firmware-manager/       # Verified firmware catalog & compatibility engine
│   └── shared-types/           # Common types in both Rust crate and TypeScript package
├── docs/                       # Technical & Security documentation
└── tests/                      # Integration and end-to-end tests
```

## Communication Protocols

1. **WebUSB (Web Application)**:
   - Direct browser-to-USB communication for supported browsers (Chrome, Edge, Brave).
   - Graceful fallback and explanation when running on non-supported browsers (Safari, Firefox).
   - Explicit user consent for each connected device.

2. **Native Rust Engine (Desktop Application)**:
   - Tauri 2 IPC commands communicating directly with `uat-device-engine`.
   - Low-latency native USB handling with platform driver support across macOS, Linux, and Windows.

3. **Backend Service (Axum API)**:
   - REST endpoints for device status, capabilities, firmware catalog, and recovery guides.
   - Operates locally without requiring cloud authentication or external database dependencies for basic USB servicing.
