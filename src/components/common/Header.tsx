import React, { useState, useEffect } from 'react';
import { RefreshCw, Moon, Sun, Settings, Search, Wifi, X } from 'lucide-react';
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
      <div className="neu-card rounded-2xl p-4">
        <div className="flex items-center justify-between">
          {/* Logo and Title */}
          <div className="flex items-center gap-3">
            <div className="w-10 h-10 rounded-xl bg-gradient-to-br from-neu-primary to-neu-secondary flex items-center justify-center">
              <Wifi className="w-6 h-6 text-white" />
            </div>
            <h1 className="text-2xl font-bold bg-gradient-to-r from-neu-primary to-neu-secondary bg-clip-text text-transparent">
              NetSnip
            </h1>
          </div>

          {/* Search Bar */}
          <div className="flex-1 max-w-md mx-8">
            <div className="relative">
              <Search className="absolute left-3 top-1/2 -translate-y-1/2 w-5 h-5 text-neu-text-secondary dark:text-dark-text-secondary" />
              <input
                type="text"
                value={localSearchQuery}
                onChange={(e) => setLocalSearchQuery(e.target.value)}
                placeholder="Search devices by name, IP, MAC..."
                className="neu-input w-full pl-10 pr-10 py-2 text-sm"
              />
              {localSearchQuery && (
                <button
                  onClick={() => setLocalSearchQuery('')}
                  className="absolute right-3 top-1/2 -translate-y-1/2 text-neu-text-secondary hover:text-neu-text"
                  aria-label="Clear search"
                >
                  <X className="w-4 h-4" />
                </button>
              )}
            </div>
          </div>

          {/* Actions */}
          <div className="flex items-center gap-3">
            {/* Status Indicators */}
            <div className="flex items-center gap-4 mr-4">
              <div className="flex items-center gap-2">
                <div className="w-2 h-2 rounded-full bg-green-500 animate-pulse"></div>
                <span className="text-sm text-neu-text-secondary dark:text-dark-text-secondary">
                  {onlineDevices} Online
                </span>
              </div>
              {blockedDevices > 0 && (
                <div className="flex items-center gap-2">
                  <div className="w-2 h-2 rounded-full bg-red-500"></div>
                  <span className="text-sm text-neu-text-secondary dark:text-dark-text-secondary">
                    {blockedDevices} Blocked
                  </span>
                </div>
              )}
            </div>

            {/* Theme Toggle */}
            <button
              onClick={onToggleTheme}
              className="neu-button p-2 rounded-xl"
              aria-label="Toggle theme"
            >
              {theme === 'light' ? (
                <Moon className="w-5 h-5" />
              ) : (
                <Sun className="w-5 h-5" />
              )}
            </button>

            {/* Refresh Button */}
            <button
              onClick={() => scanNetwork()}
              disabled={scanning}
              className="neu-button p-2 rounded-xl disabled:opacity-50"
              aria-label="Refresh network"
            >
              <RefreshCw className={`w-5 h-5 ${scanning ? 'animate-spin' : ''}`} />
            </button>

            {/* Settings Button */}
            <button
              className="neu-button p-2 rounded-xl"
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