use anyhow::Result;
use std::collections::HashMap;
use std::net::Ipv4Addr;
use std::process::Command;
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tokio::sync::Mutex;
use tokio::time::sleep;

#[derive(Debug, Clone)]
pub struct DeviceBandwidth {
    pub ip: Ipv4Addr,
    pub mac: String,
    pub bytes_sent: u64,
    pub bytes_received: u64,
    pub last_measurement: SystemTime,
    pub bandwidth_mbps: f64,
}

pub struct NetworkStats {
    device_stats: Arc<Mutex<HashMap<String, DeviceBandwidth>>>,
    monitoring_active: Arc<Mutex<bool>>,
}

impl NetworkStats {
    pub fn new() -> Self {
        Self {
            device_stats: Arc::new(Mutex::new(HashMap::new())),
            monitoring_active: Arc::new(Mutex::new(false)),
        }
    }

    /// Start monitoring network statistics using system tools
    pub async fn start_monitoring(&self) -> Result<()> {
        let mut active = self.monitoring_active.lock().await;
        if *active {
            return Ok(());
        }
        *active = true;
        drop(active);

        let device_stats = self.device_stats.clone();
        let monitoring_active = self.monitoring_active.clone();

        // Spawn monitoring task
        tokio::spawn(async move {
            log::info!("Starting network statistics monitoring");

            loop {
                // Check if we should stop
                {
                    let active = monitoring_active.lock().await;
                    if !*active {
                        break;
                    }
                }

                // Get network statistics using netstat or nettop
                if let Ok(stats) = Self::get_network_stats() {
                    let mut device_stats = device_stats.lock().await;

                    for (mac, new_stats) in stats {
                        // Calculate bandwidth based on previous measurement
                        if let Some(prev_stats) = device_stats.get(&mac) {
                            if let Ok(elapsed) = new_stats.last_measurement.duration_since(prev_stats.last_measurement) {
                                let elapsed_secs = elapsed.as_secs_f64();
                                if elapsed_secs > 0.0 {
                                    let bytes_diff = (new_stats.bytes_sent + new_stats.bytes_received)
                                        .saturating_sub(prev_stats.bytes_sent + prev_stats.bytes_received);

                                    // Convert to MB/s
                                    let bandwidth_mbps = (bytes_diff as f64) / (elapsed_secs * 1_000_000.0);

                                    let mut updated_stats = new_stats.clone();
                                    updated_stats.bandwidth_mbps = bandwidth_mbps;
                                    device_stats.insert(mac.clone(), updated_stats);
                                    continue;
                                }
                            }
                        }

                        device_stats.insert(mac, new_stats);
                    }
                }

                // Wait before next measurement
                sleep(Duration::from_secs(5)).await;
            }

            log::info!("Network statistics monitoring stopped");
        });

        Ok(())
    }

    /// Stop monitoring
    pub async fn stop_monitoring(&self) -> Result<()> {
        let mut active = self.monitoring_active.lock().await;
        *active = false;
        Ok(())
    }

    /// Get current network statistics from system
    fn get_network_stats() -> Result<HashMap<String, DeviceBandwidth>> {
        let mut stats = HashMap::new();

        // Try to get statistics from ARP table and estimate based on connection count
        // This is a simplified approach that doesn't require elevated privileges

        // Get ARP table
        let output = Command::new("arp")
            .arg("-a")
            .output()?;

        let output_str = String::from_utf8_lossy(&output.stdout);

        for line in output_str.lines() {
            if line.is_empty() || line.contains("incomplete") {
                continue;
            }

            // Parse each line to extract IP and MAC
            if let Some(start) = line.find('(') {
                if let Some(end) = line.find(')') {
                    let ip_str = &line[start + 1..end];
                    if let Ok(ip) = ip_str.parse::<Ipv4Addr>() {
                        // Find MAC address (after "at")
                        if let Some(at_pos) = line.find(" at ") {
                            let after_at = &line[at_pos + 4..];
                            if let Some(mac_end) = after_at.find(" on ") {
                                let mac = after_at[..mac_end].to_string();

                                // Create a basic stat entry
                                // Without packet capture, we can't get real bandwidth
                                // but we can track device presence
                                stats.insert(mac.clone(), DeviceBandwidth {
                                    ip,
                                    mac,
                                    bytes_sent: 0,
                                    bytes_received: 0,
                                    last_measurement: SystemTime::now(),
                                    bandwidth_mbps: 0.0,
                                });
                            }
                        }
                    }
                }
            }
        }

        Ok(stats)
    }

    /// Get bandwidth for a specific device by MAC
    pub async fn get_device_bandwidth(&self, mac: &str) -> Option<f64> {
        let stats = self.device_stats.lock().await;
        stats.get(mac).map(|s| s.bandwidth_mbps)
    }

    /// Get all device bandwidth updates
    pub async fn get_all_bandwidth(&self) -> Vec<(String, f64)> {
        let stats = self.device_stats.lock().await;
        stats.iter()
            .map(|(mac, stat)| (mac.clone(), stat.bandwidth_mbps))
            .collect()
    }

}