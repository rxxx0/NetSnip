import React, { useMemo } from 'react';
import { useNetworkStore } from '../../stores/networkStore';

export const Dashboard: React.FC = () => {
  const { devices, networkInfo, scanning } = useNetworkStore();

  const stats = useMemo(() => {
    const online = devices.filter(d => d.status === 'online').length;
    const blocked = devices.filter(d => d.status === 'blocked').length;
    const limited = devices.filter(d => d.status === 'limited').length;
    const totalBandwidth = devices.reduce((acc, d) => acc + d.bandwidthCurrent, 0);

    return {
      totalDevices: devices.length,
      onlineDevices: online,
      blockedDevices: blocked,
      limitedDevices: limited,
      totalBandwidth: totalBandwidth.toFixed(1),
    };
  }, [devices]);

  const StatCard: React.FC<{
    label: string;
    value: string | number;
    color?: string;
    pulse?: boolean;
  }> = ({ label, value, color = 'var(--accent-primary)', pulse = false }) => (
    <div className="neu-card-hover p-4 text-center transition-all">
      <div className="mb-2">
        <div
          className={`w-10 h-10 rounded-lg mx-auto flex items-center justify-center ${pulse ? 'pulse-soft' : ''}`}
          style={{
            background: `${color}15`,
            border: `1px solid ${color}30`,
          }}
        >
          <div
            className="w-5 h-5 rounded"
            style={{
              background: color,
            }}
          />
        </div>
      </div>
      <p className="text-2xl font-bold text-text-primary mb-0.5">
        {value}
      </p>
      <p className="text-xs font-medium text-text-secondary">
        {label}
      </p>
    </div>
  );

  return (
    <div className="mb-6">
      <div className="mb-4">
        <h2 className="text-lg font-semibold text-text-primary mb-1">
          Network Overview
        </h2>
        <p className="text-xs text-text-secondary">
          {networkInfo ? (
            <span className="font-mono">
              {networkInfo.interfaceName} • {networkInfo.localIp}
            </span>
          ) : (
            scanning ? 'Scanning network...' : 'No network info available'
          )}
        </p>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4 stagger-children">
        <StatCard
          label="Total Devices"
          value={stats.totalDevices}
          color="var(--accent-primary)"
        />

        <StatCard
          label="Online Now"
          value={stats.onlineDevices}
          color="var(--accent-success)"
          pulse={stats.onlineDevices > 0}
        />

        <StatCard
          label="Blocked"
          value={stats.blockedDevices}
          color="var(--accent-danger)"
        />

        <StatCard
          label="Bandwidth"
          value={`${stats.totalBandwidth} MB/s`}
          color="var(--accent-warning)"
        />
      </div>

      {/* Network Status Banner */}
      {scanning && (
        <div className="mt-4 neu-card p-3 animate-slide-up">
          <div className="flex items-center gap-3">
            <div className="radar-sweep scale-50"></div>
            <div>
              <p className="text-sm font-medium text-text-primary">
                Scanning network...
              </p>
              <p className="text-xs text-text-secondary">
                Discovering devices on your network
              </p>
            </div>
          </div>
        </div>
      )}

      {/* Warning for blocked devices */}
      {stats.blockedDevices > 0 && !scanning && (
        <div className="mt-4 neu-card p-3 animate-fade-in" style={{
          background: 'rgba(239, 68, 68, 0.05)',
          borderColor: 'rgba(239, 68, 68, 0.2)',
        }}>
          <div className="flex items-center gap-2">
            <div className="w-1.5 h-1.5 rounded-full bg-red-500"></div>
            <p className="text-sm font-medium text-text-primary">
              {stats.blockedDevices} device{stats.blockedDevices !== 1 ? 's' : ''} blocked
            </p>
          </div>
        </div>
      )}
    </div>
  );
};