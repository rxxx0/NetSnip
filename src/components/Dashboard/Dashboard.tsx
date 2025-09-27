import React, { useMemo, useState, useEffect, useRef, useCallback } from "react";
import { useNetworkStore } from "../../stores/networkStore";

export const Dashboard: React.FC = () => {
  const { devices, networkInfo, scanning, filter, setFilter } =
    useNetworkStore();

  // Track previous visual values for pulse animation
  const [pulseClasses, setPulseClasses] = useState({
    total: '',
    blocked: '',
    limited: '',
    bandwidth: '',
  });

  const prevVisualValues = useRef({
    total: '',
    blocked: '',
    limited: '',
    bandwidth: '',
  });

  // Format bandwidth with dynamic units - memoized for stability
  const formatBandwidth = useCallback((mbps: number): string => {
    if (mbps >= 1000) {
      return `${(mbps / 1000).toFixed(1)} GB/s`;
    }
    return `${mbps.toFixed(1)} MB/s`;
  }, []);

  const stats = useMemo(() => {
    const online = devices.filter((d) => d.status === "online").length;
    const blocked = devices.filter((d) => d.status === "blocked").length;
    const limited = devices.filter((d) => d.status === "limited").length;
    const totalBandwidth = devices.reduce(
      (acc, d) => acc + d.bandwidthCurrent,
      0
    );

    return {
      totalDevices: devices.length,
      onlineDevices: online,
      blockedDevices: blocked,
      limitedDevices: limited,
      totalBandwidth: totalBandwidth,
    };
  }, [devices]);

  // Check for visual changes and trigger pulse animations
  useEffect(() => {
    const currentVisualValues = {
      total: stats.totalDevices.toString(),
      blocked: stats.blockedDevices.toString(),
      limited: stats.limitedDevices.toString(),
      bandwidth: formatBandwidth(stats.totalBandwidth),
    };

    const newPulseClasses = {
      total: '',
      blocked: '',
      limited: '',
      bandwidth: '',
    };

    // Only pulse if the visual value has changed
    if (prevVisualValues.current.total && prevVisualValues.current.total !== currentVisualValues.total) {
      newPulseClasses.total = 'pulse-green';
    }
    if (prevVisualValues.current.blocked && prevVisualValues.current.blocked !== currentVisualValues.blocked) {
      newPulseClasses.blocked = 'pulse-red';
    }
    if (prevVisualValues.current.limited && prevVisualValues.current.limited !== currentVisualValues.limited) {
      newPulseClasses.limited = 'pulse-yellow';
    }
    if (prevVisualValues.current.bandwidth && prevVisualValues.current.bandwidth !== currentVisualValues.bandwidth) {
      newPulseClasses.bandwidth = 'pulse-blue';
    }

    setPulseClasses(newPulseClasses);
    prevVisualValues.current = currentVisualValues;

    // Clear pulse classes after animation completes
    const timer = setTimeout(() => {
      setPulseClasses({
        total: '',
        blocked: '',
        limited: '',
        bandwidth: '',
      });
    }, 600);

    return () => clearTimeout(timer);
  }, [stats, formatBandwidth]);

  const StatCard: React.FC<{
    label: string;
    value: string | number;
    isActive?: boolean;
    onClick?: () => void;
    activeColor?: string;
    pulseClass?: string;
  }> = ({ label, value, isActive, onClick, activeColor, pulseClass }) => (
    <button
      onClick={onClick}
      className={`neu-card p-4 text-center transition-all cursor-pointer w-full ${
        isActive ? "neu-pressed" : "hover:-translate-y-0.5"
      }`}
    >
      <p
        className={`text-2xl font-semibold transition-colors ${
          isActive ? activeColor : "text-text-primary"
        } ${pulseClass || ''}`}
      >
        {value}
      </p>
      <p className="text-xs text-text-secondary mt-1">{label}</p>
    </button>
  );

  const handleFilterClick = (filterType: "all" | "blocked" | "limited") => {
    setFilter(filter === filterType ? "all" : filterType);
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
          ) : scanning ? (
            "Scanning network..."
          ) : (
            "No network info available"
          )}
        </p>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
        <StatCard
          label="Total Devices"
          value={stats.totalDevices}
          isActive={filter === "all"}
          onClick={() => handleFilterClick("all")}
          activeColor="text-green-500"
          pulseClass={pulseClasses.total}
        />

        <StatCard
          label="Blocked"
          value={stats.blockedDevices}
          isActive={filter === "blocked"}
          onClick={() => handleFilterClick("blocked")}
          activeColor="text-red-500"
          pulseClass={pulseClasses.blocked}
        />

        <StatCard
          label="Limited"
          value={stats.limitedDevices}
          isActive={filter === "limited"}
          onClick={() => handleFilterClick("limited")}
          activeColor="text-yellow-500"
          pulseClass={pulseClasses.limited}
        />

        <StatCard
          label="Bandwidth"
          value={formatBandwidth(stats.totalBandwidth)}
          pulseClass={pulseClasses.bandwidth}
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
