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

    // Return realistic mock data with varied device types
    Ok(vec![
        Device {
            id: "gateway".to_string(),
            name: "Netgear Nighthawk".to_string(),
            custom_name: Some("Home Router".to_string()),
            ip: "192.168.1.1".to_string(),
            mac: "E0:46:9A:2B:1C:3D".to_string(),
            manufacturer: Some("Netgear Inc.".to_string()),
            device_type: "router".to_string(),
            status: "online".to_string(),
            bandwidth_current: 523.7,
            bandwidth_limit: None,
            is_gateway: true,
            is_current_device: false,
            last_seen: chrono::Utc::now(),
        },
        Device {
            id: "device-1".to_string(),
            name: "MacBook Pro".to_string(),
            custom_name: Some("My Laptop".to_string()),
            ip: "192.168.1.105".to_string(),
            mac: "3C:22:FB:91:45:2E".to_string(),
            manufacturer: Some("Apple, Inc.".to_string()),
            device_type: "computer".to_string(),
            status: "online".to_string(),
            bandwidth_current: 45.3,
            bandwidth_limit: None,
            is_gateway: false,
            is_current_device: true,
            last_seen: chrono::Utc::now(),
        },
        Device {
            id: "device-2".to_string(),
            name: "iPhone 15 Pro".to_string(),
            custom_name: None,
            ip: "192.168.1.112".to_string(),
            mac: "A8:51:AB:C3:9F:7B".to_string(),
            manufacturer: Some("Apple, Inc.".to_string()),
            device_type: "phone".to_string(),
            status: "online".to_string(),
            bandwidth_current: 2.8,
            bandwidth_limit: Some(10.0),
            is_gateway: false,
            is_current_device: false,
            last_seen: chrono::Utc::now(),
        },
        Device {
            id: "device-3".to_string(),
            name: "Samsung Smart TV".to_string(),
            custom_name: Some("Living Room TV".to_string()),
            ip: "192.168.1.134".to_string(),
            mac: "58:3A:FD:2E:91:C4".to_string(),
            manufacturer: Some("Samsung Electronics".to_string()),
            device_type: "tv".to_string(),
            status: "online".to_string(),
            bandwidth_current: 18.4,
            bandwidth_limit: None,
            is_gateway: false,
            is_current_device: false,
            last_seen: chrono::Utc::now(),
        },
        Device {
            id: "device-4".to_string(),
            name: "iPad Air".to_string(),
            custom_name: None,
            ip: "192.168.1.156".to_string(),
            mac: "DC:56:E7:12:8A:3F".to_string(),
            manufacturer: Some("Apple, Inc.".to_string()),
            device_type: "tablet".to_string(),
            status: "blocked".to_string(),
            bandwidth_current: 0.0,
            bandwidth_limit: None,
            is_gateway: false,
            is_current_device: false,
            last_seen: chrono::Utc::now() - chrono::Duration::minutes(5),
        },
        Device {
            id: "device-5".to_string(),
            name: "Echo Dot".to_string(),
            custom_name: Some("Alexa".to_string()),
            ip: "192.168.1.178".to_string(),
            mac: "B4:7C:9C:8E:2D:1A".to_string(),
            manufacturer: Some("Amazon Technologies Inc.".to_string()),
            device_type: "iot".to_string(),
            status: "online".to_string(),
            bandwidth_current: 0.3,
            bandwidth_limit: None,
            is_gateway: false,
            is_current_device: false,
            last_seen: chrono::Utc::now(),
        },
        Device {
            id: "device-6".to_string(),
            name: "PlayStation 5".to_string(),
            custom_name: None,
            ip: "192.168.1.189".to_string(),
            mac: "00:D9:D1:A7:5B:9C".to_string(),
            manufacturer: Some("Sony Interactive Entertainment".to_string()),
            device_type: "gaming".to_string(),
            status: "limited".to_string(),
            bandwidth_current: 35.2,
            bandwidth_limit: Some(50.0),
            is_gateway: false,
            is_current_device: false,
            last_seen: chrono::Utc::now(),
        },
        Device {
            id: "device-7".to_string(),
            name: "Dell XPS Desktop".to_string(),
            custom_name: Some("Work PC".to_string()),
            ip: "192.168.1.201".to_string(),
            mac: "F8:B1:56:D4:3E:7A".to_string(),
            manufacturer: Some("Dell Inc.".to_string()),
            device_type: "computer".to_string(),
            status: "online".to_string(),
            bandwidth_current: 12.7,
            bandwidth_limit: None,
            is_gateway: false,
            is_current_device: false,
            last_seen: chrono::Utc::now(),
        },
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