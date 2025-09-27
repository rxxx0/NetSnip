import React, { useState, useEffect } from 'react';
import { Zap, Edit2, Ban, Play, Check, GripVertical } from 'lucide-react';
import { type Device, useNetworkStore } from '../../stores/networkStore';
import { useSortable } from '@dnd-kit/sortable';
import { CSS } from '@dnd-kit/utilities';
import { useNotification } from '../../hooks/useNotification';

interface DeviceCardProps {
  device: Device;
  viewMode: 'grid' | 'list';
}

export const DeviceCard: React.FC<DeviceCardProps> = ({ device, viewMode }) => {
  const { cutDevice, restoreDevice, limitBandwidth, updateDeviceName } = useNetworkStore();
  const { showNotification } = useNotification();
  const [isEditing, setIsEditing] = useState(false);
  const [customName, setCustomName] = useState(device.customName || '');
  const [bandwidthLimit, setBandwidthLimit] = useState(device.bandwidthLimit?.toString() || '');
  const [showBandwidthInput, setShowBandwidthInput] = useState(false);

  // Drag and drop setup
  const {
    attributes,
    listeners,
    setNodeRef,
    transform,
    transition,
    isDragging,
  } = useSortable({ id: device.id });

  const style = {
    transform: CSS.Transform.toString(transform),
    transition,
    opacity: isDragging ? 0.5 : 1,
  };

  // Update customName when device prop changes and load from localStorage
  useEffect(() => {
    const storedName = localStorage.getItem(`device-name-${device.id}`);
    setCustomName(storedName || device.customName || '');
    setBandwidthLimit(device.bandwidthLimit?.toString() || '');
  }, [device.id, device.customName, device.bandwidthLimit]);

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

  const handleSaveName = () => {
    updateDeviceName(device.id, customName);
    // Persist to localStorage
    localStorage.setItem(`device-name-${device.id}`, customName);
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
        // Check if it's a special device that needs confirmation
        if (device.isCurrentDevice) {
          const confirmed = await showNotification({
            type: 'confirm',
            title: 'Disconnect Current Device?',
            message: 'Warning: You are about to disconnect your own device. This will interrupt your network connection. Are you sure?',
            confirmText: 'Disconnect',
            cancelText: 'Cancel',
          });
          if (!confirmed) return;
        } else if (device.isGateway) {
          const confirmed = await showNotification({
            type: 'confirm',
            title: 'Disconnect Gateway?',
            message: 'Warning: Disconnecting the gateway will affect ALL devices on the network. Are you absolutely sure?',
            confirmText: 'Disconnect Gateway',
            cancelText: 'Cancel',
          });
          if (!confirmed) return;
        }
        await cutDevice(device.id);
        break;
      case 'restore':
        await restoreDevice(device.id);
        break;
      case 'limit':
        handleSetBandwidthLimit();
        break;
    }
  };

  if (viewMode === 'list') {
    return (
      <div
        ref={setNodeRef}
        style={style}
        className="neu-card p-3 rounded-lg flex items-center justify-between hover:transform hover:scale-[1.005] transition-all sortable-item">
        {/* Drag Handle */}
        <div
          {...attributes}
          {...listeners}
          className="cursor-move p-1 mr-2 hover:bg-gray-100 dark:hover:bg-gray-800 rounded"
        >
          <GripVertical className="w-4 h-4 text-gray-400" />
        </div>

        <div className="flex items-center gap-3 flex-1">
          {/* Device Type Indicator - Square shape */}
          <div
            className="w-3 h-3 rounded"
            style={{
              background: getDeviceTypeColor(),
            }}
          />

          {/* Info */}
          <div>
            <div className="flex items-center gap-2 mb-0.5">
              {isEditing ? (
                <div className="flex items-center gap-1">
                  <input
                    type="text"
                    value={customName}
                    onChange={(e) => setCustomName(e.target.value)}
                    className="neu-input px-2 py-1 text-xs"
                    autoFocus
                    onKeyPress={(e) => e.key === 'Enter' && handleSaveName()}
                    onClick={(e) => e.stopPropagation()}
                  />
                  <button onClick={(e) => {
                    e.stopPropagation();
                    handleSaveName();
                  }} className="text-green-500">
                    <Check className="w-3 h-3" />
                  </button>
                </div>
              ) : (
                <div className="flex items-center gap-2">
                  <h3 className="text-sm font-semibold text-text-primary">
                    {device.customName || device.name}
                  </h3>
                  <button
                    onClick={(e) => {
                      e.stopPropagation();
                      setIsEditing(true);
                    }}
                    className="opacity-0 hover:opacity-50 transition-opacity"
                  >
                    <Edit2 className="w-2.5 h-2.5" />
                  </button>
                  {device.isGateway && (
                    <span className="px-1.5 py-0.5 text-[10px] font-medium rounded-full bg-blue-500/10 text-blue-500">
                      Gateway
                    </span>
                  )}
                  {device.isCurrentDevice && (
                    <span className="px-1.5 py-0.5 text-[10px] font-medium rounded-full bg-purple-500/10 text-purple-500">
                      This Device
                    </span>
                  )}
                </div>
              )}
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
              onClick={(e) => {
                e.stopPropagation();
                handleAction('restore');
              }}
              className="neu-button-primary h-8 px-3 flex items-center justify-center gap-1 text-xs"
            >
              <Play className="w-3 h-3" />
              Restore
            </button>
          ) : (
            <>
              <button
                onClick={(e) => {
                  e.stopPropagation();
                  handleAction('cut');
                }}
                className="neu-button-danger h-8 px-3 flex items-center justify-center gap-1 text-xs"
                disabled={device.isCurrentDevice}
              >
                <Ban className="w-3 h-3" />
                Cut
              </button>
              <button
                onClick={(e) => {
                  e.stopPropagation();
                  setShowBandwidthInput(!showBandwidthInput);
                }}
                className="neu-button h-8 px-3 flex items-center justify-center gap-1 text-xs"
              >
                <Zap className="w-3 h-3" />
                Limit
              </button>
            </>
          )}
        </div>
      </div>
    );
  }

  return (
    <div
      ref={setNodeRef}
      style={style}
      className="neu-card-hover p-4 rounded-lg relative animate-scale-in flex flex-col sortable-item">
      {/* Drag Handle */}
      <div
        {...attributes}
        {...listeners}
        className="absolute top-2 right-2 cursor-move p-1 hover:bg-gray-100 dark:hover:bg-gray-800 rounded z-10"
      >
        <GripVertical className="w-4 h-4 text-gray-400" />
      </div>

      {/* Device Header */}
      <div className="flex items-start justify-between mb-2">
        <div className="flex items-center gap-3">
          {/* Device Type Indicator - Square shape */}
          <div
            className="w-3 h-3 rounded"
            style={{
              background: getDeviceTypeColor(),
            }}
          />
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
              <div>
                <div className="flex items-center gap-1.5">
                  <h3 className="text-sm font-semibold text-text-primary">
                    {device.customName || device.name}
                  </h3>
                  <button onClick={(e) => {
                    e.stopPropagation();
                    setIsEditing(true);
                  }} className="opacity-0 group-hover:opacity-50 hover:!opacity-100 transition-opacity">
                    <Edit2 className="w-2.5 h-2.5" />
                  </button>
                  {device.isGateway && (
                    <span className="inline-flex px-1.5 py-0.5 text-[10px] font-medium rounded-full bg-blue-500/10 text-blue-500">
                      Gateway
                    </span>
                  )}
                  {device.isCurrentDevice && (
                    <span className="inline-flex px-1.5 py-0.5 text-[10px] font-medium rounded-full bg-purple-500/10 text-purple-500">
                      This Device
                    </span>
                  )}
                </div>
                <p className="text-xs text-text-secondary capitalize">
                  {device.manufacturer || device.deviceType}
                </p>
              </div>
            )}
          </div>
        </div>

        {/* Empty div for spacing */}
      </div>

      {/* Device Info */}
      <div className="space-y-1 mb-2">
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
      <div className="mb-2">
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

      {/* Action Buttons - Fixed height */}
      <div className="flex gap-1 mt-auto">
        {device.status === 'blocked' ? (
          <button
            onClick={(e) => {
              e.stopPropagation();
              handleAction('restore');
            }}
            className="neu-button-primary h-8 flex-1 text-xs flex items-center justify-center gap-1 ripple"
          >
            <Play className="w-3 h-3" />
            Restore
          </button>
        ) : (
          <>
            <button
              onClick={(e) => {
                e.stopPropagation();
                handleAction('cut');
              }}
              className="neu-button-danger h-8 flex-1 text-xs flex items-center justify-center gap-1 ripple"
              disabled={device.isCurrentDevice}
            >
              <Ban className="w-3 h-3" />
              Cut
            </button>
            <button
              onClick={(e) => {
                e.stopPropagation();
                setShowBandwidthInput(!showBandwidthInput);
              }}
              className="neu-button h-8 flex-1 text-xs flex items-center justify-center gap-1 ripple"
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