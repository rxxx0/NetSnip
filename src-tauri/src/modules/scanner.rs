use anyhow::Result;
use ipnetwork::IpNetwork;
use pnet::datalink::{self, NetworkInterface};
use pnet::packet::arp::{ArpHardwareTypes, ArpOperations, MutableArpPacket};
use pnet::packet::ethernet::{EtherTypes, MutableEthernetPacket};
use pnet::packet::Packet;
use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr};
use std::process::Command;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::time::sleep;
use crate::modules::vendor::VendorLookup;

#[derive(Clone, Debug)]
pub struct NetworkDevice {
    pub ip: Ipv4Addr,
    pub mac: String,
    pub hostname: Option<String>,
    pub manufacturer: Option<String>,
    pub device_type: String,
    pub is_gateway: bool,
}

pub struct NetworkScanner {
    interface: NetworkInterface,
    discovered_devices: Arc<Mutex<HashMap<Ipv4Addr, NetworkDevice>>>,
    vendor_lookup: VendorLookup,
}

impl NetworkScanner {
    pub fn new() -> Result<Self> {
        let interface = datalink::interfaces()
            .into_iter()
            .find(|iface| {
                iface.is_up()
                    && !iface.is_loopback()
                    && iface.ips.iter().any(|ip| ip.is_ipv4())
            })
            .ok_or_else(|| anyhow::anyhow!("No suitable network interface found"))?;

        log::info!("Using network interface: {}", interface.name);

        Ok(Self {
            interface,
            discovered_devices: Arc::new(Mutex::new(HashMap::new())),
            vendor_lookup: VendorLookup::new(),
        })
    }

    /// Get the default gateway using macOS route command
    pub async fn get_gateway(&self) -> Result<(Ipv4Addr, String)> {
        let output = Command::new("route")
            .args(&["-n", "get", "default"])
            .output()?;

        let output_str = String::from_utf8_lossy(&output.stdout);
        let mut gateway_ip = None;

        // Parse the output to find the gateway IP
        for line in output_str.lines() {
            if line.trim().starts_with("gateway:") {
                if let Some(ip_str) = line.split_whitespace().nth(1) {
                    gateway_ip = ip_str.parse::<Ipv4Addr>().ok();
                    break;
                }
            }
        }

        let gateway_ip = gateway_ip.ok_or_else(|| anyhow::anyhow!("Could not determine gateway IP"))?;

        // Get the MAC address of the gateway using arp
        let gateway_mac = self.get_mac_for_ip(gateway_ip).await?;

        Ok((gateway_ip, gateway_mac))
    }

    /// Get MAC address for an IP using the arp command
    pub async fn get_mac_for_ip(&self, ip: Ipv4Addr) -> Result<String> {
        // First, try to ping the IP to ensure it's in the ARP cache
        let _ = Command::new("ping")
            .args(&["-c", "1", "-W", "1000", &ip.to_string()])
            .output();

        // Give it a moment to populate the ARP cache
        sleep(Duration::from_millis(100)).await;

        // Now check the ARP table
        let output = Command::new("arp")
            .arg("-n")
            .arg(ip.to_string())
            .output()?;

        let output_str = String::from_utf8_lossy(&output.stdout);

        // Parse the ARP output to find the MAC address
        for line in output_str.lines() {
            if line.contains(&ip.to_string()) {
                // MAC address is typically the third field
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 4 {
                    let mac = parts[3];
                    if mac != "(incomplete)" && mac.contains(':') {
                        return Ok(mac.to_string());
                    }
                }
            }
        }

        Err(anyhow::anyhow!("Could not determine MAC address for {}", ip))
    }

    /// Scan the local subnet for devices using ARP
    pub async fn scan_network(&self) -> Result<Vec<NetworkDevice>> {
        let mut devices = Vec::new();

        // Get our IP and subnet
        let local_ip = self.get_local_ip()?;
        let subnet_mask = self.get_subnet_mask()?;

        // Calculate the network range
        let network = IpNetwork::new(IpAddr::V4(local_ip), subnet_mask)?;

        log::info!("Scanning network: {}", network);

        // Get the gateway first
        let (gateway_ip, _gateway_mac) = match self.get_gateway().await {
            Ok(gw) => {
                log::info!("Gateway detected: {} ({})", gw.0, gw.1);
                gw
            },
            Err(e) => {
                log::warn!("Could not determine gateway: {}", e);
                (Ipv4Addr::new(0, 0, 0, 0), String::new())
            }
        };

        // First, use arp -a to get all known devices
        let output = Command::new("arp")
            .arg("-a")
            .output()?;

        let output_str = String::from_utf8_lossy(&output.stdout);

        // Parse the ARP table
        for line in output_str.lines() {
            // Skip header and empty lines
            if line.is_empty() || line.contains("incomplete") {
                continue;
            }

            // Parse each line to extract IP and MAC
            // Format: hostname (IP) at MAC on interface [flags]
            if let Some(start) = line.find('(') {
                if let Some(end) = line.find(')') {
                    let ip_str = &line[start + 1..end];
                    if let Ok(ip) = ip_str.parse::<Ipv4Addr>() {
                        // Skip if not in our network
                        if !network.contains(IpAddr::V4(ip)) {
                            continue;
                        }

                        // Find MAC address (after "at")
                        if let Some(at_pos) = line.find(" at ") {
                            let after_at = &line[at_pos + 4..];
                            if let Some(mac_end) = after_at.find(" on ") {
                                let mac = &after_at[..mac_end];

                                // Extract hostname if available
                                let hostname = if start > 0 {
                                    Some(line[..start].trim().to_string())
                                } else {
                                    None
                                };

                                let manufacturer = self.vendor_lookup.lookup(mac);
                                let device_type = self.vendor_lookup.get_device_type(mac, hostname.as_ref());

                                devices.push(NetworkDevice {
                                    ip,
                                    mac: mac.to_string(),
                                    hostname,
                                    manufacturer,
                                    device_type,
                                    is_gateway: ip == gateway_ip,
                                });
                            }
                        }
                    }
                }
            }
        }

        // Also scan the network range with ping to discover more devices
        log::info!("Performing ping sweep...");
        let scan_tasks = self.ping_sweep(&network).await;

        // Merge the results
        let mut devices_map: HashMap<Ipv4Addr, NetworkDevice> = devices
            .into_iter()
            .map(|d| (d.ip, d.clone()))
            .collect();

        for ip in scan_tasks {
            if !devices_map.contains_key(&ip) {
                // Try to get MAC for newly discovered IP
                if let Ok(mac) = self.get_mac_for_ip(ip).await {
                    let manufacturer = self.vendor_lookup.lookup(&mac);
                    let device_type = self.vendor_lookup.get_device_type(&mac, None);
                    devices_map.insert(
                        ip,
                        NetworkDevice {
                            ip,
                            mac,
                            hostname: None,
                            manufacturer,
                            device_type,
                            is_gateway: ip == gateway_ip,
                        },
                    );
                }
            }
        }

        // Store discovered devices
        let mut discovered = self.discovered_devices.lock().await;
        *discovered = devices_map.clone();

        let result: Vec<NetworkDevice> = devices_map.into_values().collect();
        log::info!("Network scan complete. Found {} devices", result.len());

        Ok(result)
    }

    /// Perform a ping sweep to discover active hosts
    async fn ping_sweep(&self, network: &IpNetwork) -> Vec<Ipv4Addr> {
        let mut active_ips = Vec::new();
        let mut handles = Vec::new();

        for ip in network.iter() {
            if let IpAddr::V4(ipv4) = ip {
                // Skip network and broadcast addresses
                if ipv4 == network.network() || ipv4 == network.broadcast() {
                    continue;
                }

                let ip_copy = ipv4;
                let handle = tokio::spawn(async move {
                    // Quick ping with 1 second timeout (macOS uses -t for TTL, -W for timeout in ms)
                    let output = Command::new("ping")
                        .args(&["-c", "1", "-W", "1000", "-t", "1", &ip_copy.to_string()])
                        .output();

                    match output {
                        Ok(out) if out.status.success() => Some(ip_copy),
                        _ => None,
                    }
                });

                handles.push(handle);

                // Limit concurrent pings to avoid overwhelming the system
                if handles.len() >= 30 {
                    for handle in handles.drain(..) {
                        if let Ok(Some(ip)) = handle.await {
                            active_ips.push(ip);
                        }
                    }
                }
            }
        }

        // Wait for remaining handles
        for handle in handles {
            if let Ok(Some(ip)) = handle.await {
                active_ips.push(ip);
            }
        }

        log::info!("Ping sweep found {} active hosts", active_ips.len());
        active_ips
    }

    /// Get the local IP address
    fn get_local_ip(&self) -> Result<Ipv4Addr> {
        self.interface
            .ips
            .iter()
            .find_map(|ip| {
                if let IpAddr::V4(ipv4) = ip.ip() {
                    Some(ipv4)
                } else {
                    None
                }
            })
            .ok_or_else(|| anyhow::anyhow!("No IPv4 address found on interface"))
    }

    /// Get the subnet mask
    fn get_subnet_mask(&self) -> Result<u8> {
        self.interface
            .ips
            .iter()
            .find_map(|ip| {
                if ip.is_ipv4() {
                    Some(ip.prefix())
                } else {
                    None
                }
            })
            .ok_or_else(|| anyhow::anyhow!("No subnet mask found"))
    }

    pub fn get_interface_info(&self) -> (String, String, String) {
        let name = self.interface.name.clone();
        let mac = self.interface.mac.map(|m| m.to_string()).unwrap_or_default();
        let ip = self
            .interface
            .ips
            .iter()
            .find(|ip| ip.is_ipv4())
            .map(|ip| ip.ip().to_string())
            .unwrap_or_default();

        (name, mac, ip)
    }

    pub async fn get_discovered_devices(&self) -> Vec<NetworkDevice> {
        let devices = self.discovered_devices.lock().await;
        devices.values().cloned().collect()
    }

    #[allow(dead_code)]
    fn create_arp_request_packet(
        source_ip: Ipv4Addr,
        source_mac: [u8; 6],
        target_ip: Ipv4Addr,
    ) -> Vec<u8> {
        let mut ethernet_buffer = vec![0u8; 42];
        let mut ethernet_packet = MutableEthernetPacket::new(&mut ethernet_buffer).unwrap();

        ethernet_packet.set_destination([0xff, 0xff, 0xff, 0xff, 0xff, 0xff].into());
        ethernet_packet.set_source(source_mac.into());
        ethernet_packet.set_ethertype(EtherTypes::Arp);

        let mut arp_buffer = vec![0u8; 28];
        let mut arp_packet = MutableArpPacket::new(&mut arp_buffer).unwrap();

        arp_packet.set_hardware_type(ArpHardwareTypes::Ethernet);
        arp_packet.set_protocol_type(EtherTypes::Ipv4);
        arp_packet.set_hw_addr_len(6);
        arp_packet.set_proto_addr_len(4);
        arp_packet.set_operation(ArpOperations::Request);
        arp_packet.set_sender_hw_addr(source_mac.into());
        arp_packet.set_sender_proto_addr(source_ip);
        arp_packet.set_target_hw_addr([0, 0, 0, 0, 0, 0].into());
        arp_packet.set_target_proto_addr(target_ip);

        ethernet_packet.set_payload(arp_packet.packet());
        ethernet_buffer
    }
}