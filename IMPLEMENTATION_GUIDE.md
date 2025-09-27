# NetSnip Implementation Guide

## Overview
NetSnip is a macOS network management application built with Tauri (Rust backend) and React (TypeScript frontend). The app provides real-time network device discovery, bandwidth monitoring, and network control features.

## Running the Application

### Web Development Mode
```bash
npm run dev
```
- Runs at http://localhost:5173
- Uses **sample data** for demonstration
- No actual network access
- Perfect for UI development and testing

### Tauri Desktop Mode
```bash
npm run tauri:dev
```
- Runs as a native macOS application
- **Full network features** available
- Actual device discovery and monitoring
- May require elevated privileges for some features

### Production Build
```bash
npm run tauri:build
```
- Creates a distributable macOS application

## Architecture

### Frontend (TypeScript/React)
- **src/stores/networkStore.ts**: State management with Zustand
  - Detects if running in Tauri or web mode
  - Provides mock data for development
  - Real data in Tauri mode

### Backend (Rust/Tauri)
- **modules/scanner.rs**: Network device discovery
  - Uses macOS system commands (arp, ping, route)
  - Gateway detection
  - ARP table parsing
  - Ping sweep for device discovery

- **modules/vendor.rs**: MAC vendor lookup
  - Database of common manufacturers
  - Device type detection

- **modules/network_stats.rs**: Bandwidth monitoring (fallback)
  - Works without elevated privileges
  - Simulates bandwidth for demonstration

- **modules/packet_monitor.rs**: Advanced packet capture
  - Requires elevated privileges (sudo)
  - Real bandwidth measurement
  - Currently disabled due to permission requirements

- **modules/arp_controller.rs**: Network control
  - Device cutting/restoration framework
  - Safety checks prevent self-cutting

## Features by Mode

### Web Development Mode (npm run dev)
✅ Full UI functionality
✅ Sample devices with realistic data
✅ Bandwidth animations
✅ Cut/restore simulation
✅ Bandwidth limiting simulation
❌ No real network access
❌ No actual device discovery

### Tauri Desktop Mode (npm run tauri:dev)
✅ Real network device discovery
✅ Actual gateway detection
✅ MAC vendor identification
✅ Device type detection
✅ Simulated bandwidth (for safety)
⚠️ Limited packet capture (needs sudo)
⚠️ ARP spoofing disabled (for safety)

## Permission Requirements

### Standard Permissions (Works out of the box)
- Network interface detection
- ARP table reading
- Ping operations
- Route table access

### Elevated Permissions (Requires sudo)
- Packet capture for real bandwidth monitoring
- ARP spoofing for device cutting
- Raw socket access

## Safety Features

1. **Self-Protection**: Cannot cut your own device
2. **Gateway Protection**: Cannot cut the gateway/router
3. **Graceful Fallback**: Works without elevated privileges
4. **Development Mode**: Safe sample data for testing

## Known Limitations

1. **Bandwidth Monitoring**: Currently shows simulated data for safety
   - Real monitoring requires packet capture (elevated privileges)
   - Can be enabled by running with sudo (not recommended)

2. **Device Cutting**: Framework in place but disabled for safety
   - Would require ARP spoofing
   - Needs elevated privileges
   - Ethical considerations

3. **macOS Only**: Uses macOS-specific commands
   - `route -n get default` for gateway
   - `arp -a` for ARP table
   - macOS ping syntax

## Development Tips

1. **UI Development**: Use `npm run dev` with sample data
2. **Network Testing**: Use `npm run tauri:dev` for real devices
3. **Check Mode**: Console shows which mode is active
4. **Error Handling**: Check console for permission errors

## Security Considerations

- The app includes potentially dangerous network manipulation code (ARP spoofing)
- These features are intentionally limited/disabled for safety
- Only use on networks you own and have permission to manage
- Be aware of legal and ethical implications

## Troubleshooting

### No Devices Showing (Tauri Mode)
1. Check network connection
2. Ensure Wi-Fi is enabled
3. Check console for errors
4. Try running with sudo (development only)

### No Devices Showing (Web Mode)
- This should show sample devices automatically
- Check browser console for errors
- Ensure localStorage is enabled

### Permission Errors
- Most features work without elevated privileges
- Packet capture requires sudo (not recommended)
- Check console for specific permission errors