'use client';

import React from 'react';
import { useDeviceStore } from '@/store/deviceStore';
import { Button } from '../ui/Button';
import { Badge } from '../ui/Badge';
import { Smartphone, Usb, AlertCircle, CheckCircle2, ShieldCheck } from 'lucide-react';

export function Header() {
  const {
    connectedDevices,
    selectedDeviceId,
    webUsbSupported,
    connectionStatus,
    connectDevice,
    disconnectDevice,
  } = useDeviceStore();

  const selectedDevice = connectedDevices.find((d) => d.id === selectedDeviceId);

  return (
    <header className="sticky top-0 z-40 border-b border-slate-800/80 bg-slate-950/80 backdrop-blur-md px-6 py-3.5">
      <div className="flex items-center justify-between">
        {/* Brand */}
        <div className="flex items-center space-x-3">
          <div className="flex h-9 w-9 items-center justify-center rounded-lg bg-gradient-to-tr from-cyan-600 to-blue-600 shadow-md shadow-cyan-500/20">
            <Smartphone className="h-5 w-5 text-white" />
          </div>
          <div>
            <div className="flex items-center space-x-2">
              <span className="font-bold text-white text-base tracking-tight">Universal Android Tool</span>
              <span className="rounded bg-cyan-500/10 px-2 py-0.5 text-[10px] font-mono text-cyan-400 border border-cyan-500/30">
                v0.1.0-alpha
              </span>
            </div>
            <p className="text-[11px] text-slate-400">Enterprise Cross-Platform Servicing Engine</p>
          </div>
        </div>

        {/* Global Status & Quick Controls */}
        <div className="flex items-center space-x-4">
          {/* Security Indicator */}
          <div className="hidden lg:flex items-center space-x-1.5 text-xs text-slate-400 bg-slate-900/60 border border-slate-800 px-3 py-1.5 rounded-lg">
            <ShieldCheck className="h-4 w-4 text-emerald-400" />
            <span>Authorized Servicing Standard</span>
          </div>

          {/* WebUSB Support Pill */}
          {webUsbSupported === null ? (
            <Badge variant="outline">Checking USB...</Badge>
          ) : webUsbSupported ? (
            <Badge variant="success" className="gap-1 hidden sm:inline-flex">
              <CheckCircle2 className="h-3 w-3" />
              WebUSB Active
            </Badge>
          ) : (
            <Badge variant="warning" className="gap-1">
              <AlertCircle className="h-3 w-3" />
              WebUSB Unsupported
            </Badge>
          )}

          {/* Quick Connect / Disconnect Action */}
          {selectedDevice ? (
            <div className="flex items-center space-x-2">
              <span className="text-xs text-slate-300 font-mono hidden md:inline">
                {selectedDevice.manufacturer} (0x{selectedDevice.vendor_id.toString(16)})
              </span>
              <Button
                variant="outline"
                size="sm"
                onClick={() => disconnectDevice(selectedDevice.id)}
                className="text-red-400 border-red-500/30 hover:bg-red-500/10"
              >
                Disconnect
              </Button>
            </div>
          ) : (
            <Button
              variant="cyan"
              size="sm"
              loading={connectionStatus === 'connecting'}
              onClick={connectDevice}
              disabled={!webUsbSupported}
              className="gap-1.5"
            >
              <Usb className="h-4 w-4" />
              Connect Device
            </Button>
          )}
        </div>
      </div>
    </header>
  );
}
