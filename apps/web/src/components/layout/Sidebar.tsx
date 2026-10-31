'use client';

import React from 'react';
import { useDeviceStore, type NavTab } from '@/store/deviceStore';
import {
  LayoutDashboard,
  Smartphone,
  ShieldAlert,
  Lock,
  Activity,
  HardDriveDownload,
  Usb,
  History,
  Settings,
  ChevronRight,
} from 'lucide-react';
import { cn } from '@/lib/utils';

export function Sidebar() {
  const { activeTab, setTab, connectedDevices, selectedDeviceId } = useDeviceStore();
  const selectedDevice = connectedDevices.find((d) => d.id === selectedDeviceId);

  const navItems: { id: NavTab; label: string; icon: React.ElementType; badge?: string }[] = [
    { id: 'dashboard', label: 'Dashboard', icon: LayoutDashboard },
    { id: 'device-manager', label: 'Device Manager', icon: Smartphone, badge: connectedDevices.length > 0 ? `${connectedDevices.length}` : undefined },
    { id: 'frp-recovery', label: 'FRP Recovery', icon: ShieldAlert },
    { id: 'screen-recovery', label: 'Screen Recovery', icon: Lock },
    { id: 'diagnostics', label: 'Device Diagnostics', icon: Activity },
    { id: 'firmware', label: 'Firmware Manager', icon: HardDriveDownload },
    { id: 'usb-connection', label: 'USB Connection', icon: Usb },
    { id: 'history', label: 'Operation History', icon: History },
    { id: 'settings', label: 'Settings', icon: Settings },
  ];

  return (
    <aside className="w-64 border-r border-slate-800/80 bg-slate-950/60 flex flex-col justify-between p-4 flex-shrink-0 select-none">
      {/* Navigation List */}
      <div className="space-y-1">
        <div className="px-3 py-2 text-[10px] font-semibold uppercase tracking-wider text-slate-500">
          Servicing Navigation
        </div>

        {navItems.map((item) => {
          const Icon = item.icon;
          const isActive = activeTab === item.id;

          return (
            <button
              key={item.id}
              onClick={() => setTab(item.id)}
              className={cn(
                'w-full flex items-center justify-between px-3 py-2.5 rounded-lg text-xs font-medium transition-all group',
                isActive
                  ? 'bg-cyan-500/10 text-cyan-400 border border-cyan-500/20 shadow-sm'
                  : 'text-slate-400 hover:text-slate-100 hover:bg-slate-900/60'
              )}
            >
              <div className="flex items-center space-x-2.5">
                <Icon
                  className={cn(
                    'h-4 w-4 transition-colors',
                    isActive ? 'text-cyan-400' : 'text-slate-500 group-hover:text-slate-300'
                  )}
                />
                <span>{item.label}</span>
              </div>

              <div className="flex items-center space-x-1.5">
                {item.badge && (
                  <span className="rounded-full bg-cyan-950 px-2 py-0.5 text-[10px] font-mono text-cyan-400 border border-cyan-800/50">
                    {item.badge}
                  </span>
                )}
                {isActive && <ChevronRight className="h-3.5 w-3.5 text-cyan-400" />}
              </div>
            </button>
          );
        })}
      </div>

      {/* Active Device Mini-Card in Footer */}
      <div className="pt-4 border-t border-slate-800/80">
        <div className="rounded-xl border border-slate-800 bg-slate-900/40 p-3 space-y-2">
          <div className="flex items-center justify-between">
            <span className="text-[10px] font-semibold uppercase tracking-wider text-slate-500">Target Device</span>
            <span
              className={cn(
                'h-2 w-2 rounded-full',
                selectedDevice ? 'bg-emerald-400' : 'bg-slate-600'
              )}
            />
          </div>

          {selectedDevice ? (
            <div className="space-y-0.5">
              <p className="text-xs font-semibold text-white truncate">
                {selectedDevice.manufacturer} {selectedDevice.model}
              </p>
              <p className="text-[10px] font-mono text-cyan-400">
                VID: 0x{selectedDevice.vendor_id.toString(16)} | PID: 0x{selectedDevice.product_id.toString(16)}
              </p>
            </div>
          ) : (
            <p className="text-xs text-slate-500 italic">No device connected</p>
          )}
        </div>
      </div>
    </aside>
  );
}
