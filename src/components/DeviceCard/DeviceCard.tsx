import React, { useState, useEffect, useRef } from 'react';
import { Edit2, Check } from 'lucide-react';
import { type Device, useNetworkStore } from '../../stores/networkStore';
import { useSortable } from '@dnd-kit/sortable';
import { CSS } from '@dnd-kit/utilities';
import { useNotification } from '../../hooks/useNotification';
import { useClickOutside } from '../../hooks/useClickOutside';

interface DeviceCardProps {
  device: Device;
  viewMode: 'grid' | 'list';
}

export const DeviceCard: React.FC<DeviceCardProps> = ({ device, viewMode }) => {
  const { cutDevice, restoreDevice, limitBandwidth, updateDeviceName, removeBandwidthLimit } = useNetworkStore();
  const { showNotification } = useNotification();
  const [isEditing, setIsEditing] = useState(false);
  const [customName, setCustomName] = useState(device.customName || '');
  const [bandwidthLimit, setBandwidthLimit] = useState('');
  const [showLimitInput, setShowLimitInput] = useState(false);
  const limitRef = useRef<HTMLDivElement>(null);

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

  // Close limit input when clicking outside
  useClickOutside(limitRef, () => {
    if (showLimitInput) {
      setShowLimitInput(false);
    }
  }, showLimitInput);

  // Update customName when device prop changes and load from localStorage
  useEffect(() => {
    const storedName = localStorage.getItem(`device-name-${device.id}`);
    setCustomName(storedName || device.customName || '');
  }, [device.id, device.customName]);

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

  const handleOpenLimitInput = () => {
    // Set current limit value or empty string for new limit
    setBandwidthLimit(device.bandwidthLimit?.toString() || '');
    setShowLimitInput(true);
  };

  const handleSetBandwidthLimit = () => {
    // Don't allow empty input - show error instead
    if (bandwidthLimit.trim() === '') {
      showNotification({
        type: 'error',
        title: 'Invalid Limit',
        message: 'Please enter a valid bandwidth limit or click "Remove limit" to clear',
      });
      return;
    }

    const limit = parseFloat(bandwidthLimit);

    if (!isNaN(limit) && limit > 0 && limit <= 10000) {
      limitBandwidth(device.id, limit);
      setShowLimitInput(false);
    } else {
      // Invalid input - keep input open and show error feedback
      showNotification({
        type: 'error',
        title: 'Invalid Limit',
        message: 'Please enter a valid bandwidth limit between 0.1 and 10000 MB/s',
      });
    }
  };

  // Format bandwidth with dynamic units
  const formatBandwidth = (mbps: number): string => {
    if (mbps >= 1000) {
      return `${(mbps / 1000).toFixed(1)} GB/s`;
    }
    return `${mbps.toFixed(1)} MB/s`;
  };

  const handleAction = async (action: 'cut' | 'restore' | 'limit') => {
    switch (action) {
      case 'cut':
        // Check if it's a special device that needs confirmation
        if (device.isCurrentDevice) {
          const confirmed = await showNotification({
            type: 'confirm',
            title: 'Disconnect Current Device?',
            message: 'You are about to disconnect your own device. This will interrupt your network connection.',
            confirmText: 'Disconnect',
            cancelText: 'Cancel',
          });
          if (!confirmed) return;
        } else if (device.isGateway) {
          const confirmed = await showNotification({
            type: 'confirm',
            title: 'Disconnect Gateway?',
            message: 'Disconnecting the gateway will affect ALL devices on the network. Are you absolutely sure?',
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
        {...attributes}
        {...listeners}
        className="neu-card p-3 rounded-lg flex items-center justify-between hover:transform hover:scale-[1.005] transition-all sortable-item cursor-move">
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
              {formatBandwidth(device.bandwidthCurrent)}
            </p>
            {device.bandwidthLimit && (
              <p className="text-xs text-yellow-500">
                Limited: {formatBandwidth(device.bandwidthLimit)}
              </p>
            )}
          </div>

          {device.status === 'blocked' ? (
            <button
              onClick={(e) => {
                e.stopPropagation();
                handleAction('restore');
              }}
              className="neu-button-primary h-8 px-3 flex items-center justify-center text-xs"
              style={{ minWidth: '110px' }}
            >
              Restore
            </button>
          ) : (
            <>
              <button
                onClick={(e) => {
                  e.stopPropagation();
                  handleAction('cut');
                }}
                className="neu-button-danger h-8 px-3 flex items-center justify-center text-xs"
                disabled={device.isCurrentDevice}
              >
                Cut
              </button>
              <div className="relative" ref={limitRef}>
                <button
                  onClick={(e) => {
                    e.stopPropagation();
                    handleOpenLimitInput();
                  }}
                  className={`neu-button h-8 px-3 flex items-center justify-center text-xs ${
                    showLimitInput ? 'neu-pressed' : ''
                  }`}
                >
                  Limit
                </button>
                {showLimitInput && (
                  <div
                    className="absolute top-full mt-1 right-0 p-2 neu-card rounded-lg animate-flow-down z-50"
                    style={{ minWidth: '180px' }}
                    onClick={(e) => e.stopPropagation()}
                  >
                    <div className="flex items-center gap-1">
                      <input
                        type="number"
                        value={bandwidthLimit}
                        onChange={(e) => setBandwidthLimit(e.target.value)}
                        placeholder={device.bandwidthLimit ? `Current: ${device.bandwidthLimit} MB/s` : "MB/s"}
                        className="neu-input flex-1 text-xs px-2 py-1.5"
                        min="0.1"
                        step="0.1"
                        autoFocus
                        onKeyPress={(e) => {
                          if (e.key === 'Enter') {
                            e.preventDefault();
                            handleSetBandwidthLimit();
                          }
                        }}
                      />
                      <button
                        onClick={handleSetBandwidthLimit}
                        className="neu-button-primary text-xs px-3 py-1.5"
                      >
                        Apply
                      </button>
                    </div>
                    {device.bandwidthLimit && (
                      <button
                        onClick={() => {
                          removeBandwidthLimit(device.id);
                          setShowLimitInput(false);
                        }}
                        className="text-xs text-text-secondary hover:text-red-500 mt-1 transition-colors"
                      >
                        Remove limit
                      </button>
                    )}
                  </div>
                )}
              </div>
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
      {...attributes}
      {...listeners}
      className="neu-card-hover p-4 rounded-lg relative animate-scale-in flex flex-col sortable-item cursor-move">
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
                  onClick={(e) => e.stopPropagation()}
                  className="neu-input px-2 py-1 text-xs"
                  autoFocus
                  onKeyPress={(e) => e.key === 'Enter' && handleSaveName()}
                />
                <button onClick={(e) => {
                  e.stopPropagation();
                  handleSaveName();
                }} className="text-green-500">
                  <Check className="w-3 h-3" />
                </button>
                <button onClick={(e) => {
                  e.stopPropagation();
                  setIsEditing(false);
                }} className="text-red-500 text-xs">
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
            {formatBandwidth(device.bandwidthCurrent)}
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
          <p className="text-xs text-yellow-500 mt-0.5">
            Limited: {formatBandwidth(device.bandwidthLimit)}
          </p>
        )}
      </div>


      {/* Action Buttons - Fixed height */}
      <div className="flex gap-1 mt-auto">
        {device.status === 'blocked' ? (
          <button
            onClick={(e) => {
              e.stopPropagation();
              handleAction('restore');
            }}
            className="neu-button-primary h-8 flex-1 text-xs flex items-center justify-center ripple"
          >
            Restore
          </button>
        ) : (
          <>
            <button
              onClick={(e) => {
                e.stopPropagation();
                handleAction('cut');
              }}
              className="neu-button-danger h-8 flex-1 text-xs flex items-center justify-center ripple"
              disabled={device.isCurrentDevice}
            >
              Cut
            </button>
            <div className="relative flex-1" ref={limitRef}>
              <button
                onClick={(e) => {
                  e.stopPropagation();
                  handleOpenLimitInput();
                }}
                className={`neu-button h-8 w-full text-xs flex items-center justify-center ripple ${
                  showLimitInput ? 'neu-pressed' : ''
                }`}
              >
                Limit
              </button>
              {showLimitInput && (
                <div
                  className="absolute bottom-full mb-1 left-0 right-0 p-2 neu-card rounded-lg animate-flow-up z-50"
                  onClick={(e) => e.stopPropagation()}
                >
                  <div className="flex items-center gap-1">
                    <input
                      type="number"
                      value={bandwidthLimit}
                      onChange={(e) => setBandwidthLimit(e.target.value)}
                      placeholder={device.bandwidthLimit ? `Current: ${device.bandwidthLimit} MB/s` : "MB/s"}
                      className="neu-input flex-1 text-xs px-2 py-1.5"
                      min="0.1"
                      step="0.1"
                      autoFocus
                      onKeyPress={(e) => {
                        if (e.key === 'Enter') {
                          e.preventDefault();
                          handleSetBandwidthLimit();
                        }
                      }}
                    />
                    <button
                      onClick={handleSetBandwidthLimit}
                      className="neu-button-primary text-xs px-3 py-1.5"
                    >
                      Apply
                    </button>
                  </div>
                  {device.bandwidthLimit && (
                    <button
                      onClick={() => {
                        removeBandwidthLimit(device.id);
                        setShowLimitInput(false);
                      }}
                      className="text-xs text-text-secondary hover:text-red-500 mt-1 transition-colors"
                    >
                      Remove limit
                    </button>
                  )}
                </div>
              )}
            </div>
          </>
        )}
      </div>
    </div>
  );
};