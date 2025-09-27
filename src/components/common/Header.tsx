import React, { useState, useEffect, useRef } from 'react';
import { RefreshCw, Moon, Sun, Search, X } from 'lucide-react';
import { useNetworkStore } from '../../stores/networkStore';
import { useDebounce } from '../../hooks/useDebounce';
import { SettingsMenu } from './SettingsMenu';
import { useClickOutside } from '../../hooks/useClickOutside';

interface HeaderProps {
  theme: 'light' | 'dark';
  onToggleTheme: () => void;
}

export const Header: React.FC<HeaderProps> = ({ theme, onToggleTheme }) => {
  const { scanning, scanNetwork, searchQuery, setSearchQuery, getFilteredDevices, selectDevice } = useNetworkStore();
  const [localSearchQuery, setLocalSearchQuery] = useState(searchQuery);
  const [showResults, setShowResults] = useState(false);
  const debouncedSearchQuery = useDebounce(localSearchQuery, 300);
  const searchRef = useRef<HTMLDivElement>(null);

  useClickOutside(searchRef, () => setShowResults(false), showResults);

  useEffect(() => {
    setSearchQuery(debouncedSearchQuery);
    setShowResults(debouncedSearchQuery.length > 0);
  }, [debouncedSearchQuery, setSearchQuery]);

  const searchResults = getFilteredDevices();

  return (
    <header className="mb-6">
      <div className="neu-card p-4">
        <div className="flex items-center justify-between gap-4">
          {/* Logo - Clean and simple */}
          <h1 className="text-xl font-bold" style={{ color: 'var(--accent-primary)' }}>
            NetSnip
          </h1>

          {/* Search Bar - Smaller and refined */}
          <div className="flex-1 max-w-md" ref={searchRef}>
            <div className="relative">
              <Search className="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-text-muted" />
              <input
                type="text"
                value={localSearchQuery}
                onChange={(e) => {
                  setLocalSearchQuery(e.target.value);
                  setShowResults(e.target.value.length > 0);
                }}
                onFocus={() => setShowResults(localSearchQuery.length > 0)}
                placeholder="Search devices..."
                className="neu-input w-full pl-9 pr-8 py-2 text-sm"
              />
              {localSearchQuery && (
                <button
                  onClick={() => {
                    setLocalSearchQuery('');
                    setShowResults(false);
                  }}
                  className="absolute right-2 top-1/2 -translate-y-1/2 p-1 text-text-muted hover:text-text-secondary transition-colors"
                  aria-label="Clear search"
                >
                  <X className="w-3 h-3" />
                </button>
              )}

              {/* Search Results Dropdown */}
              {showResults && searchResults.length > 0 && (
                <div className="absolute top-full mt-2 w-full neu-card rounded-lg p-2 max-h-64 overflow-y-auto z-50 animate-slide-down">
                  {searchResults.slice(0, 5).map(device => (
                    <div
                      key={device.id}
                      className="p-2 hover:bg-gray-100 dark:hover:bg-gray-800 rounded cursor-pointer transition-colors"
                      onClick={() => {
                        selectDevice(device);
                        setLocalSearchQuery('');
                        setShowResults(false);
                      }}
                    >
                      <div className="flex items-center justify-between">
                        <div>
                          <div className="font-medium text-sm text-text-primary">
                            {device.customName || device.name}
                          </div>
                          <div className="text-xs text-gray-500 font-mono">
                            {device.ip} • {device.mac}
                          </div>
                        </div>
                        <div className={`text-xs px-2 py-0.5 rounded-full capitalize ${
                          device.status === 'blocked' ? 'bg-red-500/10 text-red-500' :
                          device.status === 'limited' ? 'bg-yellow-500/10 text-yellow-500' :
                          'bg-green-500/10 text-green-500'
                        }`}>
                          {device.status}
                        </div>
                      </div>
                    </div>
                  ))}
                </div>
              )}
            </div>
          </div>

          {/* Actions - Minimal and compact */}
          <div className="flex items-center gap-1">
            {/* Refresh Button */}
            <button
              onClick={async () => {
                console.log('Refresh button clicked');
                console.log('window.__TAURI__ available:', !!window.__TAURI__);
                console.log('window.__TAURI__ value:', window.__TAURI__);
                try {
                  await scanNetwork();
                  console.log('Scan completed');
                } catch (error) {
                  console.error('Scan failed:', error);
                }
              }}
              disabled={scanning}
              className={`neu-button p-2 rounded-lg ${scanning ? 'neu-pressed' : ''}`}
              aria-label="Refresh network scan"
            >
              <RefreshCw className={`w-4 h-4 ${scanning ? 'animate-spin' : ''}`} />
            </button>

            {/* Theme Toggle */}
            <button
              onClick={onToggleTheme}
              className="neu-button p-2 rounded-lg"
              aria-label={`Switch to ${theme === 'light' ? 'dark' : 'light'} mode`}
            >
              {theme === 'light' ? (
                <Moon className="w-4 h-4" />
              ) : (
                <Sun className="w-4 h-4" />
              )}
            </button>

            {/* Settings Menu */}
            <SettingsMenu />
          </div>
        </div>
      </div>
    </header>
  );
};