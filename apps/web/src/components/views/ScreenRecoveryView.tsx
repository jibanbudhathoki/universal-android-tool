'use client';

import React, { useState } from 'react';
import { useDeviceStore } from '@/store/deviceStore';
import { Card, CardHeader, CardTitle, CardDescription, CardContent } from '../ui/Card';
import { Button } from '../ui/Button';
import { Badge } from '../ui/Badge';
import { Dialog } from '../ui/Dialog';
import { Lock, AlertTriangle, ExternalLink, HardDrive, ShieldAlert, Cpu } from 'lucide-react';

export function ScreenRecoveryView() {
  const { connectedDevices, selectedDeviceId, addLog } = useDeviceStore();
  const selectedDevice = connectedDevices.find((d) => d.id === selectedDeviceId);

  const [confirmModalOpen, setConfirmModalOpen] = useState(false);
  const [dataLossAcknowledged, setDataLossAcknowledged] = useState(false);

  const handleResetConfirmation = () => {
    if (dataLossAcknowledged) {
      setConfirmModalOpen(false);
      addLog(`[${new Date().toLocaleTimeString()}] [SCREEN RECOVERY] User confirmed authorized recovery mode reset protocol for ${selectedDevice?.id ?? 'device'}`);
      addLog(`[${new Date().toLocaleTimeString()}] Please follow the hardware key sequence to complete Wipe Data in Stock Recovery.`);
    }
  };

  const manufacturerKeyCombos: Record<string, string> = {
    Samsung: 'Hold [Volume Up] + [Side/Power] button while connected to USB until the Samsung Recovery logo appears.',
    'Google Pixel': 'Power off device. Hold [Power] + [Volume Down] until Fastboot appears, then select Recovery Mode with Volume buttons and press Power.',
    Xiaomi: 'Power off device. Hold [Volume Up] + [Power] until Mi Recovery menu appears. Use Volume keys to navigate to Wipe Data.',
    Redmi: 'Power off device. Hold [Volume Up] + [Power] until Mi Recovery menu appears.',
    POCO: 'Power off device. Hold [Volume Up] + [Power] until POCO Recovery menu appears.',
    OPPO: 'Power off device. Hold [Volume Down] + [Power] until OPPO ColorOS Recovery appears.',
    Vivo: 'Power off device. Hold [Volume Up] + [Power] to enter Fastboot Mode, then select Recovery Mode.',
    Realme: 'Power off device. Hold [Volume Down] + [Power] until Realme UI Recovery appears.',
    Motorola: 'Power off device. Hold [Volume Down] + [Power] until Fastboot menu appears, then scroll to Recovery Mode.',
    OnePlus: 'Power off device. Hold [Volume Down] + [Power] until OxygenOS Recovery appears.',
    Nokia: 'Connect to USB charger. Power off device. Hold [Volume Up] + [Power] until Android Recovery appears.',
  };

  const activeCombo =
    (selectedDevice && manufacturerKeyCombos[selectedDevice.manufacturer]) ||
    'Power off device. Hold [Volume Down] + [Power] or [Volume Up] + [Power] until the manufacturer recovery menu appears.';

  return (
    <div className="space-y-6">
      <div>
        <h2 className="text-xl font-bold text-white tracking-tight flex items-center gap-2">
          <Lock className="h-5 w-5 text-cyan-400" />
          Screen Lock Recovery
        </h2>
        <p className="text-xs text-slate-400">
          Authorized screen-lock recovery workflows and stock recovery mode guides
        </p>
      </div>

      {/* Prominent Data-Loss Warning */}
      <div className="rounded-xl border border-red-500/30 bg-red-950/20 p-5 space-y-3 text-red-200">
        <div className="flex items-center space-x-2">
          <AlertTriangle className="h-5 w-5 text-red-400" />
          <h4 className="text-sm font-bold uppercase tracking-wider text-red-300">
            Critical Data Loss Warning
          </h4>
        </div>
        <p className="text-xs leading-relaxed text-red-200/90">
          Android hardware encryption protects user data. Unauthorized screen-lock removal without erasing user data is impossible on modern Android devices without exploit compromises.
          Legitimate recovery procedures will completely wipe all local user data, photos, and apps.
        </p>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
        <div className="lg:col-span-2 space-y-6">
          {/* Recovery Methods Card */}
          <Card>
            <CardHeader>
              <CardTitle>Authorized Recovery Methods</CardTitle>
              <CardDescription>
                Official options for resetting device access when lock credentials have been forgotten
              </CardDescription>
            </CardHeader>

            <CardContent className="space-y-5">
              {/* Method 1: Remote Erasure */}
              <div className="rounded-xl border border-slate-800 bg-slate-950/50 p-4 space-y-3">
                <div className="flex items-center justify-between">
                  <div className="flex items-center space-x-2">
                    <span className="flex h-6 w-6 items-center justify-center rounded-full bg-cyan-500/10 text-cyan-400 font-bold text-xs">
                      1
                    </span>
                    <h4 className="text-sm font-bold text-white">Cloud Remote Reset (Recommended)</h4>
                  </div>
                  <Badge variant="cyan">Zero Hardware Risk</Badge>
                </div>
                <p className="text-xs text-slate-400 leading-relaxed">
                  If the device is connected to a network, you can reset the lock screen through official account portals:
                </p>
                <div className="flex flex-wrap gap-3 pt-1">
                  <a
                    href="https://www.google.com/android/find"
                    target="_blank"
                    rel="noopener noreferrer"
                    className="inline-flex items-center gap-1.5 rounded-lg border border-slate-700 bg-slate-900 px-3 py-1.5 text-xs font-medium text-white hover:border-cyan-500"
                  >
                    Google Find My Device
                    <ExternalLink className="h-3 w-3 text-slate-400" />
                  </a>
                  <a
                    href="https://smartthingsfind.samsung.com"
                    target="_blank"
                    rel="noopener noreferrer"
                    className="inline-flex items-center gap-1.5 rounded-lg border border-slate-700 bg-slate-900 px-3 py-1.5 text-xs font-medium text-white hover:border-cyan-500"
                  >
                    Samsung SmartThings Find
                    <ExternalLink className="h-3 w-3 text-slate-400" />
                  </a>
                </div>
              </div>

              {/* Method 2: Hardware Recovery Mode */}
              <div className="rounded-xl border border-slate-800 bg-slate-950/50 p-4 space-y-3">
                <div className="flex items-center justify-between">
                  <div className="flex items-center space-x-2">
                    <span className="flex h-6 w-6 items-center justify-center rounded-full bg-cyan-500/10 text-cyan-400 font-bold text-xs">
                      2
                    </span>
                    <h4 className="text-sm font-bold text-white">Stock Recovery Mode Procedure</h4>
                  </div>
                  <Badge variant="outline">Offline Procedure</Badge>
                </div>

                <div className="rounded-lg bg-slate-900 p-3 text-xs text-cyan-200 border border-slate-800 space-y-1">
                  <span className="text-[10px] uppercase font-semibold text-slate-400 block">
                    Key Combination for {selectedDevice?.manufacturer || 'Current Device'}:
                  </span>
                  <p className="font-mono text-cyan-300 leading-relaxed">{activeCombo}</p>
                </div>

                <ol className="text-xs text-slate-400 space-y-1.5 list-decimal list-inside pl-1">
                  <li>Trigger the hardware key combination shown above.</li>
                  <li>In the Stock Recovery menu, use Volume buttons to navigate to <strong>Wipe data / factory reset</strong>.</li>
                  <li>Press the Power button to confirm the reset.</li>
                  <li>Select <strong>Reboot system now</strong> once wiping is finished.</li>
                </ol>

                <div className="pt-2">
                  <Button
                    variant="destructive"
                    size="sm"
                    onClick={() => setConfirmModalOpen(true)}
                  >
                    Confirm Authorized Reset Protocol
                  </Button>
                </div>
              </div>
            </CardContent>
          </Card>
        </div>

        {/* Right Column: Pre-Flight Checklist */}
        <div className="space-y-6">
          <Card>
            <CardHeader>
              <CardTitle>Recovery Pre-Flight Checklist</CardTitle>
              <CardDescription>Requirements before initiating reset</CardDescription>
            </CardHeader>
            <CardContent className="space-y-3 text-xs text-slate-300">
              <div className="flex items-start space-x-2.5">
                <HardDrive className="h-4 w-4 text-cyan-400 mt-0.5 flex-shrink-0" />
                <span>Device battery level should be at least 50% or connected to charger.</span>
              </div>
              <div className="flex items-start space-x-2.5">
                <ShieldAlert className="h-4 w-4 text-amber-400 mt-0.5 flex-shrink-0" />
                <span>Factory reset will NOT remove FRP (Google account verification will trigger on setup).</span>
              </div>
              <div className="flex items-start space-x-2.5">
                <Cpu className="h-4 w-4 text-emerald-400 mt-0.5 flex-shrink-0" />
                <span>SD card data is preserved; all internal flash memory will be wiped.</span>
              </div>
            </CardContent>
          </Card>
        </div>
      </div>

      {/* Confirmation Dialog */}
      <Dialog
        isOpen={confirmModalOpen}
        onClose={() => setConfirmModalOpen(false)}
        title="Confirm Authorized Factory Reset"
        description="A factory reset permanently erases all personal data from the device"
        isDestructive
        confirmLabel="Proceed with Authorized Reset"
        confirmDisabled={!dataLossAcknowledged}
        onConfirm={handleResetConfirmation}
      >
        <div className="space-y-3">
          <p className="text-xs text-red-300">
            By proceeding, you confirm that you are the lawful owner or authorized technician. All device encryption keys will be discarded,
            rendering existing user data unrecoverable.
          </p>

          <label className="flex items-start space-x-2.5 cursor-pointer text-xs text-slate-300 pt-2">
            <input
              type="checkbox"
              checked={dataLossAcknowledged}
              onChange={(e) => setDataLossAcknowledged(e.target.checked)}
              className="mt-0.5 rounded border-slate-700 text-red-600 focus:ring-red-500"
            />
            <span>I explicitly acknowledge that all user data will be permanently erased.</span>
          </label>
        </div>
      </Dialog>
    </div>
  );
}
