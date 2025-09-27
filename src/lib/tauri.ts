// Tauri API initialization
import { invoke } from '@tauri-apps/api/core';

// Check if running in Tauri
export const isTauri = () => {
  return typeof window !== 'undefined' &&
         window.__TAURI__ !== undefined;
};

// Safe invoke wrapper that checks Tauri availability
export const safeInvoke = async (cmd: string, args?: any): Promise<any> => {
  console.log(`[Tauri] Attempting to invoke: ${cmd}`, args);

  if (!isTauri()) {
    console.warn(`[Tauri] Not in Tauri context, mocking ${cmd}`);
    // Return mock data for development
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
      default:
        return null;
    }
  }

  try {
    const result = await invoke(cmd, args);
    console.log(`[Tauri] ${cmd} success:`, result);
    return result;
  } catch (error) {
    console.error(`[Tauri] ${cmd} error:`, error);
    throw error;
  }
};

// Initialize Tauri on load
if (typeof window !== 'undefined') {
  window.addEventListener('DOMContentLoaded', () => {
    console.log('[Tauri] DOM loaded, checking Tauri availability...');
    console.log('[Tauri] window.__TAURI__:', window.__TAURI__);
    if (isTauri()) {
      console.log('[Tauri] ✅ Tauri API is available');
    } else {
      console.log('[Tauri] ⚠️ Tauri API not available - running in browser mode');
    }
  });
}

export default {
  isTauri,
  invoke: safeInvoke
};