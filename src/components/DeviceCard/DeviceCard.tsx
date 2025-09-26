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
      <div className="neu-card p-4 rounded-xl flex items-center justify-between hover:transform hover:scale-[1.01] transition-all">
        <div className="flex items-center gap-4">
          {/* Device Type Indicator */}
          <div
            className="w-12 h-12 rounded-xl flex items-center justify-center"
            style={{
              background: `linear-gradient(145deg, ${getDeviceTypeColor()}22, ${getDeviceTypeColor()}33)`,
            }}
          >
            <div
              className="w-6 h-6 rounded-md"
              style={{
                background: `linear-gradient(145deg, ${getDeviceTypeColor()}, ${getDeviceTypeColor()}dd)`,
              }}
            />
          </div>

          {/* Info */}
          <div>
            <div className="flex items-center gap-2">
              <h3 className="font-semibold text-text-primary">
                {device.customName || device.name}
              </h3>
              {device.isGateway && (
                <span className="neu-badge text-xs">Gateway</span>
              )}
              {device.isCurrentDevice && (
                <span className="neu-badge text-xs">This Device</span>
              )}
              <div className={`w-2 h-2 rounded-full ${getStatusIndicator()}`}></div>
            </div>
            <p className="text-sm text-text-secondary font-mono">
              {device.ip} • {device.mac}
            </p>
          </div>
        </div>

        {/* Actions */}
        <div className="flex items-center gap-3">
          <div className="text-right mr-4">
            <p className="text-lg font-bold text-text-primary">
              {device.bandwidthCurrent.toFixed(1)} MB/s
            </p>
            {device.bandwidthLimit && (
              <p className="text-xs text-text-secondary">
                Limited: {device.bandwidthLimit} MB/s
              </p>
            )}
          </div>

          {device.status === 'blocked' ? (
            <button
              onClick={() => handleAction('restore')}
              className="neu-button-primary flex items-center gap-2"
            >
              <Play className="w-4 h-4" />
              Restore
            </button>
          ) : (
            <button
              onClick={() => handleAction('cut')}
              className="neu-button-danger flex items-center gap-2"
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
    <div className="neu-card-hover p-5 rounded-2xl relative animate-scale-in">
      {/* Status Indicator */}
      <div className={`absolute top-4 right-4 w-3 h-3 rounded-full ${getStatusIndicator()}`}></div>

      {/* Device Header */}
      <div className="flex items-start justify-between mb-4">
        <div className="flex items-center gap-3">
          <div
            className="w-14 h-14 rounded-xl flex items-center justify-center"
            style={{
              background: `linear-gradient(145deg, ${getDeviceTypeColor()}22, ${getDeviceTypeColor()}33)`,
            }}
          >
            <div
              className="w-8 h-8 rounded-lg"
              style={{
                background: `linear-gradient(145deg, ${getDeviceTypeColor()}, ${getDeviceTypeColor()}dd)`,
              }}
            />
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
                  onKeyPress={(e) => e.key === 'Enter' && handleSaveName()}
                />
                <button onClick={handleSaveName} className="text-green-500">
                  <Check className="w-4 h-4" />
                </button>
                <button onClick={() => setIsEditing(false)} className="text-red-500">
                  ✕
                </button>
              </div>
            ) : (
              <div className="flex items-center gap-2">
                <h3 className="font-semibold text-text-primary">
                  {device.customName || device.name}
                </h3>
                <button onClick={() => setIsEditing(true)} className="opacity-0 group-hover:opacity-50 hover:!opacity-100 transition-opacity">
                  <Edit2 className="w-3 h-3" />
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
            className="neu-button p-2 rounded-lg opacity-50 hover:opacity-100 transition-opacity"
          >
            <MoreVertical className="w-4 h-4" />
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
      <div className="space-y-2 mb-4">
        <div className="flex justify-between items-center">
          <span className="text-sm text-text-secondary">IP</span>
          <span className="text-sm font-mono text-text-primary">{device.ip}</span>
        </div>
        <div className="flex justify-between items-center">
          <span className="text-sm text-text-secondary">MAC</span>
          <span className="text-sm font-mono text-text-primary">
            {device.mac}
          </span>
        </div>
      </div>

      {/* Bandwidth Bar */}
      <div className="mb-4">
        <div className="flex justify-between items-center mb-2">
          <span className="text-sm text-text-secondary">Bandwidth</span>
          <span className="text-sm font-bold text-text-primary">
            {device.bandwidthCurrent.toFixed(1)} MB/s
          </span>
        </div>
        <div className="h-1.5 bg-gray-200 dark:bg-gray-700 rounded-full overflow-hidden">
          <div
            className="h-full transition-all duration-300"
            style={{
              width: `${Math.min((device.bandwidthCurrent / (device.bandwidthLimit || 100)) * 100, 100)}%`,
              background: `linear-gradient(90deg, ${getDeviceTypeColor()}, ${getDeviceTypeColor()}dd)`,
            }}
          />
        </div>
        {device.bandwidthLimit && (
          <p className="text-xs text-text-secondary mt-1">
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
      <div className="flex gap-2">
        {device.status === 'blocked' ? (
          <button
            onClick={() => handleAction('restore')}
            className="neu-button-primary flex-1 py-2 text-sm flex items-center justify-center gap-2 ripple"
          >
            <Play className="w-4 h-4" />
            Restore
          </button>
        ) : (
          <>
            <button
              onClick={() => handleAction('cut')}
              className="neu-button-danger flex-1 py-2 text-sm flex items-center justify-center gap-2 ripple"
              disabled={device.isCurrentDevice}
            >
              <Ban className="w-4 h-4" />
              Cut
            </button>
            <button
              onClick={() => setShowBandwidthInput(!showBandwidthInput)}
              className="neu-button flex-1 py-2 text-sm flex items-center justify-center gap-2 ripple"
            >
              <Zap className="w-4 h-4" />
              {showBandwidthInput ? 'Cancel' : 'Limit'}
            </button>
          </>
        )}
      </div>
    </div>
  );
};