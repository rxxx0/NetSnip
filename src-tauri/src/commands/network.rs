use crate::AppState;
use serde::{Deserialize, Serialize};
use tauri::State;

#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkInfo {
    pub gateway_ip: String,
    pub gateway_mac: String,
    pub local_ip: String,
    pub local_mac: String,
    pub subnet_mask: String,
    pub interface_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Device {
    pub id: String,
    pub name: String,
    pub custom_name: Option<String>,
    pub ip: String,
    pub mac: String,
    pub manufacturer: Option<String>,
    pub device_type: String,
    pub status: String,
    pub bandwidth_current: f64,
    pub bandwidth_limit: Option<f64>,
    pub is_gateway: bool,
    pub is_current_device: bool,
    pub last_seen: chrono::DateTime<chrono::Utc>,
}

#[tauri::command]
pub async fn scan_network(state: State<'_, AppState>) -> Result<Vec<Device>, String> {
    let scanner = state.scanner.lock().await;

    log::info!("Starting network scan...");

    // Perform the actual network scan
    let scanned_devices = scanner.scan_network().await
        .map_err(|e| format!("Failed to scan network: {}", e))?;

    // Get our own IP for comparison
    let (_, _, our_ip) = scanner.get_interface_info();

    // Convert NetworkDevice to Device format expected by frontend
    let devices: Vec<Device> = scanned_devices
        .into_iter()
        .map(|device| {
            // Generate a unique ID from the MAC address
            let device_id = device.mac.replace(':', "_").to_lowercase();

            // Determine if this is our device
            let is_current = our_ip.contains(&device.ip.to_string());

            // Determine device status (all devices start as online)
            let status = "online".to_string();

            Device {
                id: device_id,
                name: device.hostname.clone().unwrap_or_else(|| {
                    // Generate a default name based on device type and last octet of IP
                    let last_octet = device.ip.octets()[3];
                    format!("{}-{}", device.device_type, last_octet)
                }),
                custom_name: None,
                ip: device.ip.to_string(),
                mac: device.mac,
                manufacturer: device.manufacturer,
                device_type: device.device_type,
                status,
                bandwidth_current: 0.0, // Will be updated by bandwidth monitoring
                bandwidth_limit: None,
                is_gateway: device.is_gateway,
                is_current_device: is_current,
                last_seen: chrono::Utc::now(),
            }
        })
        .collect();

    log::info!("Network scan complete. Found {} devices", devices.len());

    Ok(devices)
}

#[tauri::command]
pub async fn get_network_info(state: State<'_, AppState>) -> Result<NetworkInfo, String> {
    let scanner = state.scanner.lock().await;

    // Get actual interface info
    let (interface_name, local_mac, local_ip) = scanner.get_interface_info();

    // Get gateway info
    let (gateway_ip, gateway_mac) = match scanner.get_gateway().await {
        Ok(gateway) => (gateway.0.to_string(), gateway.1),
        Err(e) => {
            log::warn!("Could not get gateway info: {}", e);
            (String::new(), String::new())
        }
    };

    // TODO: Get actual subnet mask from interface
    let subnet_mask = "255.255.255.0".to_string(); // Common default

    log::info!("Network info: Interface: {}, IP: {}, Gateway: {}",
              interface_name, local_ip, gateway_ip);

    Ok(NetworkInfo {
        gateway_ip,
        gateway_mac,
        local_ip,
        local_mac,
        subnet_mask,
        interface_name,
    })
}