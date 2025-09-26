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

    // Get MAC vendor information
    let mac = "AA:BB:CC:DD:EE:FF";
    let manufacturer = crate::utils::mac_vendor::get_manufacturer_from_mac(mac);

    // TODO: Implement actual network scanning
    Ok(vec![
        Device {
            id: "gateway".to_string(),
            name: "Router".to_string(),
            custom_name: None,
            ip: "192.168.1.1".to_string(),
            mac: "AA:BB:CC:DD:EE:FF".to_string(),
            manufacturer: manufacturer.or(Some("Netgear".to_string())),
            device_type: "router".to_string(),
            status: "online".to_string(),
            bandwidth_current: 450.0,
            bandwidth_limit: None,
            is_gateway: true,
            is_current_device: false,
            last_seen: chrono::Utc::now(),
        }
    ])
}

#[tauri::command]
pub async fn get_network_info(state: State<'_, AppState>) -> Result<NetworkInfo, String> {
    // TODO: Implement actual network info retrieval
    Ok(NetworkInfo {
        gateway_ip: "192.168.1.1".to_string(),
        gateway_mac: "AA:BB:CC:DD:EE:FF".to_string(),
        local_ip: "192.168.1.105".to_string(),
        local_mac: "11:22:33:44:55:66".to_string(),
        subnet_mask: "255.255.255.0".to_string(),
        interface_name: "en0".to_string(),
    })
}