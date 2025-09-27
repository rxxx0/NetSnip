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

    // TODO: Implement actual network scanning
    // For now, return empty array until network scanning is implemented
    log::info!("Network scan requested - implementation pending");

    Ok(vec![])
}

#[tauri::command]
pub async fn get_network_info(state: State<'_, AppState>) -> Result<NetworkInfo, String> {
    let scanner = state.scanner.lock().await;

    // Get actual interface info
    let (interface_name, local_mac, local_ip) = scanner.get_interface_info();

    // TODO: Implement actual gateway detection
    log::info!("Network info requested for interface: {}", interface_name);

    Ok(NetworkInfo {
        gateway_ip: String::new(),  // To be implemented with actual gateway detection
        gateway_mac: String::new(),
        local_ip,
        local_mac,
        subnet_mask: String::new(),  // To be implemented
        interface_name,
    })
}