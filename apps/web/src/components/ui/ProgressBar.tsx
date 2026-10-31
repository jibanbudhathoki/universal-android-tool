import React from 'react';
import { cn } from '@/lib/utils';

interface ProgressBarProps {
  progress: number; // 0 - 100
  label?: string;
  showPercent?: boolean;
  className?: string;
  status?: 'default' | 'success' | 'warning' | 'destructive';
}

export function ProgressBar({
  progress,
  label,
  showPercent = true,
  className,
  status = 'default',
}: ProgressBarProps) {
  const clamped = Math.min(100, Math.max(0, progress));

  const colors = {
    default: 'bg-cyan-500 shadow-sm shadow-cyan-500/50',
    success: 'bg-emerald-500 shadow-sm shadow-emerald-500/50',
    warning: 'bg-amber-500 shadow-sm shadow-amber-500/50',
    destructive: 'bg-red-500 shadow-sm shadow-red-500/50',
  };

  return (
    <div className={cn('w-full space-y-1.5', className)}>
      {(label || showPercent) && (
        <div className="flex justify-between items-center text-xs text-slate-300">
          {label && <span className="font-medium text-slate-400">{label}</span>}
          {showPercent && <span className="font-mono text-cyan-400 font-semibold">{clamped}%</span>}
        </div>
      )}
      <div className="h-2 w-full overflow-hidden rounded-full bg-slate-800">
        <div
          className={cn('h-full transition-all duration-300 ease-out', colors[status])}
          style={{ width: `${clamped}%` }}
        />
      </div>
    </div>
  );
}
