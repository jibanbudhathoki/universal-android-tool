'use client';

import React from 'react';
import { useDeviceStore } from '@/store/deviceStore';
import { Card, CardHeader, CardTitle, CardDescription, CardContent } from '../ui/Card';
import { Badge } from '../ui/Badge';
import { History } from 'lucide-react';

export function OperationHistoryView() {
  const { operations } = useDeviceStore();

  return (
    <div className="space-y-6">
      <div>
        <h2 className="text-xl font-bold text-white tracking-tight flex items-center gap-2">
          <History className="h-5 w-5 text-cyan-400" />
          Operation History & Audit Log
        </h2>
        <p className="text-xs text-slate-400">
          Chronological record of all hardware diagnostic probes, verification steps, and servicing tasks
        </p>
      </div>

      <Card>
        <CardHeader>
          <CardTitle>Session History ({operations.length})</CardTitle>
          <CardDescription>
            All servicing events executed in this browser session
          </CardDescription>
        </CardHeader>

        <CardContent>
          {operations.length === 0 ? (
            <div className="py-12 text-center space-y-3">
              <History className="h-10 w-10 text-slate-700 mx-auto" />
              <p className="text-sm font-medium text-slate-300">No Operations Recorded Yet</p>
              <p className="text-xs text-slate-500 max-w-sm mx-auto">
                Completed device diagnostics and servicing checks will automatically be recorded here.
              </p>
            </div>
          ) : (
            <div className="space-y-4">
              {operations.map((op) => (
                <div
                  key={op.id}
                  className="rounded-xl border border-slate-800 bg-slate-950/50 p-4 space-y-3"
                >
                  <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-2">
                    <div className="flex items-center space-x-2.5">
                      <span className="font-semibold text-white text-sm">{op.operation_type}</span>
                      <Badge
                        variant={
                          op.status === 'completed'
                            ? 'success'
                            : op.status === 'failed'
                            ? 'destructive'
                            : 'cyan'
                        }
                      >
                        {op.status.toUpperCase()}
                      </Badge>
                    </div>

                    <span className="text-xs font-mono text-slate-500">
                      Target: {op.device_id}
                    </span>
                  </div>

                  {/* Operation Logs Preview */}
                  <div className="rounded-lg bg-slate-900/80 p-3 font-mono text-[11px] text-slate-300 border border-slate-800/80 space-y-1">
                    {op.logs.map((log, i) => (
                      <div key={i} className="truncate">
                        {log}
                      </div>
                    ))}
                  </div>
                </div>
              ))}
            </div>
          )}
        </CardContent>
      </Card>
    </div>
  );
}
