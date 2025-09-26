import React, { useState } from 'react';
import { Grid, List, Filter } from 'lucide-react';
import { DeviceCard } from '../DeviceCard/DeviceCard';
import { useNetworkStore } from '../../stores/networkStore';

export const DeviceList: React.FC = () => {
  const { scanning, getFilteredDevices, searchQuery } = useNetworkStore();
  const [viewMode, setViewMode] = useState<'grid' | 'list'>('grid');
  const [filter, setFilter] = useState<'all' | 'online' | 'blocked' | 'limited'>('all');

  // Get filtered devices based on search and status filter
  const searchFilteredDevices = getFilteredDevices();
  const filteredDevices = searchFilteredDevices.filter(device => {
    if (filter === 'all') return true;
    return device.status === filter;
  });

  return (
    <div className="neu-card rounded-2xl p-6">
      {/* Header */}
      <div className="flex items-center justify-between mb-6">
        <div>
          <h2 className="text-xl font-semibold text-neu-text dark:text-dark-text">
            Connected Devices
          </h2>
          <p className="text-sm text-neu-text-secondary dark:text-dark-text-secondary mt-1">
            {filteredDevices.length} device{filteredDevices.length !== 1 ? 's' : ''} found
          </p>
        </div>

        <div className="flex items-center gap-3">
          {/* Filter Dropdown */}
          <div className="relative">
            <select
              value={filter}
              onChange={(e) => setFilter(e.target.value as any)}
              className="neu-input pr-10 pl-4 py-2 text-sm appearance-none cursor-pointer"
            >
              <option value="all">All Devices</option>
              <option value="online">Online Only</option>
              <option value="blocked">Blocked Only</option>
              <option value="limited">Limited Only</option>
            </select>
            <Filter className="absolute right-3 top-1/2 -translate-y-1/2 w-4 h-4 pointer-events-none text-neu-text-secondary" />
          </div>

          {/* View Mode Toggle */}
          <div className="flex gap-1 neu-card p-1 rounded-lg">
            <button
              onClick={() => setViewMode('grid')}
              className={`p-2 rounded-lg transition-all ${
                viewMode === 'grid'
                  ? 'neu-button'
                  : 'hover:bg-gray-100 dark:hover:bg-gray-800'
              }`}
              aria-label="Grid view"
            >
              <Grid className="w-4 h-4" />
            </button>
            <button
              onClick={() => setViewMode('list')}
              className={`p-2 rounded-lg transition-all ${
                viewMode === 'list'
                  ? 'neu-button'
                  : 'hover:bg-gray-100 dark:hover:bg-gray-800'
              }`}
              aria-label="List view"
            >
              <List className="w-4 h-4" />
            </button>
          </div>
        </div>
      </div>

      {/* Loading State */}
      {scanning && (
        <div className="flex flex-col items-center justify-center py-20">
          <div className="w-16 h-16 rounded-full border-4 border-neu-primary/20 border-t-neu-primary animate-spin"></div>
          <p className="mt-4 text-neu-text-secondary dark:text-dark-text-secondary">
            Scanning network...
          </p>
        </div>
      )}

      {/* No Devices State */}
      {!scanning && filteredDevices.length === 0 && (
        <div className="flex flex-col items-center justify-center py-20">
          <div className="w-24 h-24 rounded-full bg-gray-100 dark:bg-gray-800 flex items-center justify-center mb-4">
            <Grid className="w-12 h-12 text-gray-400" />
          </div>
          <p className="text-lg font-medium text-neu-text dark:text-dark-text">
            No devices found
          </p>
          <p className="text-sm text-neu-text-secondary dark:text-dark-text-secondary mt-1">
            {searchQuery ? 'Try a different search term' : filter !== 'all' ? 'Try changing the filter' : 'Make sure you\'re connected to a network'}
          </p>
        </div>
      )}

      {/* Device Grid/List */}
      {!scanning && filteredDevices.length > 0 && (
        <div
          className={
            viewMode === 'grid'
              ? 'grid grid-cols-1 lg:grid-cols-2 xl:grid-cols-3 gap-6'
              : 'space-y-4'
          }
        >
          {filteredDevices.map(device => (
            <DeviceCard
              key={device.id}
              device={device}
              viewMode={viewMode}
            />
          ))}
        </div>
      )}
    </div>
  );
};