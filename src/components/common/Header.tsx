import React, { useState, useEffect } from 'react';
import { RefreshCw, Moon, Sun, Search, X } from 'lucide-react';
import { useNetworkStore } from '../../stores/networkStore';
import { useDebounce } from '../../hooks/useDebounce';
import { SettingsMenu } from './SettingsMenu';

interface HeaderProps {
  theme: 'light' | 'dark';
  onToggleTheme: () => void;
}

export const Header: React.FC<HeaderProps> = ({ theme, onToggleTheme }) => {
  const { scanning, scanNetwork, searchQuery, setSearchQuery } = useNetworkStore();
  const [localSearchQuery, setLocalSearchQuery] = useState(searchQuery);
  const debouncedSearchQuery = useDebounce(localSearchQuery, 300);

  useEffect(() => {
    setSearchQuery(debouncedSearchQuery);
  }, [debouncedSearchQuery, setSearchQuery]);

  return (
    <header className="mb-6">
      <div className="neu-card p-4">
        <div className="flex items-center justify-between gap-4">
          {/* Logo - Clean and simple */}
          <h1 className="text-xl font-bold" style={{ color: 'var(--accent-primary)' }}>
            NetSnip
          </h1>

          {/* Search Bar - Smaller and refined */}
          <div className="flex-1 max-w-md">
            <div className="relative">
              <Search className="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-text-muted" />
              <input
                type="text"
                value={localSearchQuery}
                onChange={(e) => setLocalSearchQuery(e.target.value)}
                placeholder="Search devices..."
                className="neu-input w-full pl-9 pr-8 py-2 text-sm"
              />
              {localSearchQuery && (
                <button
                  onClick={() => setLocalSearchQuery('')}
                  className="absolute right-2 top-1/2 -translate-y-1/2 p-1 text-text-muted hover:text-text-secondary transition-colors"
                  aria-label="Clear search"
                >
                  <X className="w-3 h-3" />
                </button>
              )}
            </div>
          </div>

          {/* Actions - Minimal and compact */}
          <div className="flex items-center gap-1">
            {/* Refresh Button */}
            <button
              onClick={() => scanNetwork()}
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