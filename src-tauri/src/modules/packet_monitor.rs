use anyhow::Result;
use pnet::datalink::{self, Channel, NetworkInterface};
use pnet::packet::ethernet::{EthernetPacket, EtherTypes};
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::tcp::TcpPacket;
use pnet::packet::udp::UdpPacket;
use pnet::packet::Packet;
use std::collections::HashMap;
use std::net::Ipv4Addr;
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tokio::sync::Mutex;

#[derive(Debug, Clone)]
pub struct DeviceTraffic {
    pub ip: Ipv4Addr,
    pub bytes_sent: u64,
    pub bytes_received: u64,
    pub packets_sent: u64,
    pub packets_received: u64,
    pub last_update: SystemTime,
}

pub struct PacketMonitor {
    interface: NetworkInterface,
    traffic_stats: Arc<Mutex<HashMap<Ipv4Addr, DeviceTraffic>>>,
    local_ip: Ipv4Addr,
    running: Arc<Mutex<bool>>,
}

impl PacketMonitor {
    pub fn new(interface_name: Option<String>) -> Result<Self> {
        // Find the network interface to monitor
        let interface = if let Some(name) = interface_name {
            datalink::interfaces()
                .into_iter()
                .find(|iface| iface.name == name)
                .ok_or_else(|| anyhow::anyhow!("Interface {} not found", name))?
        } else {
            // Auto-select the default interface
            datalink::interfaces()
                .into_iter()
                .find(|iface| {
                    iface.is_up()
                        && !iface.is_loopback()
                        && iface.ips.iter().any(|ip| ip.is_ipv4())
                })
                .ok_or_else(|| anyhow::anyhow!("No suitable network interface found"))?
        };

        // Get the local IP address
        let local_ip = interface
            .ips
            .iter()
            .find_map(|ip| {
                if let std::net::IpAddr::V4(ipv4) = ip.ip() {
                    Some(ipv4)
                } else {
                    None
                }
            })
            .ok_or_else(|| anyhow::anyhow!("No IPv4 address found on interface"))?;

        log::info!("Packet monitor initialized for interface: {} ({})",
                   interface.name, local_ip);

        Ok(Self {
            interface,
            traffic_stats: Arc::new(Mutex::new(HashMap::new())),
            local_ip,
            running: Arc::new(Mutex::new(false)),
        })
    }

    /// Start monitoring network packets
    pub async fn start_monitoring(&self) -> Result<()> {
        let mut running = self.running.lock().await;
        if *running {
            return Err(anyhow::anyhow!("Packet monitoring already running"));
        }
        *running = true;
        drop(running);

        let interface = self.interface.clone();
        let traffic_stats = self.traffic_stats.clone();
        let local_ip = self.local_ip;
        let running = self.running.clone();

        // Spawn monitoring task
        tokio::spawn(async move {
            if let Err(e) = Self::monitor_loop(interface, traffic_stats, local_ip, running).await {
                log::error!("Packet monitoring error: {}", e);
            }
        });

        log::info!("Packet monitoring started");
        Ok(())
    }

    /// Stop monitoring network packets
    pub async fn stop_monitoring(&self) -> Result<()> {
        let mut running = self.running.lock().await;
        *running = false;
        log::info!("Packet monitoring stopped");
        Ok(())
    }

    /// Main monitoring loop
    async fn monitor_loop(
        interface: NetworkInterface,
        traffic_stats: Arc<Mutex<HashMap<Ipv4Addr, DeviceTraffic>>>,
        local_ip: Ipv4Addr,
        running: Arc<Mutex<bool>>,
    ) -> Result<()> {
        // Create a channel for receiving packets
        let (_, mut rx) = match datalink::channel(&interface, Default::default()) {
            Ok(Channel::Ethernet(tx, rx)) => (tx, rx),
            Ok(_) => return Err(anyhow::anyhow!("Unhandled channel type")),
            Err(e) => return Err(anyhow::anyhow!("Failed to create channel: {}", e)),
        };

        log::info!("Packet capture started on interface: {}", interface.name);

        // Process packets
        loop {
            // Check if we should stop
            {
                let running = running.lock().await;
                if !*running {
                    break;
                }
            }

            // Set a timeout for packet reception
            match tokio::time::timeout(Duration::from_millis(100), async {
                rx.next()
            }).await {
                Ok(Ok(packet)) => {
                    // Process the packet
                    if let Some(ethernet) = EthernetPacket::new(packet) {
                        Self::process_packet(&ethernet, &traffic_stats, local_ip).await;
                    }
                }
                Ok(Err(e)) => {
                    log::error!("Error receiving packet: {}", e);
                }
                Err(_) => {
                    // Timeout - check if we should continue
                    continue;
                }
            }
        }

        Ok(())
    }

    /// Process a captured packet
    async fn process_packet(
        ethernet: &EthernetPacket,
        traffic_stats: &Arc<Mutex<HashMap<Ipv4Addr, DeviceTraffic>>>,
        local_ip: Ipv4Addr,
    ) {
        // Only process IPv4 packets
        if ethernet.get_ethertype() != EtherTypes::Ipv4 {
            return;
        }

        if let Some(ipv4) = Ipv4Packet::new(ethernet.payload()) {
            let source_ip = ipv4.get_source();
            let dest_ip = ipv4.get_destination();
            let packet_size = ipv4.get_total_length() as u64;

            let mut stats = traffic_stats.lock().await;
            let now = SystemTime::now();

            // Track traffic for source IP
            if source_ip != local_ip && Self::is_local_network(source_ip) {
                let entry = stats.entry(source_ip).or_insert_with(|| DeviceTraffic {
                    ip: source_ip,
                    bytes_sent: 0,
                    bytes_received: 0,
                    packets_sent: 0,
                    packets_received: 0,
                    last_update: now,
                });

                // This device is sending data
                entry.bytes_sent += packet_size;
                entry.packets_sent += 1;
                entry.last_update = now;
            }

            // Track traffic for destination IP
            if dest_ip != local_ip && Self::is_local_network(dest_ip) {
                let entry = stats.entry(dest_ip).or_insert_with(|| DeviceTraffic {
                    ip: dest_ip,
                    bytes_sent: 0,
                    bytes_received: 0,
                    packets_sent: 0,
                    packets_received: 0,
                    last_update: now,
                });

                // This device is receiving data
                entry.bytes_received += packet_size;
                entry.packets_received += 1;
                entry.last_update = now;
            }
        }
    }

    /// Check if an IP is in the local network (not a public IP)
    fn is_local_network(ip: Ipv4Addr) -> bool {
        let octets = ip.octets();
        // Check for private IP ranges
        // 10.0.0.0/8
        octets[0] == 10 ||
        // 172.16.0.0/12
        (octets[0] == 172 && octets[1] >= 16 && octets[1] <= 31) ||
        // 192.168.0.0/16
        (octets[0] == 192 && octets[1] == 168) ||
        // 169.254.0.0/16 (Link-local)
        (octets[0] == 169 && octets[1] == 254)
    }

    /// Get traffic statistics for all monitored devices
    pub async fn get_traffic_stats(&self) -> HashMap<Ipv4Addr, DeviceTraffic> {
        let stats = self.traffic_stats.lock().await;
        stats.clone()
    }

    /// Get traffic statistics for a specific device
    pub async fn get_device_traffic(&self, ip: Ipv4Addr) -> Option<DeviceTraffic> {
        let stats = self.traffic_stats.lock().await;
        stats.get(&ip).cloned()
    }

    /// Calculate bandwidth for a device (in Mbps)
    pub async fn calculate_bandwidth(&self, ip: Ipv4Addr) -> Option<f64> {
        let stats = self.traffic_stats.lock().await;

        if let Some(traffic) = stats.get(&ip) {
            // Calculate time since last update
            if let Ok(elapsed) = SystemTime::now().duration_since(traffic.last_update) {
                let elapsed_secs = elapsed.as_secs_f64();

                if elapsed_secs > 0.0 && elapsed_secs < 60.0 { // Only consider recent data
                    // Total bytes transferred
                    let total_bytes = traffic.bytes_sent + traffic.bytes_received;

                    // Convert to Mbps (megabits per second)
                    let mbps = (total_bytes as f64 * 8.0) / (elapsed_secs * 1_000_000.0);

                    return Some(mbps);
                }
            }
        }

        None
    }

    /// Clear statistics for all devices
    pub async fn clear_stats(&self) {
        let mut stats = self.traffic_stats.lock().await;
        stats.clear();
    }

    /// Clear statistics for a specific device
    pub async fn clear_device_stats(&self, ip: Ipv4Addr) {
        let mut stats = self.traffic_stats.lock().await;
        stats.remove(&ip);
    }

    /// Get whether monitoring is currently running
    pub async fn is_running(&self) -> bool {
        let running = self.running.lock().await;
        *running
    }
}