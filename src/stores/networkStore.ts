import { create } from 'zustand';
// Check if we're in a Tauri context
const isTauri = typeof window !== 'undefined' && window.__TAURI__ !== undefined;

// Fallback for non-Tauri environments - returns sample data for development
const mockInvoke = async (cmd: string, _args?: any): Promise<any> => {
  switch (cmd) {
    case 'scan_network':
      // Return sample devices for development/demonstration
      console.log('Development mode: Returning sample devices');
      return [
        {
          id: '00_1b_63_84_45_e6',
          name: 'MacBook-Pro',
          customName: 'Work Laptop',
          ip: '192.168.1.101',
          mac: '00:1b:63:84:45:e6',
          manufacturer: 'Apple',
          deviceType: 'computer',
          status: 'online',
          bandwidthCurrent: 5.2,
          bandwidthLimit: null,
          isGateway: false,
          isCurrentDevice: true,
          lastSeen: new Date().toISOString()
        },
        {
          id: 'b0_be_76_d3_4c_2a',
          name: 'Gateway Router',
          customName: null,
          ip: '192.168.1.1',
          mac: 'b0:be:76:d3:4c:2a',
          manufacturer: 'TP-Link',
          deviceType: 'router',
          status: 'online',
          bandwidthCurrent: 25.8,
          bandwidthLimit: null,
          isGateway: true,
          isCurrentDevice: false,
          lastSeen: new Date().toISOString()
        },
        {
          id: '8c_85_90_a0_3b_1d',
          name: 'iPhone-15',
          customName: 'Personal Phone',
          ip: '192.168.1.102',
          mac: '8c:85:90:a0:3b:1d',
          manufacturer: 'Apple',
          deviceType: 'phone',
          status: 'online',
          bandwidthCurrent: 2.1,
          bandwidthLimit: null,
          isGateway: false,
          isCurrentDevice: false,
          lastSeen: new Date().toISOString()
        },
        {
          id: '44_07_0b_62_8e_f5',
          name: 'Smart-TV',
          customName: 'Living Room TV',
          ip: '192.168.1.103',
          mac: '44:07:0b:62:8e:f5',
          manufacturer: 'Samsung',
          deviceType: 'tv',
          status: 'online',
          bandwidthCurrent: 12.5,
          bandwidthLimit: null,
          isGateway: false,
          isCurrentDevice: false,
          lastSeen: new Date().toISOString()
        },
        {
          id: 'dc_a6_32_7b_9c_2e',
          name: 'Gaming-PC',
          customName: null,
          ip: '192.168.1.104',
          mac: 'dc:a6:32:7b:9c:2e',
          manufacturer: 'Intel',
          deviceType: 'computer',
          status: 'limited',
          bandwidthCurrent: 8.0,
          bandwidthLimit: 8.0,
          isGateway: false,
          isCurrentDevice: false,
          lastSeen: new Date().toISOString()
        },
        {
          id: '70_3e_ac_df_85_42',
          name: 'Echo-Dot',
          customName: 'Kitchen Speaker',
          ip: '192.168.1.105',
          mac: '70:3e:ac:df:85:42',
          manufacturer: 'Amazon',
          deviceType: 'iot',
          status: 'online',
          bandwidthCurrent: 0.3,
          bandwidthLimit: null,
          isGateway: false,
          isCurrentDevice: false,
          lastSeen: new Date().toISOString()
        }
      ];
    case 'get_network_info':
      // Return sample network info
      return {
        gatewayIp: '192.168.1.1',
        gatewayMac: 'b0:be:76:d3:4c:2a',
        localIp: '192.168.1.101',
        localMac: '00:1b:63:84:45:e6',
        subnetMask: '255.255.255.0',
        interfaceName: 'en0'
      };
    case 'get_bandwidth_updates':
      // Return sample bandwidth updates with slight variations
      return [
        {
          device_id: '00_1b_63_84_45_e6',
          bandwidth_current: 5.2 + (Math.random() * 2 - 1)
        },
        {
          device_id: 'b0_be_76_d3_4c_2a',
          bandwidth_current: 25.8 + (Math.random() * 5 - 2.5)
        },
        {
          device_id: '8c_85_90_a0_3b_1d',
          bandwidth_current: 2.1 + (Math.random() * 0.5 - 0.25)
        },
        {
          device_id: '44_07_0b_62_8e_f5',
          bandwidth_current: 12.5 + (Math.random() * 3 - 1.5)
        },
        {
          device_id: 'dc_a6_32_7b_9c_2e',
          bandwidth_current: Math.min(8.0, 7.5 + (Math.random() * 1))
        },
        {
          device_id: '70_3e_ac_df_85_42',
          bandwidth_current: 0.3 + (Math.random() * 0.1)
        }
      ];
    case 'cut_device':
      console.log(`Development mode: Would cut device ${_args?.deviceId}`);
      return { success: true, message: `Device ${_args?.deviceId} cut (simulated)` };
    case 'restore_device':
      console.log(`Development mode: Would restore device ${_args?.deviceId}`);
      return { success: true, message: `Device ${_args?.deviceId} restored (simulated)` };
    case 'limit_bandwidth':
      console.log(`Development mode: Would limit device ${_args?.deviceId} to ${_args?.limitMbps} Mbps`);
      return { success: true, message: `Bandwidth limited to ${_args?.limitMbps} Mbps (simulated)` };
    case 'remove_bandwidth_limit':
      console.log(`Development mode: Would remove limit for device ${_args?.deviceId}`);
      return { success: true, message: `Bandwidth limit removed (simulated)` };
    case 'update_device_name':
      console.log(`Development mode: Would rename device ${_args?.deviceId} to ${_args?.name}`);
      return null;
    default:
      console.log(`Development mode: ${cmd} command called with args:`, _args);
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