import { describe, it, expect, beforeEach, vi } from 'vitest';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import App from '../App';
import { useNetworkStore } from '../stores/networkStore';

// Mock the network store
vi.mock('../stores/networkStore');

describe('NetSnip App', () => {
  beforeEach(() => {
    vi.clearAllMocks();

    // Mock store implementation
    (useNetworkStore as any).mockReturnValue({
      devices: [],
      networkInfo: null,
      scanning: false,
      loading: false,
      error: null,
      selectedDevice: null,
      searchQuery: '',
      scanNetwork: vi.fn(),
      getNetworkInfo: vi.fn(),
      cutDevice: vi.fn(),
      restoreDevice: vi.fn(),
      limitBandwidth: vi.fn(),
      updateDeviceName: vi.fn(),
      selectDevice: vi.fn(),
      clearError: vi.fn(),
      setSearchQuery: vi.fn(),
      getFilteredDevices: vi.fn(() => []),
    });
  });

  it('should render the application', () => {
    render(<App />);
    expect(screen.getByText(/NetSnip/i)).toBeInTheDocument();
  });

  it('should toggle theme', () => {
    render(<App />);
    const themeButton = screen.getByLabelText(/toggle theme/i);

    // Check initial theme
    expect(document.documentElement.classList.contains('dark')).toBe(false);

    // Toggle to dark
    fireEvent.click(themeButton);
    expect(document.documentElement.classList.contains('dark')).toBe(true);

    // Toggle back to light
    fireEvent.click(themeButton);
    expect(document.documentElement.classList.contains('dark')).toBe(false);
  });

  it('should display network overview', () => {
    render(<App />);
    expect(screen.getByText(/Network Overview/i)).toBeInTheDocument();
    expect(screen.getByText(/Total Devices/i)).toBeInTheDocument();
    expect(screen.getByText(/Online Now/i)).toBeInTheDocument();
    expect(screen.getByText(/Blocked/i)).toBeInTheDocument();
    expect(screen.getByText(/Bandwidth/i)).toBeInTheDocument();
  });

  it('should display error messages', () => {
    (useNetworkStore as any).mockReturnValue({
      ...useNetworkStore(),
      error: 'Test error message',
      clearError: vi.fn(),
    });

    render(<App />);
    expect(screen.getByText(/Test error message/i)).toBeInTheDocument();
  });

  it('should handle network scanning', async () => {
    const scanNetwork = vi.fn();
    const getNetworkInfo = vi.fn();

    (useNetworkStore as any).mockReturnValue({
      ...useNetworkStore(),
      scanNetwork,
      getNetworkInfo,
      scanning: true,
    });

    render(<App />);

    // Should call network functions on mount
    await waitFor(() => {
      expect(getNetworkInfo).toHaveBeenCalled();
      expect(scanNetwork).toHaveBeenCalled();
    });

    // Should show scanning state
    expect(screen.getByText(/Scanning network/i)).toBeInTheDocument();
  });

  it('should display devices when available', () => {
    const mockDevices = [
      {
        id: 'device-1',
        name: 'Test Router',
        customName: null,
        ip: '192.168.1.1',
        mac: 'AA:BB:CC:DD:EE:FF',
        manufacturer: 'TestCorp',
        deviceType: 'router' as const,
        status: 'online' as const,
        bandwidthCurrent: 100,
        bandwidthLimit: null,
        isGateway: true,
        isCurrentDevice: false,
        lastSeen: new Date(),
      },
    ];

    (useNetworkStore as any).mockReturnValue({
      ...useNetworkStore(),
      devices: mockDevices,
      getFilteredDevices: vi.fn(() => mockDevices),
    });

    render(<App />);
    expect(screen.getByText(/Test Router/i)).toBeInTheDocument();
    expect(screen.getByText(/192.168.1.1/i)).toBeInTheDocument();
  });
});