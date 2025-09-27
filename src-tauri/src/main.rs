#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod modules;
mod utils;

use std::sync::Arc;
use tokio::sync::Mutex;
use tauri::Manager;
use modules::scanner::NetworkScanner;
use modules::arp_controller::ArpController;
use modules::bandwidth::BandwidthController;
use modules::database::Database;
use modules::packet_monitor::PacketMonitor;
use modules::network_stats::NetworkStats;
use pnet::datalink;

pub struct AppState {
    pub scanner: Arc<Mutex<NetworkScanner>>,
    pub arp_controller: Arc<Mutex<ArpController>>,
    pub bandwidth_controller: Arc<Mutex<BandwidthController>>,
    pub database: Arc<Mutex<Database>>,
    pub packet_monitor: Arc<Mutex<Option<PacketMonitor>>>,
    pub network_stats: Arc<Mutex<NetworkStats>>,
}

impl AppState {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let database = Database::new("netsnip.db").await?;

        // Find the network interface for ARP controller
        let interface = datalink::interfaces()
            .into_iter()
            .find(|iface| {
                iface.is_up()
                    && !iface.is_loopback()
                    && iface.ips.iter().any(|ip| ip.is_ipv4())
            })
            .ok_or("No suitable network interface found")?;

        // Create ARP controller with the interface
        let arp_controller = ArpController::new(interface.clone())?;

        // Try to create packet monitor (may fail if no permissions)
        let packet_monitor = match PacketMonitor::new(Some(interface.name.clone())) {
            Ok(monitor) => {
                log::info!("Packet monitor initialized");
                Some(monitor)
            },
            Err(e) => {
                log::warn!("Could not initialize packet monitor: {}. Bandwidth monitoring will be limited.", e);
                None
            }
        };

        Ok(Self {
            scanner: Arc::new(Mutex::new(NetworkScanner::new()?)),
            arp_controller: Arc::new(Mutex::new(arp_controller)),
            bandwidth_controller: Arc::new(Mutex::new(BandwidthController::new())),
            database: Arc::new(Mutex::new(database)),
            packet_monitor: Arc::new(Mutex::new(packet_monitor)),
            network_stats: Arc::new(Mutex::new(NetworkStats::new())),
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

            // Try to start monitoring
            runtime.block_on(async {
                // First try packet monitoring if available
                let packet_monitor = app_state.packet_monitor.lock().await;
                if let Some(monitor) = packet_monitor.as_ref() {
                    if let Err(e) = monitor.start_monitoring().await {
                        log::warn!("Could not start packet monitoring: {}", e);
                    } else {
                        log::info!("Packet monitoring started successfully");
                    }
                }
                drop(packet_monitor);

                // Always start network stats monitoring as fallback
                let network_stats = app_state.network_stats.lock().await;
                if let Err(e) = network_stats.start_monitoring().await {
                    log::warn!("Could not start network stats monitoring: {}", e);
                } else {
                    log::info!("Network stats monitoring started");
                }
            });

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