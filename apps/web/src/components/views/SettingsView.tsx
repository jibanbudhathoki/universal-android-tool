'use client';

import React from 'react';
import { useDeviceStore } from '@/store/deviceStore';
import { Card, CardHeader, CardTitle, CardDescription, CardContent } from '../ui/Card';
import { Button } from '../ui/Button';
import { Settings, ShieldCheck, KeyRound, Server } from 'lucide-react';

export function SettingsView() {
  const {
    backendApiUrl,
    setBackendApiUrl,
    allowSerialNumber,
    setAllowSerialNumber,
    addLog,
  } = useDeviceStore();

  const handleSaveApiUrl = (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    const formData = new FormData(e.currentTarget);
    const url = formData.get('apiUrl') as string;
    if (url) {
      setBackendApiUrl(url);
      addLog(`[${new Date().toLocaleTimeString()}] Updated backend API URL to: ${url}`);
    }
  };

  return (
    <div className="space-y-6">
      <div>
        <h2 className="text-xl font-bold text-white tracking-tight flex items-center gap-2">
          <Settings className="h-5 w-5 text-cyan-400" />
          Settings & Preferences
        </h2>
        <p className="text-xs text-slate-400">
          Application configuration, privacy toggles, and backend connection settings
        </p>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        {/* Privacy & Hardware Security Settings */}
        <Card>
          <CardHeader>
            <CardTitle className="flex items-center gap-2">
              <KeyRound className="h-4 w-4 text-cyan-400" />
              Privacy & Telemetry Controls
            </CardTitle>
            <CardDescription>
              Control device identifier extraction and privacy safeguards
            </CardDescription>
          </CardHeader>

          <CardContent className="space-y-4">
            <div className="flex items-center justify-between p-3 rounded-lg border border-slate-800 bg-slate-950/40">
              <div className="space-y-0.5">
                <span className="text-xs font-semibold text-white block">
                  Serial Number Extraction
                </span>
                <span className="text-[11px] text-slate-400 block max-w-sm">
                  Extract hardware serial numbers only when explicitly authorized by the servicing technician.
                </span>
              </div>

              <label className="relative inline-flex items-center cursor-pointer">
                <input
                  type="checkbox"
                  checked={allowSerialNumber}
                  onChange={(e) => setAllowSerialNumber(e.target.checked)}
                  className="sr-only peer"
                />
                <div className="w-11 h-6 bg-slate-800 peer-focus:outline-none rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-slate-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-cyan-600" />
              </label>
            </div>
          </CardContent>
        </Card>

        {/* Backend API Connection */}
        <Card>
          <CardHeader>
            <CardTitle className="flex items-center gap-2">
              <Server className="h-4 w-4 text-cyan-400" />
              Backend REST Engine API
            </CardTitle>
            <CardDescription>
              Local Rust Axum API endpoint for desktop bridge and persistent logging
            </CardDescription>
          </CardHeader>

          <CardContent>
            <form onSubmit={handleSaveApiUrl} className="space-y-3">
              <div className="space-y-1">
                <label className="text-xs font-medium text-slate-300">API Host URL</label>
                <input
                  type="text"
                  name="apiUrl"
                  defaultValue={backendApiUrl}
                  className="w-full rounded-lg border border-slate-700 bg-slate-950 px-3 py-2 text-xs text-white focus:outline-none focus:border-cyan-500 font-mono"
                />
              </div>

              <Button type="submit" variant="outline" size="sm">
                Save API Endpoint
              </Button>
            </form>
          </CardContent>
        </Card>

        {/* Legal & Compliance Notice */}
        <Card className="lg:col-span-2">
          <CardHeader>
            <CardTitle className="flex items-center gap-2">
              <ShieldCheck className="h-4 w-4 text-emerald-400" />
              Legal & Compliance Framework
            </CardTitle>
            <CardDescription>
              Adherence to Right to Repair and legitimate servicing regulations
            </CardDescription>
          </CardHeader>

          <CardContent className="space-y-2 text-xs text-slate-400 leading-relaxed">
            <p>
              Universal Android Tool (UAT) is designed strictly for authorized electronics technicians and verified device owners.
              In compliance with international electronics repair policies, UAT does NOT provide or facilitate:
            </p>
            <ul className="list-disc list-inside space-y-1 pl-2 text-slate-300">
              <li>Unauthorized FRP (Factory Reset Protection) circumvention</li>
              <li>Google account authentication bypasses or credential dumping</li>
              <li>Unauthorized pattern/PIN screen-lock removal without data wipe</li>
              <li>Malicious bootloader exploits or persistent backdoors</li>
            </ul>
          </CardContent>
        </Card>
      </div>
    </div>
  );
}
