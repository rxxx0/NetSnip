import React, { useState, useRef, useEffect } from 'react';
import { MoreVertical, Zap, Edit2, Ban, Play, Check } from 'lucide-react';
import { type Device, useNetworkStore } from '../../stores/networkStore';
import { useClickOutside } from '../../hooks/useClickOutside';

interface DeviceCardProps {
  device: Device;
  viewMode: 'grid' | 'list';
}

export const DeviceCard: React.FC<DeviceCardProps> = ({ device, viewMode }) => {
  const { cutDevice, restoreDevice, limitBandwidth, updateDeviceName } = useNetworkStore();
  const [showMenu, setShowMenu] = useState(false);
  const [isEditing, setIsEditing] = useState(false);
  const [customName, setCustomName] = useState(device.customName || '');
  const [bandwidthLimit, setBandwidthLimit] = useState(device.bandwidthLimit?.toString() || '');
  const [showBandwidthInput, setShowBandwidthInput] = useState(false);
  const menuRef = useRef<HTMLDivElement>(null);

  // Update customName when device prop changes
  useEffect(() => {
    setCustomName(device.customName || '');
    setBandwidthLimit(device.bandwidthLimit?.toString() || '');
  }, [device.customName, device.bandwidthLimit]);

  // Close menu when clicking outside
  useClickOutside(menuRef, () => setShowMenu(false), showMenu);

  const getDeviceTypeColor = () => {
    if (device.isGateway) return 'var(--accent-primary)';
    if (device.isCurrentDevice) return 'var(--accent-primary-light)';

    switch (device.deviceType) {
      case 'router': return 'var(--accent-primary)';
      case 'phone': return 'var(--accent-success)';
      case 'computer': return 'var(--accent-primary-light)';
      case 'tablet': return '#9333EA';
      case 'tv': return '#EC4899';
      case 'gaming': return '#14B8A6';
      case 'iot': return '#06B6D4';
      default: return 'var(--text-muted)';
    }
  };

  const getStatusIndicator = () => {
    switch (device.status) {
      case 'online': return 'bg-green-500';
      case 'blocked': return 'bg-red-500';
      case 'limited': return 'bg-orange-500';
      default: return 'bg-gray-400';
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

  if (viewMode === 'list') {
    return (
      <div className="neu-card p-3 rounded-lg flex items-center justify-between hover:transform hover:scale-[1.005] transition-all">
        <div className="flex items-center gap-3">
          {/* Device Type Indicator */}
          <div
            className="w-8 h-8 rounded-lg flex items-center justify-center"
            style={{
              background: `${getDeviceTypeColor()}15`,
              border: `1px solid ${getDeviceTypeColor()}30`,
            }}
          >
            <div
              className="w-4 h-4 rounded"
              style={{
                background: getDeviceTypeColor(),
              }}
            />
          </div>

          {/* Info */}
          <div>
            <div className="flex items-center gap-2">
              <h3 className="text-sm font-semibold text-text-primary">
                {device.customName || device.name}
              </h3>
              {device.isGateway && (
                <span className="neu-badge text-xs">Gateway</span>
              )}
              {device.isCurrentDevice && (
                <span className="neu-badge text-xs">This Device</span>
              )}
              <div className={`w-1.5 h-1.5 rounded-full ${getStatusIndicator()}`}></div>
            </div>
            <p className="text-xs text-text-secondary font-mono">
              {device.ip} • {device.mac}
            </p>
          </div>
        </div>

        {/* Actions */}
        <div className="flex items-center gap-2">
          <div className="text-right mr-2">
            <p className="text-sm font-bold text-text-primary">
              {device.bandwidthCurrent.toFixed(1)} MB/s
            </p>
            {device.bandwidthLimit && (
              <p className="text-xs text-text-secondary">
                Limited: {device.bandwidthLimit}
              </p>
            )}
          </div>

          {device.status === 'blocked' ? (
            <button
              onClick={() => handleAction('restore')}
              className="neu-button-primary flex items-center gap-1 text-xs"
            >
              <Play className="w-3 h-3" />
              Restore
            </button>
          ) : (
            <button
              onClick={() => handleAction('cut')}
              className="neu-button-danger flex items-center gap-1 text-xs"
              disabled={device.isCurrentDevice}
            >
              <Ban className="w-3 h-3" />
              Cut
            </button>
          )}
        </div>
      </div>
    );
  }

  return (
    <div className="neu-card-hover p-4 rounded-lg relative animate-scale-in">
      {/* Status Indicator */}
      <div className={`absolute top-3 right-3 w-2 h-2 rounded-full ${getStatusIndicator()}`}></div>

      {/* Device Header */}
      <div className="flex items-start justify-between mb-3">
        <div className="flex items-center gap-2">
          <div
            className="w-10 h-10 rounded-lg flex items-center justify-center"
            style={{
              background: `${getDeviceTypeColor()}15`,
              border: `1px solid ${getDeviceTypeColor()}30`,
            }}
          >
            <div
              className="w-5 h-5 rounded"
              style={{
                background: getDeviceTypeColor(),
              }}
            />
          </div>
          <div>
            {isEditing ? (
              <div className="flex items-center gap-1">
                <input
                  type="text"
                  value={customName}
                  onChange={(e) => setCustomName(e.target.value)}
                  className="neu-input px-2 py-1 text-xs"
                  autoFocus
                  onKeyPress={(e) => e.key === 'Enter' && handleSaveName()}
                />
                <button onClick={handleSaveName} className="text-green-500">
                  <Check className="w-3 h-3" />
                </button>
                <button onClick={() => setIsEditing(false)} className="text-red-500 text-xs">
                  ✕
                </button>
              </div>
            ) : (
              <div className="flex items-center gap-1">
                <h3 className="text-sm font-semibold text-text-primary">
                  {device.customName || device.name}
                </h3>
                <button onClick={() => setIsEditing(true)} className="opacity-0 group-hover:opacity-50 hover:!opacity-100 transition-opacity">
                  <Edit2 className="w-2.5 h-2.5" />
                </button>
              </div>
            )}
            <p className="text-xs text-text-secondary capitalize">
              {device.manufacturer || device.deviceType}
            </p>
          </div>
        </div>

        {/* Menu Button */}
        <div className="relative" ref={menuRef}>
          <button
            onClick={() => setShowMenu(!showMenu)}
            className="neu-button p-1.5 rounded opacity-50 hover:opacity-100 transition-opacity"
          >
            <MoreVertical className="w-3 h-3" />
          </button>

          {showMenu && (
            <div className="absolute right-0 mt-2 w-48 neu-card rounded-lg p-2 z-10 animate-fade-in">
              <button
                onClick={() => handleAction(device.status === 'blocked' ? 'restore' : 'cut')}
                className="w-full text-left px-3 py-2 rounded hover:bg-gray-100 dark:hover:bg-gray-800 text-sm transition-colors"
                disabled={device.isCurrentDevice}
              >
                {device.status === 'blocked' ? 'Restore Connection' : 'Cut Connection'}
              </button>
              <button
                onClick={() => {
                  setShowBandwidthInput(true);
                  setShowMenu(false);
                }}
                className="w-full text-left px-3 py-2 rounded hover:bg-gray-100 dark:hover:bg-gray-800 text-sm transition-colors"
              >
                Set Bandwidth Limit
              </button>
            </div>
          )}
        </div>
      </div>

      {/* Device Info */}
      <div className="space-y-1 mb-3">
        <div className="flex justify-between items-center">
          <span className="text-xs text-text-secondary">IP</span>
          <span className="text-xs font-mono text-text-primary">{device.ip}</span>
        </div>
        <div className="flex justify-between items-center">
          <span className="text-xs text-text-secondary">MAC</span>
          <span className="text-xs font-mono text-text-primary">
            {device.mac}
          </span>
        </div>
      </div>

      {/* Bandwidth Bar */}
      <div className="mb-3">
        <div className="flex justify-between items-center mb-1">
          <span className="text-xs text-text-secondary">Bandwidth</span>
          <span className="text-xs font-bold text-text-primary">
            {device.bandwidthCurrent.toFixed(1)} MB/s
          </span>
        </div>
        <div className="h-1 bg-gray-200 dark:bg-gray-700 rounded-full overflow-hidden">
          <div
            className="h-full transition-all duration-300"
            style={{
              width: `${Math.min((device.bandwidthCurrent / (device.bandwidthLimit || 100)) * 100, 100)}%`,
              background: getDeviceTypeColor(),
            }}
          />
        </div>
        {device.bandwidthLimit && (
          <p className="text-xs text-text-secondary mt-0.5">
            Limited: {device.bandwidthLimit} MB/s
          </p>
        )}
      </div>

      {/* Badges */}
      <div className="flex gap-1 mb-3">
        {device.isGateway && (
          <span className="neu-badge text-xs">Gateway</span>
        )}
        {device.isCurrentDevice && (
          <span className="neu-badge text-xs">This Device</span>
        )}
        {device.status === 'blocked' && (
          <span className="neu-badge-danger text-xs">Blocked</span>
        )}
        {device.status === 'limited' && (
          <span className="neu-badge-warning text-xs">Limited</span>
        )}
      </div>

      {/* Bandwidth Limit Input */}
      {showBandwidthInput && device.status !== 'blocked' && (
        <div className="mb-4 p-3 rounded-xl glass animate-fade-in">
          <div className="flex items-center gap-2">
            <input
              type="number"
              value={bandwidthLimit}
              onChange={(e) => setBandwidthLimit(e.target.value)}
              placeholder="MB/s"
              className="neu-input flex-1 px-3 py-2 text-sm"
              min="0.1"
              step="0.1"
            />
            <button
              onClick={() => {
                handleSetBandwidthLimit();
                setShowBandwidthInput(false);
              }}
              className="neu-button-primary text-sm"
            >
              Apply
            </button>
            <button
              onClick={() => {
                setBandwidthLimit(device.bandwidthLimit?.toString() || '');
                setShowBandwidthInput(false);
              }}
              className="neu-button text-sm"
            >
              Cancel
            </button>
          </div>
        </div>
      )}

      {/* Action Buttons */}
      <div className="flex gap-1">
        {device.status === 'blocked' ? (
          <button
            onClick={() => handleAction('restore')}
            className="neu-button-primary flex-1 py-1.5 text-xs flex items-center justify-center gap-1 ripple"
          >
            <Play className="w-3 h-3" />
            Restore
          </button>
        ) : (
          <>
            <button
              onClick={() => handleAction('cut')}
              className="neu-button-danger flex-1 py-1.5 text-xs flex items-center justify-center gap-1 ripple"
              disabled={device.isCurrentDevice}
            >
              <Ban className="w-3 h-3" />
              Cut
            </button>
            <button
              onClick={() => setShowBandwidthInput(!showBandwidthInput)}
              className="neu-button flex-1 py-1.5 text-xs flex items-center justify-center gap-1 ripple"
            >
              <Zap className="w-3 h-3" />
              {showBandwidthInput ? 'Cancel' : 'Limit'}
            </button>
          </>
        )}
      </div>
    </div>
  );
};