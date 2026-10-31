'use client';

import React from 'react';
import { useDeviceStore } from '@/store/deviceStore';
import { Card, CardHeader, CardTitle, CardDescription, CardContent } from '../ui/Card';
import { Button } from '../ui/Button';
import { Badge } from '../ui/Badge';
import { Smartphone, Usb } from 'lucide-react';
import { formatHex } from '@/lib/utils';

export function DeviceManagerView() {
  const { connectedDevices, selectedDeviceId, selectDevice, disconnectDevice, connectDevice, webUsbSupported } =
    useDeviceStore();

  return (
    <div className="space-y-6">
      <div className="flex flex-col sm:flex-row justify-between items-start sm:items-center gap-4">
        <div>
          <h2 className="text-xl font-bold text-white tracking-tight">Device Manager</h2>
          <p className="text-xs text-slate-400">
            Enumerate, manage, and inspect all active USB device connections
          </p>
        </div>

        <Button variant="cyan" size="sm" onClick={connectDevice} disabled={!webUsbSupported} className="gap-2">
          <Usb className="h-4 w-4" />
          Pair New Device
        </Button>
      </div>

      <Card>
        <CardHeader>
          <CardTitle>Discovered Devices ({connectedDevices.length})</CardTitle>
          <CardDescription>
            Click on any paired hardware interface to select it as the target for diagnostic and servicing operations
          </CardDescription>
        </CardHeader>

        <CardContent>
          {connectedDevices.length === 0 ? (
            <div className="py-12 text-center space-y-3">
              <Smartphone className="h-10 w-10 text-slate-700 mx-auto" />
              <p className="text-sm font-medium text-slate-300">No USB Devices Attached</p>
              <p className="text-xs text-slate-500 max-w-sm mx-auto">
                Connect an Android device via USB and click "Pair New Device" to initiate connection.
              </p>
            </div>
          ) : (
            <div className="divide-y divide-slate-800">
              {connectedDevices.map((device) => {
                const isSelected = device.id === selectedDeviceId;

                return (
                  <div
                    key={device.id}
                    className={`py-4 px-3 flex flex-col sm:flex-row items-start sm:items-center justify-between gap-4 rounded-xl transition-all ${
                      isSelected ? 'bg-cyan-950/20 border border-cyan-500/30' : 'hover:bg-slate-900/40'
                    }`}
                  >
                    <div className="flex items-center space-x-3">
                      <div
                        className={`h-10 w-10 rounded-xl flex items-center justify-center ${
                          isSelected ? 'bg-cyan-500 text-white' : 'bg-slate-800 text-slate-400'
                        }`}
                      >
                        <Smartphone className="h-5 w-5" />
                      </div>
                      <div>
                        <div className="flex items-center space-x-2">
                          <h4 className="text-sm font-bold text-white">
                            {device.manufacturer} {device.model}
                          </h4>
                          {isSelected && (
                            <Badge variant="cyan" className="text-[10px]">
                              Active Target
                            </Badge>
                          )}
                        </div>
                        <div className="flex items-center space-x-3 text-xs text-slate-400 font-mono mt-0.5">
                          <span>VID: {formatHex(device.vendor_id)}</span>
                          <span>PID: {formatHex(device.product_id)}</span>
                          <span className="uppercase text-cyan-400">{device.connection_mode}</span>
                        </div>
                      </div>
                    </div>

                    <div className="flex items-center space-x-2 w-full sm:w-auto justify-end">
                      {!isSelected && (
                        <Button variant="outline" size="sm" onClick={() => selectDevice(device.id)}>
                          Select Device
                        </Button>
                      )}
                      <Button
                        variant="destructive"
                        size="sm"
                        onClick={() => disconnectDevice(device.id)}
                      >
                        Disconnect
                      </Button>
                    </div>
                  </div>
                );
              })}
            </div>
          )}
        </CardContent>
      </Card>
    </div>
  );
}
