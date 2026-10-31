# Universal Android Tool (UAT) — Deployment Guide

This guide details instructions for building, packaging, and deploying the **Web Application**, **Desktop Application (Tauri 2)**, and **Rust API Daemon**.

---

## 1. Web Application Deployment (`apps/web`)

The web frontend is built using Next.js 15+ and React 19.

### 1.1 Production Build

```bash
# Build web production bundle
cd apps/web
pnpm build
```

The compiled output will be generated in `apps/web/.next`.

### 1.2 Docker Containerization

To deploy the web interface as a lightweight Docker container:

```dockerfile
# Dockerfile
FROM node:20-alpine AS builder
WORKDIR /app
RUN npm install -g pnpm

# Copy workspace dependencies
COPY package.json pnpm-lock.yaml pnpm-workspace.yaml tsconfig.base.json ./
COPY packages/ ./packages/
COPY apps/web/ ./apps/web/

RUN pnpm install --frozen-lockfile
RUN pnpm --filter @uat/web build

FROM node:20-alpine AS runner
WORKDIR /app
ENV NODE_ENV=production
ENV PORT=3000

COPY --from=builder /app/apps/web/.next/standalone ./
COPY --from=builder /app/apps/web/.next/static ./apps/web/.next/static
COPY --from=builder /app/apps/web/public ./apps/web/public

EXPOSE 3000
CMD ["node", "apps/web/server.js"]
```

### 1.3 Nginx Reverse Proxy with HTTPS

> **Important**: WebUSB requires a **Secure Context (`https://`)** in production environments. Access via insecure `http://` outside of `localhost` will disable `navigator.usb`.

```nginx
server {
    listen 443 ssl http2;
    server_name uat.yourdomain.com;

    ssl_certificate /etc/letsencrypt/live/uat.yourdomain.com/fullchain.pem;
    ssl_certificate_key /etc/letsencrypt/live/uat.yourdomain.com/privkey.pem;

    location / {
        proxy_pass http://127.0.0.1:3000;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto https;
    }

    # Proxy to local Rust API Daemon
    location /api/ {
        proxy_pass http://127.0.0.1:8080/api/;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
    }
}
```

---

## 2. Desktop Application Packaging (`apps/desktop`)

Tauri 2 packages the web frontend and Rust native core into standalone desktop executables.

### 2.1 macOS Build (`.dmg` / `.app`)

Prerequisites:
- Xcode Command Line Tools: `xcode-select --install`
- Rust target: `aarch64-apple-darwin` or `x86_64-apple-darwin`

```bash
# Build optimized desktop package for macOS
cd apps/desktop
pnpm tauri build
```
- Outputs: `apps/desktop/src-tauri/target/release/bundle/dmg/Universal Android Tool_0.1.0_aarch64.dmg`

#### Codesigning & Notarization (Apple Developer):
To distribute without Gatekeeper warnings:
```bash
export APPLE_CERTIFICATE="Developer ID Application: Your Company (TEAMID)"
export APPLE_ID="developer@company.com"
export APPLE_PASSWORD="app-specific-password"
export APPLE_TEAM_ID="TEAMID"

pnpm tauri build
```

---

### 2.2 Windows Build (`.msi` / `.exe`)

Prerequisites:
- Visual Studio 2022 with C++ Build Tools
- WiX Toolset v3 for MSI creation

```powershell
# From Windows PowerShell:
cd apps/desktop
pnpm tauri build
```
- Outputs:
  - `apps\desktop\src-tauri\target\release\bundle\nsis\Universal Android Tool_0.1.0_x64-setup.exe`
  - `apps\desktop\src-tauri\target\release\bundle\msi\Universal Android Tool_0.1.0_x64_en-US.msi`

---

### 2.3 Linux Build (`.deb` / `AppImage`)

Prerequisites:
```bash
sudo apt-get update && sudo apt-get install -y \
  libwebkit2gtk-4.1-dev \
  build-essential \
  curl \
  wget \
  file \
  libxdo-dev \
  libssl-dev \
  libayatana-appindicator3-dev \
  librsvg2-dev \
  libudev-dev
```

Build command:
```bash
cd apps/desktop
pnpm tauri build
```
- Outputs:
  - `apps/desktop/src-tauri/target/release/bundle/deb/universal-android-tool_0.1.0_amd64.deb`
  - `apps/desktop/src-tauri/target/release/bundle/appimage/Universal Android Tool_0.1.0_amd64.AppImage`

---

## 3. Rust API Daemon Deployment (`backend/rust-api`)

To run the Rust REST service in headless technician workshops or testing racks:

### 3.1 Binary Compilation

```bash
cargo build --release -p uat-rust-api
# Output located at: target/release/uat-rust-api
```

### 3.2 Linux Systemd Service

Create `/etc/systemd/system/uat-api.service`:

```ini
[Unit]
Description=Universal Android Tool (UAT) API Daemon
After=network.target

[Service]
Type=simple
User=technician
WorkingDirectory=/opt/uat
ExecStart=/opt/uat/uat-rust-api
Restart=always
RestartSec=5
Environment=RUST_LOG=info
Environment=PORT=8080

[Install]
WantedBy=multi-user.target
```

Enable and start:
```bash
sudo systemctl daemon-reload
sudo systemctl enable --now uat-api
```
