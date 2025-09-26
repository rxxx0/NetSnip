import { create } from 'zustand';
// Check if we're in a Tauri context
const isTauri = typeof window !== 'undefined' && window.__TAURI__ !== undefined;

// Fallback for non-Tauri environments
const mockInvoke = async (cmd: string, _args?: any): Promise<any> => {
  console.warn(`NetSnip: Running in web mode - ${cmd} command not available`);

  switch (cmd) {
    case 'scan_network':
      return [];
    case 'get_network_info':
      return null;
    default:
      throw new Error(`NetSnip requires the desktop app for network operations`);
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
  deviceType: 'router' | 'computer' | 'phone' | 'tablet' | 'iot' | 'unknown';
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

  scanNetwork: () => Promise<void>;
  getNetworkInfo: () => Promise<void>;
  cutDevice: (deviceId: string) => Promise<void>;
  restoreDevice: (deviceId: string) => Promise<void>;
  limitBandwidth: (deviceId: string, limitMbps: number) => Promise<void>;
  updateDeviceName: (deviceId: string, name: string) => Promise<void>;
  selectDevice: (device: Device | null) => void;
  clearError: () => void;
  setSearchQuery: (query: string) => void;
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