import React, { useMemo } from 'react';
import { useNetworkStore } from '../../stores/networkStore';

export const Dashboard: React.FC = () => {
  const { devices, networkInfo, scanning, filter, setFilter } = useNetworkStore();

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
    isActive?: boolean;
    onClick?: () => void;
    activeColor?: string;
  }> = ({ label, value, isActive, onClick, activeColor }) => (
    <button
      onClick={onClick}
      className={`neu-card p-4 text-center transition-all cursor-pointer w-full ${
        isActive ? 'neu-pressed' : 'hover:-translate-y-0.5'
      }`}
    >
      <p className={`text-2xl font-semibold transition-colors ${
        isActive ? activeColor : 'text-text-primary'
      }`}>
        {value}
      </p>
      <p className="text-xs text-text-secondary mt-1">
        {label}
      </p>
    </button>
  );

  const handleFilterClick = (filterType: 'all' | 'blocked' | 'limited') => {
    setFilter(filter === filterType ? 'all' : filterType);
  };

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
          isActive={filter === 'all'}
          onClick={() => handleFilterClick('all')}
          activeColor="text-green-500"
        />

        <StatCard
          label="Blocked"
          value={stats.blockedDevices}
          isActive={filter === 'blocked'}
          onClick={() => handleFilterClick('blocked')}
          activeColor="text-red-500"
        />

        <StatCard
          label="Limited"
          value={stats.limitedDevices}
          isActive={filter === 'limited'}
          onClick={() => handleFilterClick('limited')}
          activeColor="text-yellow-500"
        />

        <StatCard
          label="Speed"
          value={`${stats.totalBandwidth} MB/s`}
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
    </div>
  );
};