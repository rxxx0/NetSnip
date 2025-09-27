import { create } from 'zustand';
// Check if we're in a Tauri context
const isTauri = typeof window !== 'undefined' && window.__TAURI__ !== undefined;

// Fallback for non-Tauri environments
const mockInvoke = async (cmd: string, _args?: any): Promise<any> => {
  switch (cmd) {
    case 'scan_network':
      return [];
    case 'get_network_info':
      return {
        gatewayIp: '',
        gatewayMac: '',
        localIp: '',
        localMac: '',
        subnetMask: '',
        interfaceName: ''
      };
    case 'get_bandwidth_updates':
      return [];
    case 'cut_device':
      return { success: false, message: 'Not available in web mode' };
    case 'restore_device':
      return { success: false, message: 'Not available in web mode' };
    case 'limit_bandwidth':
      return { success: false, message: 'Not available in web mode' };
    case 'remove_bandwidth_limit':
      return { success: false, message: 'Not available in web mode' };
    case 'update_device_name':
      return null;
    default:
      return null;
  }
};

// Use window.__TAURI__ if available, otherwise use mock
const invoke = isTauri && window.__TAURI__
  ? window.__TAURI__.invoke
  : mockInvoke;

export interface Device {
  id: string;
  name: string;
  customName?: string;
  ip: string;
  mac: string;
  manufacturer?: string;
  deviceType: 'router' | 'computer' | 'phone' | 'tablet' | 'iot' | 'tv' | 'gaming' | 'unknown';
  status: 'online' | 'offline' | 'blocked' | 'limited';
  bandwidthCurrent: number;
  bandwidthLimit?: number;
  isGateway: boolean;
  isCurrentDevice: boolean;
  lastSeen: Date;
}

interface NetworkInfo {
  gatewayIp: string;
  gatewayMac: string;
  localIp: string;
  localMac: string;
  subnetMask: string;
  interfaceName: string;
}

interface NetworkStore {
  devices: Device[];
  networkInfo: NetworkInfo | null;
  scanning: boolean;
  loading: boolean;
  error: string | null;
  selectedDevice: Device | null;
  searchQuery: string;
  filter: 'all' | 'online' | 'blocked' | 'limited';
  pollingInterval: number | null;

  scanNetwork: () => Promise<void>;
  getNetworkInfo: () => Promise<void>;
  cutDevice: (deviceId: string) => Promise<void>;
  restoreDevice: (deviceId: string) => Promise<void>;
  limitBandwidth: (deviceId: string, limitMbps: number) => Promise<void>;
  removeBandwidthLimit: (deviceId: string) => Promise<void>;
  updateDeviceName: (deviceId: string, name: string) => Promise<void>;
  selectDevice: (device: Device | null) => void;
  clearError: () => void;
  setSearchQuery: (query: string) => void;
  setFilter: (filter: 'all' | 'online' | 'blocked' | 'limited') => void;
  getFilteredDevices: () => Device[];
  startPolling: () => void;
  stopPolling: () => void;
  updateBandwidth: () => void;
}

export const useNetworkStore = create<NetworkStore>((set, get) => ({
  devices: [],
  networkInfo: null,
  scanning: false,
  loading: false,
  error: null,
  selectedDevice: null,
  searchQuery: '',
  filter: 'all',
  pollingInterval: null,

  scanNetwork: async () => {
    set({ scanning: true, error: null });
    try {
      const devices = await invoke('scan_network') as Device[];
      set({
        devices: devices.map((d: any) => ({
          ...d,
          lastSeen: new Date(d.lastSeen as any)
        })),
        scanning: false
      });
    } catch (error) {
      set({ error: String(error), scanning: false });
    }
  },

  getNetworkInfo: async () => {
    set({ loading: true, error: null });
    try {
      const info = await invoke('get_network_info') as NetworkInfo;
      set({ networkInfo: info, loading: false });
    } catch (error) {
      set({ error: String(error), loading: false });
    }
  },

  cutDevice: async (deviceId: string) => {
    set({ loading: true, error: null });
    try {
      await invoke('cut_device', { deviceId });

      // Update device status and set bandwidth to 0
      set(state => ({
        devices: state.devices.map((d: Device) =>
          d.id === deviceId ? { ...d, status: 'blocked' as const, bandwidthCurrent: 0 } : d
        ),
        loading: false
      }));
    } catch (error) {
      set({ error: String(error), loading: false });
    }
  },

  restoreDevice: async (deviceId: string) => {
    set({ loading: true, error: null });
    try {
      await invoke('restore_device', { deviceId });

      // Update device status and restore some bandwidth
      set(state => ({
        devices: state.devices.map((d: Device) => {
          if (d.id === deviceId) {
            // Restore with a reasonable bandwidth value
            const restoredBandwidth = d.bandwidthLimit
              ? d.bandwidthLimit * 0.5 // Start at 50% of limit if limited
              : Math.random() * 20 + 5; // Random between 5-25 MB/s otherwise
            return { ...d, status: 'online' as const, bandwidthCurrent: parseFloat(restoredBandwidth.toFixed(1)) };
          }
          return d;
        }),
        loading: false
      }));
    } catch (error) {
      set({ error: String(error), loading: false });
    }
  },

  limitBandwidth: async (deviceId: string, limitMbps: number) => {
    set({ loading: true, error: null });
    try {
      await invoke('limit_bandwidth', { deviceId, limitMbps });

      // Update device bandwidth limit
      set(state => ({
        devices: state.devices.map((d: Device) => {
          if (d.id === deviceId) {
            // Apply limit and adjust current bandwidth if it exceeds the limit
            const newBandwidth = Math.min(d.bandwidthCurrent, limitMbps);
            return {
              ...d,
              bandwidthLimit: limitMbps,
              bandwidthCurrent: parseFloat(newBandwidth.toFixed(1)),
              status: 'limited' as const
            };
          }
          return d;
        }),
        loading: false
      }));
    } catch (error) {
      set({ error: String(error), loading: false });
    }
  },

  removeBandwidthLimit: async (deviceId: string) => {
    set({ loading: true, error: null });
    try {
      await invoke('remove_bandwidth_limit', { deviceId });

      // Remove bandwidth limit from device
      set(state => ({
        devices: state.devices.map((d: Device) => {
          if (d.id === deviceId) {
            return {
              ...d,
              bandwidthLimit: undefined,
              status: 'online' as const
            };
          }
          return d;
        }),
        loading: false
      }));
    } catch (error) {
      set({ error: String(error), loading: false });
    }
  },

  updateDeviceName: async (deviceId: string, name: string) => {
    set({ loading: true, error: null });
    try {
      await invoke('update_device_name', { deviceId, name });

      // Update device name
      set(state => ({
        devices: state.devices.map((d: Device) =>
          d.id === deviceId ? { ...d, customName: name } : d
        ),
        loading: false
      }));
    } catch (error) {
      set({ error: String(error), loading: false });
    }
  },

  selectDevice: (device: Device | null) => {
    set({ selectedDevice: device });
  },

  clearError: () => {
    set({ error: null });
  },

  setSearchQuery: (query: string) => {
    set({ searchQuery: query });
  },

  setFilter: (filter: 'all' | 'online' | 'blocked' | 'limited') => {
    set({ filter });
  },

  getFilteredDevices: () => {
    const state = get();
    const query = state.searchQuery.toLowerCase().trim();

    if (!query) {
      return state.devices;
    }

    return state.devices.filter(device => {
      return (
        device.name.toLowerCase().includes(query) ||
        device.customName?.toLowerCase().includes(query) ||
        device.ip.includes(query) ||
        device.mac.toLowerCase().includes(query) ||
        device.manufacturer?.toLowerCase().includes(query) ||
        device.deviceType.toLowerCase().includes(query)
      );
    });
  },

  updateBandwidth: async () => {
    try {
      // Use actual backend command if available
      if (isTauri && window.__TAURI__) {
        const updates = await invoke('get_bandwidth_updates') as Array<{
          device_id: string;
          bandwidth_current: number;
        }>;

        set(state => ({
          devices: state.devices.map(device => {
            // Find bandwidth update for this device
            const update = updates.find(u => u.device_id === device.id);

            if (update) {
              // Apply bandwidth limits
              let newBandwidth = Math.max(0, update.bandwidth_current);

              if (device.status === 'blocked') {
                newBandwidth = 0;
              } else if (device.bandwidthLimit && newBandwidth > device.bandwidthLimit) {
                newBandwidth = device.bandwidthLimit * (0.85 + Math.random() * 0.1);
              }

              return {
                ...device,
                bandwidthCurrent: parseFloat(newBandwidth.toFixed(1))
              };
            }

            // No update for this device - keep current value with slight variation
            if (device.status === 'blocked') {
              return { ...device, bandwidthCurrent: 0 };
            }

            return device;
          })
        }));
      } else {
        // In development mode without Tauri, bandwidth stays at 0
        console.log('Development mode: No bandwidth updates available');
      }
    } catch (error) {
      console.error('Failed to update bandwidth:', error);
    }
  },

  startPolling: () => {
    const state = get();

    // Clear existing interval if any
    if (state.pollingInterval) {
      clearInterval(state.pollingInterval);
    }

    // Start new polling interval (update every 20 seconds)
    const interval = setInterval(() => {
      get().updateBandwidth();
    }, 20000) as unknown as number;

    set({ pollingInterval: interval });

    // Initial update
    get().updateBandwidth();
  },

  stopPolling: () => {
    const state = get();

    if (state.pollingInterval) {
      clearInterval(state.pollingInterval);
      set({ pollingInterval: null });
    }
  },
}));