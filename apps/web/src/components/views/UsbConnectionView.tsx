'use client';

import React from 'react';
import { useDeviceStore } from '@/store/deviceStore';
import { Card, CardHeader, CardTitle, CardDescription, CardContent } from '../ui/Card';
import { Button } from '../ui/Button';
import { Badge } from '../ui/Badge';
import { Usb, CheckCircle2, AlertTriangle } from 'lucide-react';

export function UsbConnectionView() {
  const { webUsbSupported, connectDevice } = useDeviceStore();

  const isChromium = typeof window !== 'undefined' && 'chrome' in window;
  const isWebUsbAvailable = webUsbSupported ?? false;

  return (
    <div className="space-y-6">
      <div>
        <h2 className="text-xl font-bold text-white tracking-tight flex items-center gap-2">
          <Usb className="h-5 w-5 text-cyan-400" />
          USB Connection Management & Diagnostics
        </h2>
        <p className="text-xs text-slate-400">
          USB driver stack diagnostics, browser API feature detection, and platform setup instructions
        </p>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
        {/* Left Column: Browser API Health */}
        <div className="lg:col-span-2 space-y-6">
          <Card>
            <CardHeader>
              <CardTitle>Browser USB Environment Check</CardTitle>
              <CardDescription>
                Validates clientside hardware access APIs according to W3C specifications
              </CardDescription>
            </CardHeader>

            <CardContent className="space-y-3">
              {[
                {
                  name: 'WebUSB API (navigator.usb)',
                  supported: isWebUsbAvailable,
                  note: isWebUsbAvailable ? 'Available for direct USB endpoint communication' : 'Unsupported in Firefox/Safari. Use Chrome, Edge, or UAT Desktop.',
                },
                {
                  name: 'Chromium Engine Base',
                  supported: isChromium,
                  note: isChromium ? 'Chromium runtime identified' : 'Non-Chromium browser detected',
                },
                {
                  name: 'Secure Context (HTTPS or localhost)',
                  supported: typeof window !== 'undefined' && window.isSecureContext,
                  note: 'Mandatory for browser hardware access',
                },
              ].map((item, i) => (
                <div
                  key={i}
                  className="rounded-lg border border-slate-800 bg-slate-950/40 p-3.5 flex items-start justify-between"
                >
                  <div className="space-y-1">
                    <span className="text-xs font-semibold text-white block">{item.name}</span>
                    <span className="text-[11px] text-slate-400 block">{item.note}</span>
                  </div>

                  {item.supported ? (
                    <Badge variant="success" className="gap-1">
                      <CheckCircle2 className="h-3 w-3" />
                      Active
                    </Badge>
                  ) : (
                    <Badge variant="warning" className="gap-1">
                      <AlertTriangle className="h-3 w-3" />
                      Unavailable
                    </Badge>
                  )}
                </div>
              ))}
            </CardContent>
          </Card>

          {/* Platform Specific Driver Instructions */}
          <Card>
            <CardHeader>
              <CardTitle>Platform USB Driver Guidance</CardTitle>
              <CardDescription>
                System prerequisites for proper hardware communication across Windows, Linux, and macOS
              </CardDescription>
            </CardHeader>

            <CardContent className="space-y-4 text-xs text-slate-300">
              <div className="rounded-lg border border-slate-800 bg-slate-950/40 p-3 space-y-1.5">
                <span className="font-semibold text-cyan-400 block">Windows (10 / 11)</span>
                <p className="text-slate-400 leading-relaxed">
                  WebUSB requires the generic <strong>WinUSB</strong> driver. If Windows installs an exclusive proprietary driver for ADB/Fastboot,
                  use Zadig (zadig.akeo.ie) to associate the USB interface with WinUSB, or run the native <strong>UAT Desktop</strong> app which handles native interfaces automatically.
                </p>
              </div>

              <div className="rounded-lg border border-slate-800 bg-slate-950/40 p-3 space-y-1.5">
                <span className="font-semibold text-cyan-400 block">Linux (Ubuntu / Debian / Fedora)</span>
                <p className="text-slate-400 leading-relaxed">
                  Non-root USB access requires standard udev rules for Android vendor IDs:
                </p>
                <div className="bg-slate-900 p-2.5 rounded font-mono text-[11px] text-slate-300 border border-slate-800 select-all">
                  sudo apt-get install android-sdk-platform-tools-common
                </div>
              </div>

              <div className="rounded-lg border border-slate-800 bg-slate-950/40 p-3 space-y-1.5">
                <span className="font-semibold text-cyan-400 block">macOS</span>
                <p className="text-slate-400 leading-relaxed">
                  macOS grants direct user-space USB access via IOKit. No third-party kernel drivers are required. Simply grant browser USB permissions when prompted.
                </p>
              </div>
            </CardContent>
          </Card>
        </div>

        {/* Right Column: Connection Helper */}
        <div className="space-y-6">
          <Card>
            <CardHeader>
              <CardTitle>USB Connection Assistant</CardTitle>
              <CardDescription>Trigger explicit browser USB prompt</CardDescription>
            </CardHeader>
            <CardContent className="space-y-3">
              <p className="text-xs text-slate-400 leading-relaxed">
                Clicking the button below requests explicit user permission via the browser's native USB picker.
              </p>
              <Button
                variant="cyan"
                size="md"
                disabled={!isWebUsbAvailable}
                onClick={connectDevice}
                className="w-full gap-2"
              >
                <Usb className="h-4 w-4" />
                Prompt USB Device Picker
              </Button>
            </CardContent>
          </Card>
        </div>
      </div>
    </div>
  );
}
