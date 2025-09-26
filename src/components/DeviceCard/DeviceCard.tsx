import React, { useState } from 'react';
import {
  Router, Smartphone, Monitor, Tablet, Wifi, HelpCircle,
  MoreVertical, Zap, Edit2, Copy, Ban, Play
} from 'lucide-react';
import { type Device, useNetworkStore } from '../../stores/networkStore';

interface DeviceCardProps {
  device: Device;
  viewMode: 'grid' | 'list';
}

export const DeviceCard: React.FC<DeviceCardProps> = ({ device, viewMode }) => {
  const { cutDevice, restoreDevice, limitBandwidth, updateDeviceName } = useNetworkStore();
  const [showMenu, setShowMenu] = useState(false);
  const [isEditing, setIsEditing] = useState(false);
  const [customName, setCustomName] = useState(device.customName || '');
  const [bandwidthLimit] = useState(device.bandwidthLimit?.toString() || '');

  const getDeviceIcon = () => {
    const iconProps = { className: 'w-6 h-6 text-white' };
    switch (device.deviceType) {
      case 'router': return <Router {...iconProps} />;
      case 'phone': return <Smartphone {...iconProps} />;
      case 'computer': return <Monitor {...iconProps} />;
      case 'tablet': return <Tablet {...iconProps} />;
      case 'iot': return <Wifi {...iconProps} />;
      default: return <HelpCircle {...iconProps} />;
    }
  };

  const getStatusColor = () => {
    switch (device.status) {
      case 'online': return 'bg-green-500';
      case 'blocked': return 'bg-red-500';
      case 'limited': return 'bg-orange-500';
      default: return 'bg-gray-400';
    }
  };

  const getIconBgColor = () => {
    if (device.isGateway) return 'bg-gradient-to-br from-purple-500 to-purple-600';
    if (device.isCurrentDevice) return 'bg-gradient-to-br from-blue-500 to-blue-600';
    switch (device.deviceType) {
      case 'router': return 'bg-gradient-to-br from-purple-500 to-purple-600';
      case 'phone': return 'bg-gradient-to-br from-green-500 to-green-600';
      case 'computer': return 'bg-gradient-to-br from-blue-500 to-blue-600';
      case 'tablet': return 'bg-gradient-to-br from-indigo-500 to-indigo-600';
      case 'iot': return 'bg-gradient-to-br from-teal-500 to-teal-600';
      default: return 'bg-gradient-to-br from-gray-500 to-gray-600';
    }
  };

  const handleSaveName = () => {
    updateDeviceName(device.id, customName);
    setIsEditing(false);
  };

  const handleSetBandwidthLimit = () => {
    const limit = parseFloat(bandwidthLimit);
    if (!isNaN(limit) && limit > 0) {
      limitBandwidth(device.id, limit);
    }
  };

  const handleAction = async (action: 'cut' | 'restore' | 'limit') => {
    switch (action) {
      case 'cut':
        await cutDevice(device.id);
        break;
      case 'restore':
        await restoreDevice(device.id);
        break;
      case 'limit':
        handleSetBandwidthLimit();
        break;
    }
    setShowMenu(false);
  };

  const copyToClipboard = (text: string) => {
    navigator.clipboard.writeText(text);
  };

  if (viewMode === 'list') {
    return (
      <div className="neu-card p-4 rounded-xl flex items-center justify-between">
        <div className="flex items-center gap-4">
          {/* Icon */}
          <div className={`w-12 h-12 rounded-xl ${getIconBgColor()} flex items-center justify-center`}>
            {getDeviceIcon()}
          </div>

          {/* Info */}
          <div>
            <div className="flex items-center gap-2">
              <h3 className="font-semibold text-neu-text dark:text-dark-text">
                {device.customName || device.name}
              </h3>
              {device.isGateway && (
                <span className="neu-badge text-xs">Gateway</span>
              )}
              {device.isCurrentDevice && (
                <span className="neu-badge text-xs">This Device</span>
              )}
              <div className={`w-2 h-2 rounded-full ${getStatusColor()}`}></div>
            </div>
            <p className="text-sm text-neu-text-secondary dark:text-dark-text-secondary">
              {device.ip} • {device.mac}
            </p>
          </div>
        </div>

        {/* Actions */}
        <div className="flex items-center gap-3">
          <div className="text-right mr-4">
            <p className="text-lg font-bold text-neu-text dark:text-dark-text">
              {device.bandwidthCurrent.toFixed(1)} MB/s
            </p>
            {device.bandwidthLimit && (
              <p className="text-xs text-neu-text-secondary dark:text-dark-text-secondary">
                Limited to {device.bandwidthLimit} MB/s
              </p>
            )}
          </div>

          {device.status === 'blocked' ? (
            <button
              onClick={() => handleAction('restore')}
              className="neu-button px-4 py-2 text-sm flex items-center gap-2"
            >
              <Play className="w-4 h-4" />
              Restore
            </button>
          ) : (
            <button
              onClick={() => handleAction('cut')}
              className="neu-button px-4 py-2 text-sm flex items-center gap-2"
              disabled={device.isCurrentDevice}
            >
              <Ban className="w-4 h-4" />
              Cut
            </button>
          )}
        </div>
      </div>
    );
  }

  return (
    <div className="neu-card p-5 rounded-2xl relative">
      {/* Status Indicator */}
      <div className={`absolute top-3 right-3 w-3 h-3 rounded-full ${getStatusColor()}`}></div>

      {/* Device Icon and Name */}
      <div className="flex items-start justify-between mb-4">
        <div className="flex items-center gap-3">
          <div className={`w-14 h-14 rounded-xl ${getIconBgColor()} flex items-center justify-center`}>
            {getDeviceIcon()}
          </div>
          <div>
            {isEditing ? (
              <div className="flex items-center gap-2">
                <input
                  type="text"
                  value={customName}
                  onChange={(e) => setCustomName(e.target.value)}
                  className="neu-input px-2 py-1 text-sm"
                  autoFocus
                />
                <button onClick={handleSaveName} className="text-green-500">
                  ✓
                </button>
                <button onClick={() => setIsEditing(false)} className="text-red-500">
                  ✕
                </button>
              </div>
            ) : (
              <div className="flex items-center gap-2">
                <h3 className="font-semibold text-neu-text dark:text-dark-text">
                  {device.customName || device.name}
                </h3>
                <button onClick={() => setIsEditing(true)} className="opacity-50 hover:opacity-100">
                  <Edit2 className="w-3 h-3" />
                </button>
              </div>
            )}
            <p className="text-xs text-neu-text-secondary dark:text-dark-text-secondary">
              {device.manufacturer || device.deviceType}
            </p>
          </div>
        </div>

        {/* Menu Button */}
        <div className="relative">
          <button
            onClick={() => setShowMenu(!showMenu)}
            className="p-2 rounded-lg hover:bg-gray-100 dark:hover:bg-gray-800"
          >
            <MoreVertical className="w-4 h-4" />
          </button>

          {showMenu && (
            <div className="absolute right-0 mt-2 w-48 neu-card rounded-lg p-2 z-10">
              <button
                onClick={() => copyToClipboard(device.ip)}
                className="w-full text-left px-3 py-2 rounded hover:bg-gray-100 dark:hover:bg-gray-800 flex items-center gap-2 text-sm"
              >
                <Copy className="w-4 h-4" />
                Copy IP
              </button>
              <button
                onClick={() => copyToClipboard(device.mac)}
                className="w-full text-left px-3 py-2 rounded hover:bg-gray-100 dark:hover:bg-gray-800 flex items-center gap-2 text-sm"
              >
                <Copy className="w-4 h-4" />
                Copy MAC
              </button>
              <hr className="my-2 border-gray-200 dark:border-gray-700" />
              <button
                onClick={() => handleAction(device.status === 'blocked' ? 'restore' : 'cut')}
                className="w-full text-left px-3 py-2 rounded hover:bg-gray-100 dark:hover:bg-gray-800 flex items-center gap-2 text-sm"
                disabled={device.isCurrentDevice}
              >
                {device.status === 'blocked' ? (
                  <>
                    <Play className="w-4 h-4" />
                    Restore Connection
                  </>
                ) : (
                  <>
                    <Ban className="w-4 h-4" />
                    Cut Connection
                  </>
                )}
              </button>
            </div>
          )}
        </div>
      </div>

      {/* Device Info */}
      <div className="space-y-2 mb-4">
        <div className="flex justify-between items-center">
          <span className="text-sm text-neu-text-secondary dark:text-dark-text-secondary">IP</span>
          <span className="text-sm font-medium text-neu-text dark:text-dark-text">{device.ip}</span>
        </div>
        <div className="flex justify-between items-center">
          <span className="text-sm text-neu-text-secondary dark:text-dark-text-secondary">MAC</span>
          <span className="text-sm font-medium text-neu-text dark:text-dark-text font-mono">
            {device.mac}
          </span>
        </div>
      </div>

      {/* Bandwidth Bar */}
      <div className="mb-4">
        <div className="flex justify-between items-center mb-1">
          <span className="text-sm text-neu-text-secondary dark:text-dark-text-secondary">Bandwidth</span>
          <span className="text-sm font-bold text-neu-text dark:text-dark-text">
            {device.bandwidthCurrent.toFixed(1)} MB/s
          </span>
        </div>
        <div className="h-2 bg-gray-200 dark:bg-gray-700 rounded-full overflow-hidden">
          <div
            className="h-full bg-gradient-to-r from-neu-primary to-neu-secondary transition-all duration-300"
            style={{
              width: `${Math.min((device.bandwidthCurrent / (device.bandwidthLimit || 100)) * 100, 100)}%`
            }}
          />
        </div>
        {device.bandwidthLimit && (
          <p className="text-xs text-neu-text-secondary dark:text-dark-text-secondary mt-1">
            Limited to {device.bandwidthLimit} MB/s
          </p>
        )}
      </div>

      {/* Badges */}
      <div className="flex gap-2 mb-4">
        {device.isGateway && (
          <span className="neu-badge text-xs">Gateway</span>
        )}
        {device.isCurrentDevice && (
          <span className="neu-badge text-xs">This Device</span>
        )}
        {device.status === 'blocked' && (
          <span className="neu-badge text-xs bg-red-100 dark:bg-red-900/20 text-red-600 dark:text-red-400">
            Blocked
          </span>
        )}
        {device.status === 'limited' && (
          <span className="neu-badge text-xs bg-orange-100 dark:bg-orange-900/20 text-orange-600 dark:text-orange-400">
            Limited
          </span>
        )}
      </div>

      {/* Action Buttons */}
      <div className="flex gap-2">
        {device.status === 'blocked' ? (
          <button
            onClick={() => handleAction('restore')}
            className="neu-button flex-1 py-2 text-sm flex items-center justify-center gap-2"
          >
            <Play className="w-4 h-4" />
            Restore
          </button>
        ) : (
          <>
            <button
              onClick={() => handleAction('cut')}
              className="neu-button flex-1 py-2 text-sm flex items-center justify-center gap-2"
              disabled={device.isCurrentDevice}
            >
              <Ban className="w-4 h-4" />
              Cut
            </button>
            <button
              onClick={() => setShowMenu(true)}
              className="neu-button flex-1 py-2 text-sm flex items-center justify-center gap-2"
            >
              <Zap className="w-4 h-4" />
              Limit
            </button>
          </>
        )}
      </div>
    </div>
  );
};