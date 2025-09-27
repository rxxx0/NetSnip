import { useEffect, useState } from 'react';
import { Dashboard } from './components/Dashboard/Dashboard';
import { DeviceList } from './components/DeviceList/DeviceList';
import { Header } from './components/common/Header';
import { useNetworkStore } from './stores/networkStore';
import { NotificationProvider } from './hooks/useNotification';
import './styles/globals.css';

function App() {
  // Initialize theme from localStorage or system preference
  const getInitialTheme = (): 'light' | 'dark' => {
    const stored = localStorage.getItem('theme');
    if (stored === 'light' || stored === 'dark') {
      return stored;
    }
    // Check system preference
    if (window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches) {
      return 'dark';
    }
    return 'light';
  };

  const [theme, setTheme] = useState<'light' | 'dark'>(getInitialTheme);
  const { scanNetwork, getNetworkInfo, error, clearError, startPolling, stopPolling } = useNetworkStore();

  useEffect(() => {
    // Apply theme to document
    if (theme === 'dark') {
      document.documentElement.classList.add('dark');
    } else {
      document.documentElement.classList.remove('dark');
    }
    // Save theme preference
    localStorage.setItem('theme', theme);
  }, [theme]);

  useEffect(() => {
    // Initial network scan
    const init = async () => {
      await getNetworkInfo();
      await scanNetwork();
      // Start bandwidth polling after initial scan
      startPolling();
    };
    init();

    // Cleanup: stop polling when component unmounts
    return () => {
      stopPolling();
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []); // Intentionally omit dependencies as we only want this to run once

  const toggleTheme = () => {
    setTheme(prev => prev === 'light' ? 'dark' : 'light');
  };

  return (
    <NotificationProvider>
      <div className="min-h-screen bg-[var(--neu-bg)] transition-colors duration-300">
        {/* Error Toast */}
        {error && (
          <div className="fixed top-4 right-4 z-50 animate-fade-in">
            <div className="neu-card border border-red-500/30 p-4 pr-12 relative max-w-md" style={{
              background: 'linear-gradient(145deg, var(--accent-danger)11, var(--accent-danger)22)'
            }}>
              <button
                onClick={clearError}
                className="absolute top-2 right-2 text-red-500 hover:text-red-600 transition-colors"
                aria-label="Close error"
              >
                ✕
              </button>
              <p className="font-medium" style={{ color: 'var(--accent-danger)' }}>Error</p>
              <p className="text-sm mt-1 text-text-secondary">
                {error}
              </p>
            </div>
          </div>
        )}

        <div className="container mx-auto px-4 py-6">
          {/* Header */}
          <Header theme={theme} onToggleTheme={toggleTheme} />

          {/* Dashboard */}
          <Dashboard />

          {/* Device List */}
          <DeviceList />
        </div>
      </div>
    </NotificationProvider>
  );
}

export default App;