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
  const { devices, scanNetwork, getNetworkInfo, error, clearError, startPolling, stopPolling } = useNetworkStore();

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
    // Initialize network scan on app start
    const timer = setTimeout(async () => {
      console.log('App initialized, starting network scan...');
      try {
        await getNetworkInfo();
        await scanNetwork();
        // Start bandwidth polling after initial scan
        startPolling();
      } catch (error) {
        console.error('Failed to initialize network scan:', error);
      }
    }, 1000); // Give app time to initialize

    // Cleanup
    return () => {
      clearTimeout(timer);
      stopPolling();
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []); // Intentionally omit dependencies as we only want this to run once

  const toggleTheme = () => {
    setTheme(prev => prev === 'light' ? 'dark' : 'light');
  };

  // Add state for scan status
  const [scanStatus, setScanStatus] = useState<string>('Not started');

  // Add a test button for debugging
  const testScan = async () => {
    setScanStatus('Starting network scan...');
    console.log('Test scan clicked');
    try {
      console.log('Calling scanNetwork from Tauri...');
      await scanNetwork();
      setScanStatus('✅ Scan completed! Check devices below.');
      console.log('Scan completed successfully');
    } catch (error) {
      console.error('Scan error:', error);
      setScanStatus(`❌ Scan error: ${error}`);
      // Check if error suggests we're not in Tauri
      if (String(error).includes('not available') || String(error).includes('Failed to fetch')) {
        alert('Not running in Tauri window. Make sure you\'re using the NetSnip app, not a browser.');
      } else {
        alert('Error scanning: ' + error);
      }
    }
  };

  return (
    <NotificationProvider>
      <div className="min-h-screen bg-[var(--neu-bg)] transition-colors duration-300">
        {/* Debug Test Button and Status */}
        <div className="fixed top-4 left-4 z-50 space-y-2">
          <button
            onClick={testScan}
            className="bg-blue-500 hover:bg-blue-600 text-white px-4 py-2 rounded-lg shadow-lg"
          >
            🔍 Test Scan Network
          </button>
          <div className="bg-black/80 text-white px-4 py-2 rounded-lg">
            <div className="text-xs font-mono">Status: {scanStatus}</div>
            <div className="text-xs font-mono">Devices found: {devices.length}</div>
          </div>
        </div>

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