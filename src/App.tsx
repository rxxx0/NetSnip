import { useEffect, useState } from 'react';
import { Dashboard } from './components/Dashboard/Dashboard';
import { DeviceList } from './components/DeviceList/DeviceList';
import { Header } from './components/common/Header';
import { useNetworkStore } from './stores/networkStore';
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
  const { scanNetwork, getNetworkInfo, error, clearError } = useNetworkStore();

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
    };
    init();
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []); // Intentionally omit dependencies as we only want this to run once

  const toggleTheme = () => {
    setTheme(prev => prev === 'light' ? 'dark' : 'light');
  };

  return (
    <div className="min-h-screen bg-neu-bg dark:bg-dark-bg transition-colors duration-300">
      {/* Error Toast */}
      {error && (
        <div className="fixed top-4 right-4 z-50 animate-slide-up">
          <div className="neu-card bg-neu-danger/10 border border-neu-danger/30 p-4 pr-12 relative max-w-md">
            <button
              onClick={clearError}
              className="absolute top-2 right-2 text-neu-danger hover:opacity-70"
              aria-label="Close error"
            >
              ✕
            </button>
            <p className="text-neu-danger font-medium">Error</p>
            <p className="text-sm mt-1 text-neu-text dark:text-dark-text opacity-80">
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
  );
}

export default App;