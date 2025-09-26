// Global type declarations

declare global {
  interface Window {
    __TAURI__?: {
      invoke: (cmd: string, args?: Record<string, any>) => Promise<any>;
    };
  }
}

export {};