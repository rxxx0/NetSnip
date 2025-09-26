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
    <div className="neu-card-hover p-6 text-center transition-all">
      <div className="mb-3">
        <div
          className={`w-16 h-16 rounded-2xl mx-auto flex items-center justify-center ${pulse ? 'pulse-soft' : ''}`}
          style={{
            background: `linear-gradient(145deg, ${color}22, ${color}33)`,
          }}
        >
          <div
            className="w-8 h-8 rounded-lg"
            style={{
              background: `linear-gradient(145deg, ${color}, ${color}dd)`,
            }}
          />
        </div>
      </div>
      <p className="text-3xl font-bold text-text-primary mb-1">
        {value}
      </p>
      <p className="text-sm font-medium text-text-secondary">
        {label}
      </p>
    </div>
  );

  return (
    <div className="mb-8">
      <div className="mb-6">
        <h2 className="text-xl font-semibold text-text-primary mb-2">
          Network Overview
        </h2>
        <p className="text-sm text-text-secondary">
          {networkInfo ? (
            <span className="font-mono">
              {networkInfo.interfaceName} • {networkInfo.localIp}
            </span>
          ) : (
            scanning ? 'Scanning network...' : 'No network info available'
          )}
        </p>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6 stagger-children">
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
        <div className="mt-6 neu-card p-4 animate-slide-up">
          <div className="flex items-center gap-4">
            <div className="radar-sweep scale-75"></div>
            <div>
              <p className="font-medium text-text-primary">
                Scanning network...
              </p>
              <p className="text-sm text-text-secondary mt-1">
                Discovering devices on your network
              </p>
            </div>
          </div>
        </div>
      )}

      {/* Warning for blocked devices */}
      {stats.blockedDevices > 0 && !scanning && (
        <div className="mt-6 neu-card p-4 animate-fade-in" style={{
          background: 'linear-gradient(145deg, var(--accent-danger)11, var(--accent-danger)22)',
        }}>
          <div className="flex items-center gap-3">
            <div className="w-2 h-2 rounded-full bg-red-500"></div>
            <p className="font-medium text-text-primary">
              {stats.blockedDevices} device{stats.blockedDevices !== 1 ? 's' : ''} blocked
            </p>
          </div>
        </div>
      )}
    </div>
  );
};