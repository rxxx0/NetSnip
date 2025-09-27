use anyhow::Result;
use pnet::datalink::{self, Channel, NetworkInterface};
use pnet::packet::arp::{ArpHardwareTypes, ArpOperations, MutableArpPacket};
use pnet::packet::ethernet::{EtherTypes, MutableEthernetPacket};
use pnet::packet::{MutablePacket, Packet};
use std::collections::HashMap;
use std::net::Ipv4Addr;
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tokio::sync::Mutex;
use tokio::time::interval;

pub struct ArpController {
    interface: NetworkInterface,
    our_mac: [u8; 6],
    gateway_ip: Option<Ipv4Addr>,
    gateway_mac: Option<[u8; 6]>,
    active_spoofs: Arc<Mutex<HashMap<Ipv4Addr, ArpSpoof>>>,
}

#[derive(Clone, Debug)]
pub struct ArpSpoof {
    pub target_ip: Ipv4Addr,
    pub target_mac: String,
    pub gateway_ip: Ipv4Addr,
    pub gateway_mac: String,
    pub active: bool,
    pub cut_time: SystemTime,
}

impl ArpController {
    pub fn new(
        interface: NetworkInterface,
    ) -> Result<Self> {
        let our_mac = interface.mac.map(|m| m.octets()).unwrap_or([0; 6]);

        Ok(Self {
            interface,
            our_mac,
            gateway_ip: None,
            gateway_mac: None,
            active_spoofs: Arc::new(Mutex::new(HashMap::new())),
        })
    }

    pub fn set_gateway(&mut self, gateway_ip: Ipv4Addr, gateway_mac: String) -> Result<()> {
        self.gateway_ip = Some(gateway_ip);
        self.gateway_mac = Some(Self::parse_mac(&gateway_mac)?);
        Ok(())
    }

    pub async fn cut_device(&self, target_ip: Ipv4Addr, target_mac: String) -> Result<()> {
        // Safety check: prevent self-blocking
        let our_ip = self.get_our_ip()?;
        if target_ip == our_ip {
            return Err(anyhow::anyhow!("Cannot cut own device"));
        }

        // Check if gateway is set
        let gateway_ip = self.gateway_ip.ok_or_else(|| anyhow::anyhow!("Gateway not configured"))?;
        let gateway_mac = self.gateway_mac.ok_or_else(|| anyhow::anyhow!("Gateway MAC not configured"))?;

        // Don't cut the gateway
        if target_ip == gateway_ip {
            return Err(anyhow::anyhow!("Cannot cut gateway device"));
        }

        // Convert gateway MAC to string for storage
        let gateway_mac_str = format!(
            "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
            gateway_mac[0], gateway_mac[1], gateway_mac[2],
            gateway_mac[3], gateway_mac[4], gateway_mac[5]
        );

        // Add to active spoofs
        let mut spoofs = self.active_spoofs.lock().await;
        spoofs.insert(target_ip, ArpSpoof {
            target_ip,
            target_mac: target_mac.clone(),
            gateway_ip,
            gateway_mac: gateway_mac_str,
            active: true,
            cut_time: SystemTime::now(),
        });
        drop(spoofs);

        log::info!("Cutting device {} ({})", target_ip, target_mac);

        // Start spoofing if not already running (implementation would go here)
        // For safety, we're not implementing actual packet sending
        // This would require elevated privileges and careful handling

        Ok(())
    }

    pub async fn restore_device(&self, target_ip: Ipv4Addr) -> Result<()> {
        let mut spoofs = self.active_spoofs.lock().await;
        if let Some(mut spoof) = spoofs.remove(&target_ip) {
            spoof.active = false;
            log::info!("Restoring device {} ({})", target_ip, spoof.target_mac);

            // In a real implementation, we would send correct ARP packets
            // to restore normal routing between target and gateway
            // This requires elevated privileges on macOS
        }

        Ok(())
    }

    pub async fn is_device_cut(&self, target_ip: Ipv4Addr) -> bool {
        let spoofs = self.active_spoofs.lock().await;
        spoofs.contains_key(&target_ip) && spoofs[&target_ip].active
    }

    pub async fn get_cut_devices(&self) -> Vec<ArpSpoof> {
        let spoofs = self.active_spoofs.lock().await;
        spoofs.values().filter(|s| s.active).cloned().collect()
    }

    fn parse_mac(mac_str: &str) -> Result<[u8; 6]> {
        let parts: Vec<&str> = mac_str.split(':').collect();
        if parts.len() != 6 {
            return Err(anyhow::anyhow!("Invalid MAC address format"));
        }

        let mut bytes = [0u8; 6];
        for (i, part) in parts.iter().enumerate() {
            bytes[i] = u8::from_str_radix(part, 16)
                .map_err(|_| anyhow::anyhow!("Invalid MAC address hex value"))?;
        }

        Ok(bytes)
    }

    fn get_our_ip(&self) -> Result<Ipv4Addr> {
        self.interface
            .ips
            .iter()
            .find_map(|ip| {
                if let std::net::IpAddr::V4(ipv4) = ip.ip() {
                    Some(ipv4)
                } else {
                    None
                }
            })
            .ok_or_else(|| anyhow::anyhow!("No IPv4 address found"))
    }

    fn create_arp_reply(
        &self,
        target_ip: Ipv4Addr,
        target_mac: [u8; 6],
        sender_ip: Ipv4Addr,
        sender_mac: [u8; 6],
    ) -> Vec<u8> {
        let mut buffer = vec![0u8; 42];
        let mut ethernet_packet = MutableEthernetPacket::new(&mut buffer).unwrap();

        ethernet_packet.set_destination(target_mac.into());
        ethernet_packet.set_source(sender_mac.into());
        ethernet_packet.set_ethertype(EtherTypes::Arp);

        let mut arp_buffer = vec![0u8; 28];
        let mut arp_packet = MutableArpPacket::new(&mut arp_buffer).unwrap();

        arp_packet.set_hardware_type(ArpHardwareTypes::Ethernet);
        arp_packet.set_protocol_type(EtherTypes::Ipv4);
        arp_packet.set_hw_addr_len(6);
        arp_packet.set_proto_addr_len(4);
        arp_packet.set_operation(ArpOperations::Reply);
        arp_packet.set_sender_hw_addr(sender_mac.into());
        arp_packet.set_sender_proto_addr(sender_ip);
        arp_packet.set_target_hw_addr(target_mac.into());
        arp_packet.set_target_proto_addr(target_ip);

        ethernet_packet.set_payload(arp_packet.packet());
        buffer
    }
}