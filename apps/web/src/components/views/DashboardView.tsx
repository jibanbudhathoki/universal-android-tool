'use client';

import React from 'react';
import { useDeviceStore } from '@/store/deviceStore';
import { Card, CardHeader, CardTitle, CardDescription, CardContent } from '../ui/Card';
import { Button } from '../ui/Button';
import { Badge } from '../ui/Badge';
import { ProgressBar } from '../ui/ProgressBar';
import { Terminal } from '../ui/Terminal';
import {
  Smartphone,
  Usb,
  Cpu,
  ShieldCheck,
  CheckCircle2,
  XCircle,
  Activity,
  Radio,
  KeyRound,
} from 'lucide-react';
import { formatHex } from '@/lib/utils';

export function DashboardView() {
  const {
    connectedDevices,
    selectedDeviceId,
    webUsbSupported,
    connectDevice,
    disconnectDevice,
    runDiagnostic,
    activeOperation,
    realTimeLogs,
    clearLogs,
    allowSerialNumber,
    setAllowSerialNumber,
  } = useDeviceStore();

  const selectedDevice = connectedDevices.find((d) => d.id === selectedDeviceId);

  return (
    <div className="space-y-6">
      {/* Top Metric Cards */}
      <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">
        {/* Metric 1: Devices */}
        <Card className="p-4 flex items-center justify-between">
          <div className="space-y-1">
            <span className="text-[11px] font-semibold uppercase tracking-wider text-slate-500">Connected Devices</span>
            <div className="text-2xl font-bold text-white">{connectedDevices.length}</div>
            <p className="text-[11px] text-slate-400">
              {connectedDevices.length > 0 ? 'Active USB session' : 'Awaiting connection'}
            </p>
          </div>
          <div className="h-10 w-10 rounded-xl bg-blue-500/10 border border-blue-500/20 flex items-center justify-center text-blue-400">
            <Smartphone className="h-5 w-5" />
          </div>
        </Card>

        {/* Metric 2: USB Status */}
        <Card className="p-4 flex items-center justify-between">
          <div className="space-y-1">
            <span className="text-[11px] font-semibold uppercase tracking-wider text-slate-500">USB Protocol</span>
            <div className="text-2xl font-bold text-white">
              {selectedDevice ? selectedDevice.connection_mode.toUpperCase() : 'STANDBY'}
            </div>
            <p className="text-[11px] text-slate-400">
              {webUsbSupported ? 'WebUSB ready' : 'Native adapter mode'}
            </p>
          </div>
          <div className="h-10 w-10 rounded-xl bg-cyan-500/10 border border-cyan-500/20 flex items-center justify-center text-cyan-400">
            <Usb className="h-5 w-5" />
          </div>
        </Card>

        {/* Metric 3: Diagnostic Probe */}
        <Card className="p-4 flex items-center justify-between">
          <div className="space-y-1">
            <span className="text-[11px] font-semibold uppercase tracking-wider text-slate-500">Diagnostics</span>
            <div className="text-2xl font-bold text-white">
              {activeOperation ? 'IN PROGRESS' : 'READY'}
            </div>
            <p className="text-[11px] text-slate-400">
              {activeOperation ? `${activeOperation.progress_percent}% completed` : 'Safe probe active'}
            </p>
          </div>
          <div className="h-10 w-10 rounded-xl bg-emerald-500/10 border border-emerald-500/20 flex items-center justify-center text-emerald-400">
            <Activity className="h-5 w-5" />
          </div>
        </Card>

        {/* Metric 4: Platform Security */}
        <Card className="p-4 flex items-center justify-between">
          <div className="space-y-1">
            <span className="text-[11px] font-semibold uppercase tracking-wider text-slate-500">Security Standard</span>
            <div className="text-2xl font-bold text-emerald-400">AUTHORIZED</div>
            <p className="text-[11px] text-slate-400">Zero-bypass enforced</p>
          </div>
          <div className="h-10 w-10 rounded-xl bg-emerald-500/10 border border-emerald-500/20 flex items-center justify-center text-emerald-400">
            <ShieldCheck className="h-5 w-5" />
          </div>
        </Card>
      </div>

      {/* Active Operation Progress Bar */}
      {activeOperation && (
        <Card className="border-cyan-500/30 bg-cyan-950/20 p-5 space-y-3">
          <div className="flex items-center justify-between">
            <div className="flex items-center space-x-2">
              <Radio className="h-4 w-4 text-cyan-400 animate-pulse" />
              <span className="text-sm font-semibold text-white">{activeOperation.operation_type}</span>
            </div>
            <Badge variant="cyan">Running</Badge>
          </div>
          <ProgressBar progress={activeOperation.progress_percent} label="Hardware Analysis & Diagnostics" />
          <p className="text-xs text-slate-400 font-mono">
            {activeOperation.logs[activeOperation.logs.length - 1] || 'Executing operation step...'}
          </p>
        </Card>
      )}

      {/* Two Column Grid: Device Connection Panel & Capabilities */}
      <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
        {/* Column 1 & 2: Device Connection Panel */}
        <div className="lg:col-span-2 space-y-6">
          <Card>
            <CardHeader className="flex flex-row items-center justify-between">
              <div>
                <CardTitle className="flex items-center gap-2">
                  <Smartphone className="h-4 w-4 text-cyan-400" />
                  Device Connection Panel
                </CardTitle>
                <CardDescription>
                  Hardware identification and verified USB descriptor parameters
                </CardDescription>
              </div>

              {selectedDevice ? (
                <Button
                  variant="destructive"
                  size="sm"
                  onClick={() => disconnectDevice(selectedDevice.id)}
                >
                  Disconnect Device
                </Button>
              ) : (
                <Button
                  variant="cyan"
                  size="sm"
                  disabled={!webUsbSupported}
                  onClick={connectDevice}
                  className="gap-1.5"
                >
                  <Usb className="h-3.5 w-3.5" />
                  Connect Device
                </Button>
              )}
            </CardHeader>

            <CardContent>
              {selectedDevice ? (
                <div className="space-y-4">
                  {/* Manufacturer & Model banner */}
                  <div className="rounded-xl border border-slate-800 bg-slate-950/60 p-4 flex flex-col sm:flex-row justify-between items-start sm:items-center gap-3">
                    <div className="flex items-center space-x-3">
                      <div className="h-12 w-12 rounded-xl bg-cyan-500/10 border border-cyan-500/20 flex items-center justify-center text-cyan-400">
                        <Smartphone className="h-6 w-6" />
                      </div>
                      <div>
                        <h4 className="text-base font-bold text-white">
                          {selectedDevice.manufacturer} {selectedDevice.model}
                        </h4>
                        <p className="text-xs text-slate-400">
                          Hardware ID: <span className="font-mono text-cyan-300">{selectedDevice.id}</span>
                        </p>
                      </div>
                    </div>

                    <div className="flex items-center space-x-2">
                      <Button
                        variant="outline"
                        size="sm"
                        onClick={() => runDiagnostic(selectedDevice.id)}
                        disabled={Boolean(activeOperation)}
                      >
                        <Activity className="h-3.5 w-3.5 mr-1.5 text-cyan-400" />
                        Run Diagnostic
                      </Button>
                    </div>
                  </div>

                  {/* Hardware Attributes Grid */}
                  <div className="grid grid-cols-2 sm:grid-cols-4 gap-3 text-xs">
                    <div className="rounded-lg border border-slate-800 bg-slate-950/40 p-3">
                      <span className="text-slate-500 block text-[10px] uppercase">Vendor ID</span>
                      <span className="font-mono text-white text-sm font-semibold">
                        {formatHex(selectedDevice.vendor_id)}
                      </span>
                    </div>

                    <div className="rounded-lg border border-slate-800 bg-slate-950/40 p-3">
                      <span className="text-slate-500 block text-[10px] uppercase">Product ID</span>
                      <span className="font-mono text-white text-sm font-semibold">
                        {formatHex(selectedDevice.product_id)}
                      </span>
                    </div>

                    <div className="rounded-lg border border-slate-800 bg-slate-950/40 p-3">
                      <span className="text-slate-500 block text-[10px] uppercase">Connection Mode</span>
                      <span className="font-mono text-cyan-400 text-sm font-semibold uppercase">
                        {selectedDevice.connection_mode}
                      </span>
                    </div>

                    <div className="rounded-lg border border-slate-800 bg-slate-950/40 p-3">
                      <span className="text-slate-500 block text-[10px] uppercase">Device Authorization</span>
                      <span className="text-amber-400 text-sm font-semibold">Standard User</span>
                    </div>
                  </div>

                  {/* Serial Number & Security Transparency */}
                  <div className="rounded-lg border border-slate-800/80 bg-slate-950/30 p-3 flex items-center justify-between text-xs">
                    <div className="flex items-center space-x-2.5">
                      <KeyRound className="h-4 w-4 text-slate-400" />
                      <div>
                        <span className="font-medium text-slate-300">Serial Number Extraction: </span>
                        {allowSerialNumber && selectedDevice.serial_number ? (
                          <span className="font-mono text-cyan-300 font-semibold">{selectedDevice.serial_number}</span>
                        ) : (
                          <span className="text-slate-500 italic">Protected (Explicit Authorization Required)</span>
                        )}
                      </div>
                    </div>

                    <Button
                      variant="ghost"
                      size="sm"
                      onClick={() => setAllowSerialNumber(!allowSerialNumber)}
                      className="text-xs text-slate-400 hover:text-white"
                    >
                      {allowSerialNumber ? 'Hide Serial' : 'Authorize Display'}
                    </Button>
                  </div>

                  {/* Supported Protocols Tags */}
                  <div className="pt-2">
                    <span className="text-xs font-medium text-slate-400 block mb-2">Supported Protocol Adapters:</span>
                    <div className="flex flex-wrap gap-2">
                      {selectedDevice.supported_protocols.map((protocol, i) => {
                        const name = typeof protocol === 'string' ? protocol : protocol.vendor_specific;
                        return (
                          <Badge key={i} variant="cyan" className="uppercase font-mono text-[11px]">
                            {name}
                          </Badge>
                        );
                      })}
                    </div>
                  </div>
                </div>
              ) : (
                <div className="py-12 text-center space-y-4">
                  <div className="h-16 w-16 rounded-2xl bg-slate-950 border border-slate-800 flex items-center justify-center mx-auto text-slate-600">
                    <Smartphone className="h-8 w-8" />
                  </div>
                  <div className="space-y-1">
                    <h4 className="text-sm font-semibold text-white">No Android Device Selected</h4>
                    <p className="text-xs text-slate-400 max-w-sm mx-auto">
                      Connect an Android device with USB debugging enabled, or in Fastboot mode, and click "Connect Device" to begin.
                    </p>
                  </div>
                  <Button
                    variant="cyan"
                    size="md"
                    disabled={!webUsbSupported}
                    onClick={connectDevice}
                    className="gap-2"
                  >
                    <Usb className="h-4 w-4" />
                    Connect via WebUSB
                  </Button>
                </div>
              )}
            </CardContent>
          </Card>

          {/* Real-Time Operation Logs */}
          <Terminal logs={realTimeLogs} onClear={clearLogs} />
        </div>

        {/* Column 3: Capabilities & Diagnostics Matrix */}
        <div className="space-y-6">
          <Card>
            <CardHeader>
              <CardTitle className="flex items-center gap-2">
                <Cpu className="h-4 w-4 text-cyan-400" />
                Device Capabilities
              </CardTitle>
              <CardDescription>
                Real-time probe of hardware interfaces and servicing features
              </CardDescription>
            </CardHeader>

            <CardContent>
              {selectedDevice ? (
                <div className="space-y-3">
                  {[
                    { label: 'ADB Diagnostics', enabled: selectedDevice.capabilities.adb_available, desc: 'Command channel and system property reading' },
                    { label: 'Fastboot Protocol', enabled: selectedDevice.capabilities.fastboot_available, desc: 'Bootloader interface and partition query' },
                    { label: 'MTP Protocol', enabled: selectedDevice.capabilities.mtp_available, desc: 'Media Transfer Protocol access' },
                    { label: 'Diagnostic Mode', enabled: selectedDevice.capabilities.diagnostic_mode_supported, desc: 'Low-level hardware diagnostics' },
                    { label: 'Recovery Mode', enabled: selectedDevice.capabilities.recovery_mode_supported, desc: 'Stock recovery interaction' },
                    { label: 'OEM Lock Status', enabled: selectedDevice.capabilities.oem_lock_status_readable, desc: 'Bootloader lock verification' },
                    { label: 'Authorized Reset', enabled: selectedDevice.capabilities.authorized_reset_supported, desc: 'Manufacturer factory reset procedure' },
                  ].map((cap, i) => (
                    <div
                      key={i}
                      className="rounded-lg border border-slate-800/80 bg-slate-950/40 p-3 flex items-start justify-between"
                    >
                      <div className="space-y-0.5">
                        <span className="text-xs font-semibold text-slate-200 block">{cap.label}</span>
                        <span className="text-[11px] text-slate-500 leading-tight block">{cap.desc}</span>
                      </div>
                      {cap.enabled ? (
                        <CheckCircle2 className="h-4 w-4 text-emerald-400 flex-shrink-0 mt-0.5" />
                      ) : (
                        <XCircle className="h-4 w-4 text-slate-600 flex-shrink-0 mt-0.5" />
                      )}
                    </div>
                  ))}
                </div>
              ) : (
                <p className="text-xs text-slate-500 italic py-6 text-center">
                  Connect a device to inspect active hardware capabilities.
                </p>
              )}
            </CardContent>
          </Card>

          {/* Servicing Guardrail Alert */}
          <div className="rounded-xl border border-cyan-500/20 bg-cyan-950/10 p-4 space-y-2">
            <div className="flex items-center space-x-2">
              <ShieldCheck className="h-4 w-4 text-cyan-400" />
              <h4 className="text-xs font-semibold text-cyan-300">Security & Integrity Assurance</h4>
            </div>
            <p className="text-[11px] text-slate-400 leading-relaxed">
              Universal Android Tool operates strictly within manufacturer guidelines. No exploit payloads, FRP circumvention, or unauthorized locks are used.
            </p>
          </div>
        </div>
      </div>
    </div>
  );
}
