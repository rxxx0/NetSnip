import React, { useMemo, useCallback, useState, useEffect, useRef } from "react";
import { useNetworkStore } from "../../stores/networkStore";

// Separate StatCard component that manages its own pulse animation
const StatCard: React.FC<{
  label: string;
  value: string | number;
  isActive?: boolean;
  onClick?: () => void;
  activeColor?: string;
  pulseColor?: string;
}> = ({ label, value, isActive, onClick, activeColor, pulseColor }) => {
  const [isPulsing, setIsPulsing] = useState(false);
  const prevValue = useRef<string | number | null>(null);
  const [hoverClass, setHoverClass] = useState("");

  // Check if value changed and trigger pulse
  useEffect(() => {
    const currentValue = value.toString();
    if (prevValue.current !== null && prevValue.current !== currentValue) {
      setIsPulsing(true);
      const timer = setTimeout(() => setIsPulsing(false), 600);
      return () => clearTimeout(timer);
    }
    prevValue.current = currentValue;
  }, [value]);

  return (
    <button
      onClick={onClick}
      onMouseEnter={() => !isActive && setHoverClass("hover-lift-active")}
      onMouseLeave={() => setHoverClass("")}
      className={`neu-card p-4 text-center transition-all cursor-pointer w-full ${
        isActive ? "neu-pressed" : ""
      } ${hoverClass}`}
      style={{ position: 'relative' }}
    >
      <div
        className={`text-2xl font-semibold transition-colors ${
          isActive ? activeColor : "text-text-primary"
        } ${isPulsing && pulseColor ? pulseColor : ''}`}
      >
        {value}
      </div>
      <p className="text-xs text-text-secondary mt-1">{label}</p>
    </button>
  );
};

export const Dashboard: React.FC = () => {
  const { devices, networkInfo, scanning, filter, setFilter } =
    useNetworkStore();

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
          pulseColor="pulseGreen"
        />

        <StatCard
          label="Blocked"
          value={stats.blockedDevices}
          isActive={filter === "blocked"}
          onClick={() => handleFilterClick("blocked")}
          activeColor="text-red-500"
          pulseColor="pulseRed"
        />

        <StatCard
          label="Limited"
          value={stats.limitedDevices}
          isActive={filter === "limited"}
          onClick={() => handleFilterClick("limited")}
          activeColor="text-yellow-500"
          pulseColor="pulseYellow"
        />

        <StatCard
          label="Bandwidth"
          value={formatBandwidth(stats.totalBandwidth)}
          pulseColor="pulseBlue"
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
