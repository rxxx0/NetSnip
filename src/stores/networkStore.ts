import { create } from 'zustand';
// Check if we're in a Tauri context
const isTauri = typeof window !== 'undefined' && window.__TAURI__ !== undefined;

// Fallback for non-Tauri environments with mock data
const mockInvoke = async (cmd: string, _args?: any): Promise<any> => {
  switch (cmd) {
    case 'scan_network':
      // Return mock devices for testing
      return [
        {
          id: 'gateway',
          name: 'Netgear Nighthawk',
          customName: 'Home Router',
          ip: '192.168.1.1',
          mac: 'E0:46:9A:2B:1C:3D',
          manufacturer: 'Netgear Inc.',
          deviceType: 'router',
          status: 'online',
          bandwidthCurrent: 523.7,
          bandwidthLimit: null,
          isGateway: true,
          isCurrentDevice: false,
          lastSeen: new Date().toISOString(),
        },
        {
          id: 'device-1',
          name: 'MacBook Pro',
          customName: 'My Laptop',
          ip: '192.168.1.105',
          mac: '3C:22:FB:91:45:2E',
          manufacturer: 'Apple, Inc.',
          deviceType: 'computer',
          status: 'online',
          bandwidthCurrent: 45.3,
          bandwidthLimit: null,
          isGateway: false,
          isCurrentDevice: true,
          lastSeen: new Date().toISOString(),
        },
        {
          id: 'device-2',
          name: 'iPhone 15 Pro',
          customName: null,
          ip: '192.168.1.112',
          mac: 'A8:51:AB:C3:9F:7B',
          manufacturer: 'Apple, Inc.',
          deviceType: 'phone',
          status: 'online',
          bandwidthCurrent: 2.8,
          bandwidthLimit: 10.0,
          isGateway: false,
          isCurrentDevice: false,
          lastSeen: new Date().toISOString(),
        },
        {
          id: 'device-3',
          name: 'Samsung Smart TV',
          customName: 'Living Room TV',
          ip: '192.168.1.134',
          mac: '58:3A:FD:2E:91:C4',
          manufacturer: 'Samsung Electronics',
          deviceType: 'tv',
          status: 'online',
          bandwidthCurrent: 18.4,
          bandwidthLimit: null,
          isGateway: false,
          isCurrentDevice: false,
          lastSeen: new Date().toISOString(),
        },
        {
          id: 'device-4',
          name: 'iPad Air',
          customName: null,
          ip: '192.168.1.156',
          mac: 'DC:56:E7:12:8A:3F',
          manufacturer: 'Apple, Inc.',
          deviceType: 'tablet',
          status: 'blocked',
          bandwidthCurrent: 0.0,
          bandwidthLimit: null,
          isGateway: false,
          isCurrentDevice: false,
          lastSeen: new Date(Date.now() - 300000).toISOString(),
        },
        {
          id: 'device-5',
          name: 'Echo Dot',
          customName: 'Alexa',
          ip: '192.168.1.178',
          mac: 'B4:7C:9C:8E:2D:1A',
          manufacturer: 'Amazon Technologies Inc.',
          deviceType: 'iot',
          status: 'online',
          bandwidthCurrent: 0.3,
          bandwidthLimit: null,
          isGateway: false,
          isCurrentDevice: false,
          lastSeen: new Date().toISOString(),
        },
      ];
    case 'get_network_info':
      return {
        gatewayIp: '192.168.1.1',
        gatewayMac: 'E0:46:9A:2B:1C:3D',
        localIp: '192.168.1.105',
        localMac: '3C:22:FB:91:45:2E',
        subnetMask: '255.255.255.0',
        interfaceName: 'en0'
      };
    default:
      console.log(`Mock: ${cmd} command called with args:`, _args);
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

  scanNetwork: () => Promise<void>;
  getNetworkInfo: () => Promise<void>;
  cutDevice: (deviceId: string) => Promise<void>;
  restoreDevice: (deviceId: string) => Promise<void>;
  limitBandwidth: (deviceId: string, limitMbps: number) => Promise<void>;
  updateDeviceName: (deviceId: string, name: string) => Promise<void>;
  selectDevice: (device: Device | null) => void;
  clearError: () => void;
  setSearchQuery: (query: string) => void;
  setFilter: (filter: 'all' | 'online' | 'blocked' | 'limited') => void;
  getFilteredDevices: () => Device[];
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
    const device = get().devices.find(d => d.id === deviceId);

    // Safety check for current device
    if (device?.isCurrentDevice) {
      const confirmed = window.confirm(
        'Warning: You are about to disconnect your own device. This will interrupt your network connection. Are you sure?'
      );
      if (!confirmed) return;
    }

    // Safety check for gateway
    if (device?.isGateway) {
      const confirmed = window.confirm(
        'Warning: Disconnecting the gateway will affect ALL devices on the network. Are you absolutely sure?'
      );
      if (!confirmed) return;
    }

    set({ loading: true, error: null });
    try {
      await invoke('cut_device', { deviceId });

      // Update device status
      set(state => ({
        devices: state.devices.map((d: Device) =>
          d.id === deviceId ? { ...d, status: 'blocked' as const } : d
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

      // Update device status
      set(state => ({
        devices: state.devices.map((d: Device) =>
          d.id === deviceId ? { ...d, status: 'online' as const } : d
        ),
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
        devices: state.devices.map((d: Device) =>
          d.id === deviceId
            ? { ...d, bandwidthLimit: limitMbps, status: 'limited' as const }
            : d
        ),
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
}));