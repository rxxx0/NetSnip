import React, { useMemo } from 'react';
import { Users, Shield, Activity, HardDrive, TrendingUp, AlertTriangle } from 'lucide-react';
import { useNetworkStore } from '../../stores/networkStore';

export const Dashboard: React.FC = () => {
  const { devices, networkInfo } = useNetworkStore();

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
      totalData: (totalBandwidth * 0.1).toFixed(2), // Mock calculation
    };
  }, [devices]);

  const StatCard: React.FC<{
    icon: React.ReactNode;
    label: string;
    value: string | number;
    color: string;
    trend?: number;
  }> = ({ icon, label, value, color, trend }) => (
    <div className="neu-card p-5 rounded-2xl">
      <div className="flex items-start justify-between">
        <div>
          <div className={`w-12 h-12 rounded-xl ${color} flex items-center justify-center mb-3`}>
            {icon}
          </div>
          <p className="text-3xl font-bold text-neu-text dark:text-dark-text">
            {value}
          </p>
          <p className="text-sm text-neu-text-secondary dark:text-dark-text-secondary mt-1">
            {label}
          </p>
        </div>
        {trend !== undefined && (
          <div className="flex items-center gap-1">
            <TrendingUp className={`w-4 h-4 ${trend > 0 ? 'text-green-500' : 'text-red-500'}`} />
            <span className={`text-sm font-medium ${trend > 0 ? 'text-green-500' : 'text-red-500'}`}>
              {Math.abs(trend)}%
            </span>
          </div>
        )}
      </div>
    </div>
  );

  return (
    <div className="mb-8">
      <div className="mb-6">
        <h2 className="text-xl font-semibold text-neu-text dark:text-dark-text mb-2">
          Network Overview
        </h2>
        <p className="text-sm text-neu-text-secondary dark:text-dark-text-secondary">
          {networkInfo ? `Connected to ${networkInfo.interfaceName} • ${networkInfo.localIp}` : 'Scanning network...'}
        </p>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
        <StatCard
          icon={<Users className="w-6 h-6 text-white" />}
          label="Total Devices"
          value={stats.totalDevices}
          color="bg-gradient-to-br from-blue-500 to-blue-600"
          trend={12}
        />

        <StatCard
          icon={<Shield className="w-6 h-6 text-white" />}
          label="Blocked Devices"
          value={stats.blockedDevices}
          color="bg-gradient-to-br from-red-500 to-red-600"
        />

        <StatCard
          icon={<Activity className="w-6 h-6 text-white" />}
          label="Bandwidth (MB/s)"
          value={stats.totalBandwidth}
          color="bg-gradient-to-br from-green-500 to-green-600"
          trend={-5}
        />

        <StatCard
          icon={<HardDrive className="w-6 h-6 text-white" />}
          label="Data Usage (GB)"
          value={stats.totalData}
          color="bg-gradient-to-br from-purple-500 to-purple-600"
          trend={23}
        />
      </div>

      {/* Alerts Section */}
      {stats.blockedDevices > 0 && (
        <div className="mt-6 p-4 rounded-xl bg-orange-50 dark:bg-orange-950/20 border border-orange-200 dark:border-orange-800">
          <div className="flex items-center gap-3">
            <AlertTriangle className="w-5 h-5 text-orange-600" />
            <div>
              <p className="font-medium text-orange-900 dark:text-orange-200">
                {stats.blockedDevices} device{stats.blockedDevices !== 1 ? 's' : ''} currently blocked
              </p>
              <p className="text-sm text-orange-700 dark:text-orange-300 mt-1">
                Review blocked devices to ensure network stability
              </p>
            </div>
          </div>
        </div>
      )}
    </div>
  );
};