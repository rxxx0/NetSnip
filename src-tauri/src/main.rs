#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod modules;
mod utils;

use std::sync::Arc;
use tokio::sync::Mutex;
use tauri::State;
use modules::scanner::NetworkScanner;
use modules::arp_controller::ArpController;
use modules::bandwidth::BandwidthController;
use modules::database::Database;

pub struct AppState {
    scanner: Arc<Mutex<NetworkScanner>>,
    arp_controller: Arc<Mutex<Option<ArpController>>>,
    bandwidth_controller: Arc<Mutex<BandwidthController>>,
    database: Arc<Mutex<Database>>,
}

impl AppState {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let database = Database::new("netsnip.db").await?;

        Ok(Self {
            scanner: Arc::new(Mutex::new(NetworkScanner::new()?)),
            arp_controller: Arc::new(Mutex::new(None)),
            bandwidth_controller: Arc::new(Mutex::new(BandwidthController::new())),
            database: Arc::new(Mutex::new(database)),
        })
    }
}

fn main() {
    env_logger::init();

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            let runtime = tokio::runtime::Runtime::new()?;
            let app_state = runtime.block_on(AppState::new())?;
            app.manage(app_state);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::network::scan_network,
            commands::network::get_network_info,
            commands::device::get_devices,
            commands::device::cut_device,
            commands::device::restore_device,
            commands::device::limit_bandwidth,
            commands::device::remove_bandwidth_limit,
            commands::device::get_bandwidth_updates,
            commands::device::update_device_name,
            commands::settings::get_settings,
            commands::settings::update_settings,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}