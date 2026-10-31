import { useEffect, useState } from 'react';
import { useDesktopStore, type DesktopNavTab } from './store/desktopStore';
import {
  Smartphone,
  Usb,
  ShieldCheck,
  Activity,
  HardDriveDownload,
  Lock,
  ShieldAlert,
  History,
  Settings,
  Terminal as TerminalIcon,
  CheckCircle2,
  XCircle,
  Download,
  ExternalLink,
  Radio,
  Cpu,
  Layers,
  KeyRound,
  Trash2,
  AlertTriangle,
} from 'lucide-react';

export default function App() {
  const {
    devices,
    selectedDeviceId,
    systemInfo,
    activeTab,
    activeOperation,
    operations,
    firmwareList,
    realTimeLogs,
    allowSerialNumber,
    init,
    connectNativeDevice,
    disconnectDevice,
    selectDevice,
    setTab,
    setAllowSerialNumber,
    runDiagnostics,
    clearLogs,
  } = useDesktopStore();

  const [attachModalOpen, setAttachModalOpen] = useState(false);
  const [selectedPreset, setSelectedPreset] = useState<'samsung' | 'pixel' | 'xiaomi'>('samsung');

  useEffect(() => {
    init();
  }, [init]);

  const selectedDevice = devices.find((d) => d.id === selectedDeviceId);

  const handleAttachPreset = async () => {
    if (selectedPreset === 'samsung') {
      await connectNativeDevice(1204, 26720, 'Samsung', 'Galaxy S23 (SM-S911B)');
    } else if (selectedPreset === 'pixel') {
      await connectNativeDevice(6353, 20042, 'Google Pixel', 'Pixel 8 (shiba)');
    } else if (selectedPreset === 'xiaomi') {
      await connectNativeDevice(10991, 36930, 'Xiaomi', 'Xiaomi 13 (fuxi)');
    }
    setAttachModalOpen(false);
  };

  const handleExportReport = () => {
    if (!selectedDevice) return;
    const report = {
      title: 'Universal Android Tool (Desktop) - Servicing Report',
      timestamp: new Date().toISOString(),
      system: systemInfo,
      device: selectedDevice,
      operations,
    };
    const blob = new Blob([JSON.stringify(report, null, 2)], { type: 'application/json' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = `UAT-Desktop-${selectedDevice.manufacturer}-${selectedDevice.model}-${Date.now()}.json`;
    a.click();
    URL.revokeObjectURL(url);
  };

  const navItems: { id: DesktopNavTab; label: string; icon: React.ElementType }[] = [
    { id: 'dashboard', label: 'Dashboard', icon: Layers },
    { id: 'device-manager', label: 'Device Manager', icon: Smartphone },
    { id: 'frp-recovery', label: 'FRP Recovery', icon: ShieldAlert },
    { id: 'screen-recovery', label: 'Screen Recovery', icon: Lock },
    { id: 'diagnostics', label: 'Device Diagnostics', icon: Activity },
    { id: 'firmware', label: 'Firmware Manager', icon: HardDriveDownload },
    { id: 'usb-connection', label: 'USB Connection', icon: Usb },
    { id: 'history', label: 'Operation History', icon: History },
    { id: 'settings', label: 'Settings', icon: Settings },
  ];

  return (
    <div className="flex h-screen w-screen flex-col bg-slate-950 text-slate-100 overflow-hidden select-none font-sans">
      {/* Top Header */}
      <header className="flex items-center justify-between border-b border-slate-800/80 bg-slate-900/80 px-6 py-3 backdrop-blur-md">
        <div className="flex items-center space-x-3">
          <div className="flex h-9 w-9 items-center justify-center rounded-lg bg-gradient-to-tr from-cyan-600 to-blue-600 shadow-md shadow-cyan-500/20">
            <Smartphone className="h-5 w-5 text-white" />
          </div>
          <div>
            <div className="flex items-center space-x-2">
              <span className="font-bold text-white text-base tracking-tight">Universal Android Tool</span>
              <span className="rounded bg-cyan-500/10 px-2 py-0.5 text-[10px] font-mono text-cyan-400 border border-cyan-500/30">
                Desktop Edition
              </span>
            </div>
            <p className="text-[11px] text-slate-400">
              Host: {systemInfo?.os ?? 'detecting...'} ({systemInfo?.arch ?? 'arch'}) | {systemInfo?.driver_stack}
            </p>
          </div>
        </div>

        <div className="flex items-center space-x-3">
          <div className="hidden md:flex items-center space-x-1.5 text-xs text-slate-400 bg-slate-950 border border-slate-800 px-3 py-1.5 rounded-lg">
            <ShieldCheck className="h-4 w-4 text-emerald-400" />
            <span>Zero-Bypass Policy Enforced</span>
          </div>

          {selectedDevice ? (
            <button
              onClick={() => disconnectDevice(selectedDevice.id)}
              className="px-3 py-1.5 text-xs font-medium text-red-400 border border-red-500/30 bg-red-950/20 hover:bg-red-950/40 rounded-lg transition"
            >
              Disconnect
            </button>
          ) : (
            <button
              onClick={() => setAttachModalOpen(true)}
              className="px-3 py-1.5 text-xs font-medium text-white bg-cyan-600 hover:bg-cyan-500 rounded-lg transition shadow-sm shadow-cyan-600/30 flex items-center gap-1.5"
            >
              <Usb className="h-3.5 w-3.5" />
              Scan / Attach USB
            </button>
          )}
        </div>
      </header>

      {/* Main Container */}
      <div className="flex flex-1 overflow-hidden">
        {/* Sidebar */}
        <aside className="w-64 border-r border-slate-800/80 bg-slate-950/80 flex flex-col justify-between p-4 flex-shrink-0">
          <div className="space-y-1">
            <div className="px-3 py-1.5 text-[10px] font-semibold uppercase tracking-wider text-slate-500">
              Servicing Modules
            </div>
            {navItems.map((item) => {
              const Icon = item.icon;
              const isActive = activeTab === item.id;
              return (
                <button
                  key={item.id}
                  onClick={() => setTab(item.id)}
                  className={`w-full flex items-center justify-between px-3 py-2 rounded-lg text-xs font-medium transition ${
                    isActive
                      ? 'bg-cyan-500/10 text-cyan-400 border border-cyan-500/20 shadow-sm'
                      : 'text-slate-400 hover:text-slate-100 hover:bg-slate-900/60'
                  }`}
                >
                  <div className="flex items-center space-x-2.5">
                    <Icon className={`h-4 w-4 ${isActive ? 'text-cyan-400' : 'text-slate-500'}`} />
                    <span>{item.label}</span>
                  </div>
                </button>
              );
            })}
          </div>

          {/* Active Device Widget */}
          <div className="pt-4 border-t border-slate-800/80">
            <div className="rounded-xl border border-slate-800 bg-slate-900/40 p-3 space-y-1.5">
              <div className="flex items-center justify-between">
                <span className="text-[10px] font-semibold uppercase tracking-wider text-slate-500">
                  Target Device
                </span>
                <span
                  className={`h-2 w-2 rounded-full ${selectedDevice ? 'bg-emerald-400 animate-pulse' : 'bg-slate-600'}`}
                />
              </div>
              {selectedDevice ? (
                <div>
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

        {/* Dynamic Views */}
        <main className="flex-1 overflow-y-auto p-6 space-y-6">
          {/* Top Metrics Banner */}
          <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">
            <div className="rounded-xl border border-slate-800 bg-slate-900/60 p-4 flex items-center justify-between">
              <div>
                <span className="text-[11px] font-semibold uppercase tracking-wider text-slate-500">Devices</span>
                <div className="text-2xl font-bold text-white">{devices.length}</div>
                <p className="text-[11px] text-slate-400">Native USB Session</p>
              </div>
              <div className="h-10 w-10 rounded-xl bg-blue-500/10 border border-blue-500/20 flex items-center justify-center text-blue-400">
                <Smartphone className="h-5 w-5" />
              </div>
            </div>

            <div className="rounded-xl border border-slate-800 bg-slate-900/60 p-4 flex items-center justify-between">
              <div>
                <span className="text-[11px] font-semibold uppercase tracking-wider text-slate-500">USB Subsystem</span>
                <div className="text-2xl font-bold text-white">NATIVE</div>
                <p className="text-[11px] text-slate-400">Direct Kernel I/O</p>
              </div>
              <div className="h-10 w-10 rounded-xl bg-cyan-500/10 border border-cyan-500/20 flex items-center justify-center text-cyan-400">
                <Usb className="h-5 w-5" />
              </div>
            </div>

            <div className="rounded-xl border border-slate-800 bg-slate-900/60 p-4 flex items-center justify-between">
              <div>
                <span className="text-[11px] font-semibold uppercase tracking-wider text-slate-500">Diagnostics</span>
                <div className="text-2xl font-bold text-white">
                  {activeOperation ? 'RUNNING' : 'IDLE'}
                </div>
                <p className="text-[11px] text-slate-400">
                  {activeOperation ? `${activeOperation.progress}% complete` : 'Ready to probe'}
                </p>
              </div>
              <div className="h-10 w-10 rounded-xl bg-emerald-500/10 border border-emerald-500/20 flex items-center justify-center text-emerald-400">
                <Activity className="h-5 w-5" />
              </div>
            </div>

            <div className="rounded-xl border border-slate-800 bg-slate-900/60 p-4 flex items-center justify-between">
              <div>
                <span className="text-[11px] font-semibold uppercase tracking-wider text-slate-500">Compliance</span>
                <div className="text-2xl font-bold text-emerald-400">AUTHORIZED</div>
                <p className="text-[11px] text-slate-400">Legitimate Servicing</p>
              </div>
              <div className="h-10 w-10 rounded-xl bg-emerald-500/10 border border-emerald-500/20 flex items-center justify-center text-emerald-400">
                <ShieldCheck className="h-5 w-5" />
              </div>
            </div>
          </div>

          {/* Active Operation Progress Bar */}
          {activeOperation && (
            <div className="rounded-xl border border-cyan-500/30 bg-cyan-950/20 p-4 space-y-2">
              <div className="flex items-center justify-between">
                <div className="flex items-center space-x-2">
                  <Radio className="h-4 w-4 text-cyan-400 animate-pulse" />
                  <span className="text-xs font-semibold text-white">Diagnostic Operation Active</span>
                </div>
                <span className="text-xs font-mono font-bold text-cyan-400">{activeOperation.progress}%</span>
              </div>
              <div className="h-2 w-full overflow-hidden rounded-full bg-slate-800">
                <div
                  className="h-full bg-cyan-500 transition-all duration-300"
                  style={{ width: `${activeOperation.progress}%` }}
                />
              </div>
              <p className="text-[11px] font-mono text-slate-400 truncate">{activeOperation.currentMessage}</p>
            </div>
          )}

          {/* TAB 1: DASHBOARD */}
          {activeTab === 'dashboard' && (
            <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
              <div className="lg:col-span-2 space-y-6">
                {/* Connection Panel */}
                <div className="rounded-xl border border-slate-800 bg-slate-900/60 p-5 space-y-4">
                  <div className="flex items-center justify-between border-b border-slate-800 pb-3">
                    <div>
                      <h3 className="text-sm font-semibold text-white flex items-center gap-2">
                        <Smartphone className="h-4 w-4 text-cyan-400" />
                        Device Connection Panel
                      </h3>
                      <p className="text-xs text-slate-400">Native USB hardware attributes and protocol status</p>
                    </div>

                    {selectedDevice ? (
                      <button
                        onClick={() => runDiagnostics(selectedDevice.id)}
                        disabled={Boolean(activeOperation)}
                        className="px-3 py-1.5 text-xs font-medium text-cyan-400 border border-cyan-500/30 bg-cyan-950/30 hover:bg-cyan-950/50 rounded-lg transition flex items-center gap-1.5"
                      >
                        <Activity className="h-3.5 w-3.5" />
                        Run Diagnostic
                      </button>
                    ) : (
                      <button
                        onClick={() => setAttachModalOpen(true)}
                        className="px-3 py-1.5 text-xs font-medium text-white bg-cyan-600 hover:bg-cyan-500 rounded-lg transition"
                      >
                        Attach Device
                      </button>
                    )}
                  </div>

                  {selectedDevice ? (
                    <div className="space-y-4">
                      <div className="grid grid-cols-2 sm:grid-cols-4 gap-3 text-xs">
                        <div className="rounded-lg border border-slate-800 bg-slate-950/60 p-3">
                          <span className="text-[10px] text-slate-500 block uppercase">Vendor ID</span>
                          <span className="font-mono text-white text-sm font-semibold">
                            0x{selectedDevice.vendor_id.toString(16).padStart(4, '0')}
                          </span>
                        </div>
                        <div className="rounded-lg border border-slate-800 bg-slate-950/60 p-3">
                          <span className="text-[10px] text-slate-500 block uppercase">Product ID</span>
                          <span className="font-mono text-white text-sm font-semibold">
                            0x{selectedDevice.product_id.toString(16).padStart(4, '0')}
                          </span>
                        </div>
                        <div className="rounded-lg border border-slate-800 bg-slate-950/60 p-3">
                          <span className="text-[10px] text-slate-500 block uppercase">Mode</span>
                          <span className="font-mono text-cyan-400 text-sm font-semibold uppercase">
                            {selectedDevice.connection_mode}
                          </span>
                        </div>
                        <div className="rounded-lg border border-slate-800 bg-slate-950/60 p-3">
                          <span className="text-[10px] text-slate-500 block uppercase">Authorization</span>
                          <span className="text-amber-400 text-sm font-semibold">Standard</span>
                        </div>
                      </div>

                      <div className="rounded-lg border border-slate-800 bg-slate-950/40 p-3 flex items-center justify-between text-xs">
                        <div className="flex items-center space-x-2">
                          <KeyRound className="h-4 w-4 text-slate-400" />
                          <span className="text-slate-300">
                            Serial Extraction:{' '}
                            {allowSerialNumber && selectedDevice.serial_number ? (
                              <strong className="font-mono text-cyan-300">{selectedDevice.serial_number}</strong>
                            ) : (
                              <span className="text-slate-500 italic">Protected</span>
                            )}
                          </span>
                        </div>
                        <button
                          onClick={() => setAllowSerialNumber(!allowSerialNumber)}
                          className="text-xs text-slate-400 hover:text-white"
                        >
                          {allowSerialNumber ? 'Hide' : 'Authorize'}
                        </button>
                      </div>
                    </div>
                  ) : (
                    <div className="py-8 text-center space-y-2">
                      <Smartphone className="h-8 w-8 text-slate-600 mx-auto" />
                      <p className="text-xs text-slate-400">No USB device attached. Click "Attach Device" to scan.</p>
                    </div>
                  )}
                </div>

                {/* Real-Time Terminal Console */}
                <div className="rounded-xl border border-slate-800 bg-slate-950 p-4 font-mono text-xs shadow-inner space-y-2">
                  <div className="flex items-center justify-between border-b border-slate-800 pb-2">
                    <div className="flex items-center space-x-2">
                      <TerminalIcon className="h-4 w-4 text-cyan-400" />
                      <span className="font-semibold text-slate-300">Desktop Servicing Console</span>
                    </div>
                    <button onClick={clearLogs} className="text-slate-400 hover:text-white">
                      <Trash2 className="h-3.5 w-3.5" />
                    </button>
                  </div>
                  <div className="h-56 overflow-y-auto space-y-1">
                    {realTimeLogs.map((log, i) => (
                      <div key={i} className="text-slate-300 leading-relaxed truncate">
                        <span className="text-slate-600 select-none mr-1.5">›</span>
                        {log}
                      </div>
                    ))}
                  </div>
                </div>
              </div>

              {/* Capabilities Panel */}
              <div className="space-y-6">
                <div className="rounded-xl border border-slate-800 bg-slate-900/60 p-5 space-y-4">
                  <h3 className="text-sm font-semibold text-white flex items-center gap-2 border-b border-slate-800 pb-3">
                    <Cpu className="h-4 w-4 text-cyan-400" />
                    Hardware Capabilities
                  </h3>

                  {selectedDevice ? (
                    <div className="space-y-3">
                      {[
                        { label: 'ADB Diagnostics', enabled: selectedDevice.capabilities.adb_available },
                        { label: 'Fastboot Protocol', enabled: selectedDevice.capabilities.fastboot_available },
                        { label: 'MTP Protocol', enabled: selectedDevice.capabilities.mtp_available },
                        { label: 'Diagnostic Mode', enabled: selectedDevice.capabilities.diagnostic_mode_supported },
                        { label: 'Recovery Mode', enabled: selectedDevice.capabilities.recovery_mode_supported },
                        { label: 'Authorized Reset', enabled: selectedDevice.capabilities.authorized_reset_supported },
                      ].map((cap, i) => (
                        <div
                          key={i}
                          className="rounded-lg border border-slate-800/80 bg-slate-950/40 p-2.5 flex items-center justify-between text-xs"
                        >
                          <span className="text-slate-300">{cap.label}</span>
                          {cap.enabled ? (
                            <CheckCircle2 className="h-4 w-4 text-emerald-400" />
                          ) : (
                            <XCircle className="h-4 w-4 text-slate-600" />
                          )}
                        </div>
                      ))}
                    </div>
                  ) : (
                    <p className="text-xs text-slate-500 italic py-6 text-center">
                      Connect hardware to inspect capabilities.
                    </p>
                  )}
                </div>
              </div>
            </div>
          )}

          {/* TAB 2: DEVICE MANAGER */}
          {activeTab === 'device-manager' && (
            <div className="rounded-xl border border-slate-800 bg-slate-900/60 p-5 space-y-4">
              <h3 className="text-sm font-semibold text-white">Discovered Devices ({devices.length})</h3>
              {devices.length === 0 ? (
                <p className="text-xs text-slate-500 italic py-6 text-center">No devices currently attached.</p>
              ) : (
                <div className="divide-y divide-slate-800">
                  {devices.map((d) => (
                    <div key={d.id} className="py-3 flex items-center justify-between">
                      <div>
                        <p className="text-sm font-medium text-white">{d.manufacturer} {d.model}</p>
                        <p className="text-xs text-slate-500 font-mono">
                          VID: 0x{d.vendor_id.toString(16)} | PID: 0x{d.product_id.toString(16)} | Mode: {d.connection_mode}
                        </p>
                      </div>
                      <div className="flex items-center space-x-2">
                        {d.id !== selectedDeviceId && (
                          <button
                            onClick={() => selectDevice(d.id)}
                            className="px-3 py-1 text-xs border border-slate-700 hover:bg-slate-800 rounded-lg"
                          >
                            Select
                          </button>
                        )}
                        <button
                          onClick={() => disconnectDevice(d.id)}
                          className="px-3 py-1 text-xs text-red-400 border border-red-500/30 hover:bg-red-950/30 rounded-lg"
                        >
                          Disconnect
                        </button>
                      </div>
                    </div>
                  ))}
                </div>
              )}
            </div>
          )}

          {/* TAB 3: FRP RECOVERY */}
          {activeTab === 'frp-recovery' && (
            <div className="space-y-4">
              <div className="rounded-xl border border-amber-500/30 bg-amber-950/20 p-4 text-xs text-amber-200">
                <h4 className="font-bold uppercase tracking-wider text-amber-300 mb-1 flex items-center gap-1.5">
                  <ShieldAlert className="h-4 w-4" />
                  Authorized FRP Servicing Protocol
                </h4>
                <p>
                  Zero-bypass enforced. Universal Android Tool facilitates legitimate Google Account and manufacturer servicing portals.
                </p>
              </div>

              <div className="rounded-xl border border-slate-800 bg-slate-900/60 p-5 space-y-3">
                <h3 className="text-sm font-semibold text-white">Official OEM Support Portals</h3>
                <div className="grid grid-cols-1 sm:grid-cols-2 gap-3 text-xs">
                  <a
                    href="https://accounts.google.com/signin/recovery"
                    target="_blank"
                    rel="noreferrer"
                    className="p-3 rounded-lg border border-slate-800 hover:border-cyan-500/40 bg-slate-950 flex justify-between items-center"
                  >
                    <span>Google Account Recovery</span>
                    <ExternalLink className="h-3 w-3 text-slate-500" />
                  </a>
                  <a
                    href="https://www.samsung.com/us/support/troubleshooting/TSG01001402/"
                    target="_blank"
                    rel="noreferrer"
                    className="p-3 rounded-lg border border-slate-800 hover:border-cyan-500/40 bg-slate-950 flex justify-between items-center"
                  >
                    <span>Samsung Official Support</span>
                    <ExternalLink className="h-3 w-3 text-slate-500" />
                  </a>
                </div>
              </div>
            </div>
          )}

          {/* TAB 4: SCREEN RECOVERY */}
          {activeTab === 'screen-recovery' && (
            <div className="space-y-4">
              <div className="rounded-xl border border-red-500/30 bg-red-950/20 p-4 text-xs text-red-200">
                <h4 className="font-bold uppercase tracking-wider text-red-300 mb-1 flex items-center gap-1.5">
                  <AlertTriangle className="h-4 w-4" />
                  Hardware Encryption & Data Loss Policy
                </h4>
                <p>
                  Screen lock removal without user data erasure is impossible on modern Android. Legitimate recovery involves wiping local data in Stock Recovery.
                </p>
              </div>

              <div className="rounded-xl border border-slate-800 bg-slate-900/60 p-5 space-y-3 text-xs">
                <h3 className="text-sm font-semibold text-white">Hardware Key Combination Guide</h3>
                <div className="p-3 rounded-lg bg-slate-950 border border-slate-800 font-mono text-cyan-300">
                  {selectedDevice?.manufacturer === 'Samsung'
                    ? 'Hold [Volume Up] + [Side/Power] with USB connected until Samsung Recovery appears.'
                    : selectedDevice?.manufacturer === 'Google Pixel'
                    ? 'Power off. Hold [Power] + [Volume Down] to Fastboot, then select Recovery Mode.'
                    : 'Power off. Hold [Volume Down] + [Power] or [Volume Up] + [Power] until stock recovery appears.'}
                </div>
              </div>
            </div>
          )}

          {/* TAB 5: DIAGNOSTICS */}
          {activeTab === 'diagnostics' && (
            <div className="rounded-xl border border-slate-800 bg-slate-900/60 p-5 space-y-4">
              <div className="flex items-center justify-between">
                <div>
                  <h3 className="text-sm font-semibold text-white">Hardware Diagnostic Engine</h3>
                  <p className="text-xs text-slate-400">Non-destructive USB descriptor and endpoint latency test</p>
                </div>
                <div className="flex space-x-2">
                  <button
                    onClick={() => selectedDevice && runDiagnostics(selectedDevice.id)}
                    disabled={!selectedDevice || Boolean(activeOperation)}
                    className="px-3 py-1.5 text-xs font-medium text-white bg-cyan-600 hover:bg-cyan-500 rounded-lg disabled:opacity-50"
                  >
                    Run Test
                  </button>
                  <button
                    onClick={handleExportReport}
                    disabled={!selectedDevice}
                    className="px-3 py-1.5 text-xs font-medium text-slate-300 border border-slate-700 hover:bg-slate-800 rounded-lg flex items-center gap-1.5"
                  >
                    <Download className="h-3.5 w-3.5" />
                    Export Report
                  </button>
                </div>
              </div>
            </div>
          )}

          {/* TAB 6: FIRMWARE */}
          {activeTab === 'firmware' && (
            <div className="space-y-4">
              <h3 className="text-sm font-semibold text-white">Official Firmware Catalog ({firmwareList.length})</h3>
              <div className="space-y-3">
                {firmwareList.map((fw) => (
                  <div key={fw.id} className="p-4 rounded-xl border border-slate-800 bg-slate-900/60 flex justify-between items-center text-xs">
                    <div>
                      <p className="font-bold text-white text-sm">{fw.model}</p>
                      <p className="text-slate-400 font-mono">Build: {fw.build_id} | Android {fw.android_version} ({fw.region})</p>
                    </div>
                    <span className="font-mono text-cyan-400 text-[10px] bg-slate-950 px-2 py-1 rounded border border-slate-800">
                      SHA256 Verified
                    </span>
                  </div>
                ))}
              </div>
            </div>
          )}

          {/* TAB 7: USB CONNECTION */}
          {activeTab === 'usb-connection' && (
            <div className="rounded-xl border border-slate-800 bg-slate-900/60 p-5 space-y-4 text-xs">
              <h3 className="text-sm font-semibold text-white">Native USB Driver Subsystem</h3>
              <div className="p-4 rounded-lg bg-slate-950 border border-slate-800 space-y-2">
                <p className="text-slate-300">
                  Operating System: <strong className="text-white">{systemInfo?.os}</strong>
                </p>
                <p className="text-slate-300">
                  Architecture: <strong className="text-white">{systemInfo?.arch}</strong>
                </p>
                <p className="text-slate-300">
                  Active Driver Stack: <strong className="text-cyan-400">{systemInfo?.driver_stack}</strong>
                </p>
              </div>
            </div>
          )}

          {/* TAB 8: HISTORY */}
          {activeTab === 'history' && (
            <div className="rounded-xl border border-slate-800 bg-slate-900/60 p-5 space-y-4 text-xs">
              <h3 className="text-sm font-semibold text-white">Session Audit History ({operations.length})</h3>
              {operations.length === 0 ? (
                <p className="text-slate-500 italic py-6 text-center">No operations recorded yet.</p>
              ) : (
                <div className="space-y-3">
                  {operations.map((op) => (
                    <div key={op.id} className="p-3 rounded-lg border border-slate-800 bg-slate-950 space-y-1">
                      <div className="flex justify-between items-center">
                        <strong className="text-white">{op.operation_type}</strong>
                        <span className="text-emerald-400 uppercase text-[10px] font-mono">{op.status}</span>
                      </div>
                      <p className="text-slate-400 text-[11px] font-mono truncate">{op.logs[op.logs.length - 1]}</p>
                    </div>
                  ))}
                </div>
              )}
            </div>
          )}

          {/* TAB 9: SETTINGS */}
          {activeTab === 'settings' && (
            <div className="rounded-xl border border-slate-800 bg-slate-900/60 p-5 space-y-4 text-xs">
              <h3 className="text-sm font-semibold text-white">Desktop Application Settings</h3>
              <div className="flex items-center justify-between p-3 rounded-lg bg-slate-950 border border-slate-800">
                <span>Serial Number Extraction Authorization</span>
                <input
                  type="checkbox"
                  checked={allowSerialNumber}
                  onChange={(e) => setAllowSerialNumber(e.target.checked)}
                  className="rounded border-slate-700 text-cyan-600 focus:ring-cyan-500"
                />
              </div>
            </div>
          )}
        </main>
      </div>

      {/* Attach Preset Dialog Modal */}
      {attachModalOpen && (
        <div className="fixed inset-0 z-50 flex items-center justify-center p-4 bg-slate-950/80 backdrop-blur-sm">
          <div className="w-full max-w-md rounded-2xl border border-slate-800 bg-slate-900 p-6 space-y-4">
            <h3 className="text-base font-bold text-white">Attach USB Device Interface</h3>
            <p className="text-xs text-slate-400">
              Select a hardware profile descriptor to register with the native device engine:
            </p>

            <div className="space-y-2 text-xs">
              {[
                { id: 'samsung', label: 'Samsung Galaxy S23 (0x04e8:0x6860)', mfg: 'Samsung' },
                { id: 'pixel', label: 'Google Pixel 8 (0x18d1:0x4ee2)', mfg: 'Google Pixel' },
                { id: 'xiaomi', label: 'Xiaomi 13 fuxi (0x2717:0x9042)', mfg: 'Xiaomi' },
              ].map((p) => (
                <label
                  key={p.id}
                  className={`flex items-center space-x-3 p-3 rounded-lg border cursor-pointer transition ${
                    selectedPreset === p.id
                      ? 'border-cyan-500/50 bg-cyan-950/20 text-white'
                      : 'border-slate-800 bg-slate-950 text-slate-400 hover:border-slate-700'
                  }`}
                >
                  <input
                    type="radio"
                    name="preset"
                    checked={selectedPreset === p.id}
                    onChange={() => setSelectedPreset(p.id as any)}
                  />
                  <span>{p.label}</span>
                </label>
              ))}
            </div>

            <div className="flex justify-end space-x-3 pt-3 border-t border-slate-800">
              <button
                onClick={() => setAttachModalOpen(false)}
                className="px-3 py-1.5 text-xs text-slate-300 border border-slate-700 rounded-lg hover:bg-slate-800"
              >
                Cancel
              </button>
              <button
                onClick={handleAttachPreset}
                className="px-4 py-1.5 text-xs text-white bg-cyan-600 hover:bg-cyan-500 rounded-lg font-medium"
              >
                Attach & Probe
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}
