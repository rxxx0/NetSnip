import React, { useState, useEffect } from 'react';
import { RefreshCw, Moon, Sun, Settings, Search, X } from 'lucide-react';
import { useNetworkStore } from '../../stores/networkStore';
import { useDebounce } from '../../hooks/useDebounce';

interface HeaderProps {
  theme: 'light' | 'dark';
  onToggleTheme: () => void;
}

export const Header: React.FC<HeaderProps> = ({ theme, onToggleTheme }) => {
  const { scanning, scanNetwork, devices, searchQuery, setSearchQuery } = useNetworkStore();
  const [localSearchQuery, setLocalSearchQuery] = useState(searchQuery);
  const debouncedSearchQuery = useDebounce(localSearchQuery, 300);

  const onlineDevices = devices.filter(d => d.status === 'online').length;
  const blockedDevices = devices.filter(d => d.status === 'blocked').length;

  useEffect(() => {
    setSearchQuery(debouncedSearchQuery);
  }, [debouncedSearchQuery, setSearchQuery]);

  return (
    <header className="mb-8">
      <div className="neu-card p-6">
        <div className="flex items-center justify-between">
          {/* Logo - Simplified */}
          <div className="flex items-center gap-4">
            <h1 className="text-3xl font-bold text-gradient">
              NetSnip
            </h1>

            {/* Status pills */}
            <div className="flex items-center gap-3">
              <div className="flex items-center gap-2">
                <div className="w-2 h-2 rounded-full bg-green-500 animate-pulse"></div>
                <span className="text-sm font-medium text-text-secondary">
                  {onlineDevices} online
                </span>
              </div>
              {blockedDevices > 0 && (
                <div className="flex items-center gap-2">
                  <div className="w-2 h-2 rounded-full bg-red-500"></div>
                  <span className="text-sm font-medium text-text-secondary">
                    {blockedDevices} blocked
                  </span>
                </div>
              )}
            </div>
          </div>

          {/* Search Bar - More prominent */}
          <div className="flex-1 max-w-xl mx-8">
            <div className="relative">
              <Search className="absolute left-4 top-1/2 -translate-y-1/2 w-5 h-5 text-text-muted" />
              <input
                type="text"
                value={localSearchQuery}
                onChange={(e) => setLocalSearchQuery(e.target.value)}
                placeholder="Search by name, IP address, or MAC address..."
                className="neu-input w-full pl-12 pr-10 py-3 text-sm"
              />
              {localSearchQuery && (
                <button
                  onClick={() => setLocalSearchQuery('')}
                  className="absolute right-4 top-1/2 -translate-y-1/2 text-text-muted hover:text-text-secondary transition-colors"
                  aria-label="Clear search"
                >
                  <X className="w-4 h-4" />
                </button>
              )}
            </div>
          </div>

          {/* Actions - Minimal */}
          <div className="flex items-center gap-2">
            {/* Refresh Button */}
            <button
              onClick={() => scanNetwork()}
              disabled={scanning}
              className={`neu-button p-3 rounded-xl ${scanning ? 'neu-pressed' : ''}`}
              aria-label="Refresh network scan"
            >
              <RefreshCw className={`w-5 h-5 ${scanning ? 'animate-spin' : ''}`} />
            </button>

            {/* Theme Toggle */}
            <button
              onClick={onToggleTheme}
              className="neu-button p-3 rounded-xl"
              aria-label={`Switch to ${theme === 'light' ? 'dark' : 'light'} mode`}
            >
              {theme === 'light' ? (
                <Moon className="w-5 h-5" />
              ) : (
                <Sun className="w-5 h-5" />
              )}
            </button>

            {/* Settings */}
            <button
              className="neu-button p-3 rounded-xl"
              aria-label="Settings"
            >
              <Settings className="w-5 h-5" />
            </button>
          </div>
        </div>
      </div>
    </header>
  );
};