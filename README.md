# NetSnip

Network management application for macOS.

## Prerequisites

Install Rust:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

Install Node.js dependencies:
```bash
npm install
```

## Run

### Desktop App (Full Network Features)
```bash
npm run tauri:dev
```

### Web Mode (UI Development Only)
```bash
npm run dev
```

## Build

```bash
npm run tauri:build
```

## Features

- Network device discovery via ARP scanning
- Real-time bandwidth monitoring
- Device cutting/restoration via ARP spoofing
- Bandwidth limiting
- MAC vendor identification
- Gateway detection

## Architecture

- **Frontend**: React, TypeScript, Tailwind CSS, Zustand
- **Backend**: Rust, Tauri, pnet
- **Database**: SQLite

## Note

The application works best with appropriate network permissions. Some features like packet capture may require elevated privileges on macOS.