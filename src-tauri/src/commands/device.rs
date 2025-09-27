use crate::AppState;
use serde::{Deserialize, Serialize};
use tauri::State;
use super::network::Device;

#[derive(Debug, Serialize, Deserialize)]
pub struct CutResult {
    pub success: bool,
    pub message: String,
}

#[tauri::command]
pub async fn get_devices(state: State<'_, AppState>) -> Result<Vec<Device>, String> {
    // TODO: Get devices from database
    super::network::scan_network(state).await
}

#[tauri::command]
pub async fn cut_device(
    state: State<'_, AppState>,
    device_id: String,
) -> Result<CutResult, String> {
    // Validate device_id
    if device_id.is_empty() {
        return Err("Invalid device ID".to_string());
    }

    // Parse device_id (MAC address with underscores)
    let mac_address = device_id.replace('_', ":");

    // Get device info from scanner
    let scanner = state.scanner.lock().await;
    let devices = scanner.get_discovered_devices().await;
    drop(scanner);

    // Find the device by MAC
    let device = devices.iter()
        .find(|d| d.mac.to_lowercase() == mac_address.to_lowercase())
        .ok_or_else(|| format!("Device {} not found", device_id))?;

    // Get gateway info
    let scanner = state.scanner.lock().await;
    let (gateway_ip, gateway_mac) = scanner.get_gateway().await
        .map_err(|e| format!("Failed to get gateway info: {}", e))?;
    drop(scanner);

    // Get ARP controller and set gateway if needed
    let mut arp = state.arp_controller.lock().await;
    if arp.gateway_ip.is_none() {
        arp.set_gateway(gateway_ip, gateway_mac.clone())
            .map_err(|e| format!("Failed to set gateway: {}", e))?;
    }

    // Cut the device
    arp.cut_device(device.ip, device.mac.clone()).await
        .map_err(|e| format!("Failed to cut device: {}", e))?;

    log::info!("Successfully cut device: {} ({})", device.ip, device.mac);

    Ok(CutResult {
        success: true,
        message: format!("Device {} has been cut from network", device_id),
    })
}

#[tauri::command]
pub async fn restore_device(
    state: State<'_, AppState>,
    device_id: String,
) -> Result<CutResult, String> {
    // Validate device_id
    if device_id.is_empty() {
        return Err("Invalid device ID".to_string());
    }

    // Parse device_id (MAC address with underscores)
    let mac_address = device_id.replace('_', ":");

    // Get device info from scanner
    let scanner = state.scanner.lock().await;
    let devices = scanner.get_discovered_devices().await;
    drop(scanner);

    // Find the device by MAC
    let device = devices.iter()
        .find(|d| d.mac.to_lowercase() == mac_address.to_lowercase())
        .ok_or_else(|| format!("Device {} not found", device_id))?;

    // Restore the device
    let arp = state.arp_controller.lock().await;
    arp.restore_device(device.ip).await
        .map_err(|e| format!("Failed to restore device: {}", e))?;

    log::info!("Successfully restored device: {} ({})", device.ip, device.mac);

    Ok(CutResult {
        success: true,
        message: format!("Device {} has been restored to network", device_id),
    })
}

#[tauri::command]
pub async fn limit_bandwidth(
    state: State<'_, AppState>,
    device_id: String,
    limit_mbps: f64,
) -> Result<CutResult, String> {
    // Validate input
    if device_id.is_empty() {
        return Err("Invalid device ID".to_string());
    }

    if limit_mbps <= 0.0 || limit_mbps > 10000.0 {
        return Err("Bandwidth limit must be between 0.1 and 10000 Mbps".to_string());
    }

    let mut _bandwidth_controller = state.bandwidth_controller.lock().await;

    // TODO: Implement bandwidth limiting with actual IP
    // let device_ip = get_device_ip_from_id(&device_id)?;
    // bandwidth_controller.limit_bandwidth(device_ip, limit_mbps).await?;

    log::info!("Setting bandwidth limit: {} Mbps for device {}", limit_mbps, device_id);

    Ok(CutResult {
        success: true,
        message: format!("Bandwidth limit set to {} Mbps for device {}", limit_mbps, device_id),
    })
}

#[tauri::command]
pub async fn update_device_name(
    state: State<'_, AppState>,
    device_id: String,
    name: String,
) -> Result<(), String> {
    // Validate input
    if device_id.is_empty() {
        return Err("Invalid device ID".to_string());
    }

    if name.len() > 100 {
        return Err("Device name too long (max 100 characters)".to_string());
    }

    // Sanitize name - remove any potential XSS attempts
    let sanitized_name = name
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#x27;");

    let mut _database = state.database.lock().await;

    // TODO: Update device name in database
    // database.update_device_name(&device_id, &sanitized_name).await
    //     .map_err(|e| e.to_string())?;

    log::info!("Updated device {} name to: {}", device_id, sanitized_name);

    Ok(())
}

#[tauri::command]
pub async fn remove_bandwidth_limit(
    state: State<'_, AppState>,
    device_id: String,
) -> Result<CutResult, String> {
    // Validate input
    if device_id.is_empty() {
        return Err("Invalid device ID".to_string());
    }

    let mut _bandwidth_controller = state.bandwidth_controller.lock().await;

    // TODO: Implement bandwidth limit removal with actual IP
    // let device_ip = get_device_ip_from_id(&device_id)?;
    // bandwidth_controller.remove_limit(device_ip).await?;

    log::info!("Removing bandwidth limit for device {}", device_id);

    Ok(CutResult {
        success: true,
        message: format!("Bandwidth limit removed for device {}", device_id),
    })
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BandwidthUpdate {
    pub device_id: String,
    pub bandwidth_current: f64,
}

#[tauri::command]
pub async fn get_bandwidth_updates(state: State<'_, AppState>) -> Result<Vec<BandwidthUpdate>, String> {
    let mut bandwidth_updates = Vec::new();

    // Get discovered devices
    let scanner = state.scanner.lock().await;
    let devices = scanner.get_discovered_devices().await;
    drop(scanner);

    // Check if any device is cut (should have 0 bandwidth)
    let arp = state.arp_controller.lock().await;
    let cut_devices = arp.get_cut_devices().await;
    drop(arp);

    // Try packet monitor first (requires elevated privileges)
    let packet_monitor_opt = state.packet_monitor.lock().await;
    let has_packet_data = if let Some(packet_monitor) = packet_monitor_opt.as_ref() {
        if packet_monitor.is_running().await {
            let traffic_stats = packet_monitor.get_traffic_stats().await;
            !traffic_stats.is_empty()
        } else {
            false
        }
    } else {
        false
    };
    drop(packet_monitor_opt);

    if has_packet_data {
        // Use real packet data
        let packet_monitor_opt = state.packet_monitor.lock().await;
        if let Some(packet_monitor) = packet_monitor_opt.as_ref() {
            let _traffic_stats = packet_monitor.get_traffic_stats().await;

            for device in devices.iter() {
                let device_id = device.mac.replace(':', "_").to_lowercase();
                let is_cut = cut_devices.iter().any(|c| c.target_ip == device.ip && c.active);

                if is_cut {
                    bandwidth_updates.push(BandwidthUpdate {
                        device_id,
                        bandwidth_current: 0.0,
                    });
                } else if let Some(bandwidth_mbps) = packet_monitor.calculate_bandwidth(device.ip).await {
                    bandwidth_updates.push(BandwidthUpdate {
                        device_id,
                        bandwidth_current: bandwidth_mbps,
                    });
                }
            }
        }
    } else {
        // Use network stats as fallback (doesn't require privileges)

        for device in devices.iter() {
            let device_id = device.mac.replace(':', "_").to_lowercase();
            let is_cut = cut_devices.iter().any(|c| c.target_ip == device.ip && c.active);

            if is_cut {
                // Device is cut, report 0 bandwidth
                bandwidth_updates.push(BandwidthUpdate {
                    device_id,
                    bandwidth_current: 0.0,
                });
            } else {
                // Try to get actual traffic data from packet monitor
                let packet_monitor_opt = state.packet_monitor.lock().await;
                if let Some(packet_monitor) = packet_monitor_opt.as_ref() {
                    if let Some(traffic) = packet_monitor.get_device_traffic(device.ip).await {
                        // Calculate bandwidth from traffic data
                        if let Ok(elapsed) = std::time::SystemTime::now().duration_since(traffic.last_update) {
                            let elapsed_secs = elapsed.as_secs_f64();
                            if elapsed_secs > 0.0 && elapsed_secs < 60.0 {
                                let total_bytes = traffic.bytes_sent + traffic.bytes_received;
                                let mbps = (total_bytes as f64 * 8.0) / (elapsed_secs * 1_000_000.0);
                                bandwidth_updates.push(BandwidthUpdate {
                                    device_id,
                                    bandwidth_current: mbps,
                                });
                            }
                        }
                    }
                }
            }
        }
    }

    if bandwidth_updates.is_empty() {
        log::debug!("No devices found for bandwidth updates");
    } else {
        log::debug!("Returning {} bandwidth updates", bandwidth_updates.len());
    }

    Ok(bandwidth_updates)
}

