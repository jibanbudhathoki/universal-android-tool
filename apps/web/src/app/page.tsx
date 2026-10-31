'use client';

import React, { useEffect } from 'react';
import { useDeviceStore } from '@/store/deviceStore';
import { Header } from '@/components/layout/Header';
import { Sidebar } from '@/components/layout/Sidebar';
import { DashboardView } from '@/components/views/DashboardView';
import { DeviceManagerView } from '@/components/views/DeviceManagerView';
import { FrpRecoveryView } from '@/components/views/FrpRecoveryView';
import { ScreenRecoveryView } from '@/components/views/ScreenRecoveryView';
import { DeviceDiagnosticsView } from '@/components/views/DeviceDiagnosticsView';
import { FirmwareManagerView } from '@/components/views/FirmwareManagerView';
import { UsbConnectionView } from '@/components/views/UsbConnectionView';
import { OperationHistoryView } from '@/components/views/OperationHistoryView';
import { SettingsView } from '@/components/views/SettingsView';
import { AlertTriangle } from 'lucide-react';

export default function AppMain() {
  const { init, activeTab, webUsbSupported } = useDeviceStore();

  useEffect(() => {
    init();
  }, [init]);

  const renderActiveView = () => {
    switch (activeTab) {
      case 'dashboard':
        return <DashboardView />;
      case 'device-manager':
        return <DeviceManagerView />;
      case 'frp-recovery':
        return <FrpRecoveryView />;
      case 'screen-recovery':
        return <ScreenRecoveryView />;
      case 'diagnostics':
        return <DeviceDiagnosticsView />;
      case 'firmware':
        return <FirmwareManagerView />;
      case 'usb-connection':
        return <UsbConnectionView />;
      case 'history':
        return <OperationHistoryView />;
      case 'settings':
        return <SettingsView />;
      default:
        return <DashboardView />;
    }
  };

  return (
    <div className="flex h-screen w-screen flex-col bg-slate-950 text-slate-100 overflow-hidden select-none">
      {/* Top Header */}
      <Header />

      {/* Main Workspace with Fixed Sidebar and Scrollable Content */}
      <div className="flex flex-1 overflow-hidden">
        {/* Navigation Sidebar */}
        <Sidebar />

        {/* Dynamic Content View Area */}
        <main className="flex-1 overflow-y-auto p-6 md:p-8 space-y-6">
          {/* Browser Unsupported Warning Banner */}
          {webUsbSupported === false && (
            <div className="rounded-xl border border-amber-500/30 bg-amber-500/10 p-4 text-amber-200">
              <div className="flex items-start space-x-3">
                <AlertTriangle className="h-5 w-5 text-amber-400 mt-0.5 flex-shrink-0" />
                <div className="text-xs space-y-1">
                  <h4 className="font-semibold text-amber-300">
                    WebUSB API is not enabled in this browser
                  </h4>
                  <p className="text-amber-200/90 leading-relaxed">
                    Firefox and Safari do not support direct USB device communication standards. To service devices via browser USB,
                    please launch with a Chromium-based browser (Chrome, Edge, Brave, Opera) or use the native <strong>Universal Android Tool Desktop</strong> app.
                  </p>
                </div>
              </div>
            </div>
          )}

          {/* Active Servicing View */}
          <div className="max-w-7xl mx-auto w-full pb-12">
            {renderActiveView()}
          </div>
        </main>
      </div>
    </div>
  );
}
