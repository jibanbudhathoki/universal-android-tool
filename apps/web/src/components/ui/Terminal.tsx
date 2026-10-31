import React, { useEffect, useRef } from 'react';
import { Terminal as TerminalIcon, Trash2 } from 'lucide-react';
import { Button } from './Button';

interface TerminalProps {
  logs: string[];
  onClear?: () => void;
  title?: string;
  className?: string;
  maxHeight?: string;
}

export function Terminal({
  logs,
  onClear,
  title = 'Real-Time Servicing Console',
  className = '',
  maxHeight = 'h-64',
}: TerminalProps) {
  const scrollRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    if (scrollRef.current) {
      scrollRef.current.scrollTop = scrollRef.current.scrollHeight;
    }
  }, [logs]);

  return (
    <div className={`flex flex-col rounded-xl border border-slate-800 bg-slate-950 font-mono text-xs shadow-inner ${className}`}>
      {/* Terminal Title Bar */}
      <div className="flex items-center justify-between border-b border-slate-800/80 bg-slate-900/80 px-4 py-2.5">
        <div className="flex items-center space-x-2">
          <TerminalIcon className="h-4 w-4 text-cyan-400" />
          <span className="font-semibold text-slate-300 text-xs">{title}</span>
          <span className="flex h-2 w-2 relative">
            <span className="animate-ping absolute inline-flex h-full w-full rounded-full bg-emerald-400 opacity-75" />
            <span className="relative inline-flex rounded-full h-2 w-2 bg-emerald-500" />
          </span>
        </div>

        <div className="flex items-center space-x-2">
          <span className="text-[11px] text-slate-500">{logs.length} lines</span>
          {onClear && (
            <Button
              variant="ghost"
              size="icon"
              onClick={onClear}
              className="h-6 w-6 text-slate-400 hover:text-white"
              title="Clear Console"
            >
              <Trash2 className="h-3.5 w-3.5" />
            </Button>
          )}
        </div>
      </div>

      {/* Terminal Content */}
      <div
        ref={scrollRef}
        className={`overflow-y-auto p-4 space-y-1 select-text scroll-smooth ${maxHeight}`}
      >
        {logs.length === 0 ? (
          <p className="text-slate-600 italic">No output logged yet. Waiting for operations...</p>
        ) : (
          logs.map((log, index) => {
            const isError = log.includes('[ERROR]') || log.toLowerCase().includes('fail');
            const isWarn = log.includes('[WARN]');
            const isSuccess = log.includes('[SUCCESS]') || log.toLowerCase().includes('success');
            const isConnect = log.includes('[USB CONNECT]');

            return (
              <div
                key={index}
                className={`leading-relaxed break-words flex items-start space-x-1.5 ${
                  isError
                    ? 'text-red-400'
                    : isWarn
                    ? 'text-amber-400'
                    : isSuccess
                    ? 'text-emerald-400'
                    : isConnect
                    ? 'text-cyan-400 font-semibold'
                    : 'text-slate-300'
                }`}
              >
                <span className="text-slate-600 select-none text-[10px] pt-0.5">›</span>
                <span className="flex-1">{log}</span>
              </div>
            );
          })
        )}
      </div>
    </div>
  );
}
