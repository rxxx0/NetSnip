use anyhow::Result;
use pnet::datalink::{self, Channel, NetworkInterface};
use pnet::packet::arp::{ArpHardwareTypes, ArpOperations, MutableArpPacket};
use pnet::packet::ethernet::{EtherTypes, MutableEthernetPacket};
use pnet::packet::{MutablePacket, Packet};
use std::net::Ipv4Addr;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::time::interval;

pub struct ArpController {
    interface: NetworkInterface,
    our_mac: [u8; 6],
    gateway_ip: Ipv4Addr,
    gateway_mac: [u8; 6],
    active_spoofs: Arc<Mutex<Vec<ArpSpoof>>>,
}

#[derive(Clone, Debug)]
struct ArpSpoof {
    target_ip: Ipv4Addr,
    target_mac: [u8; 6],
    active: bool,
}

impl ArpController {
    pub fn new(
        interface: NetworkInterface,
        gateway_ip: Ipv4Addr,
        gateway_mac: [u8; 6],
    ) -> Result<Self> {
        let our_mac = interface.mac.map(|m| m.octets()).unwrap_or([0; 6]);

        Ok(Self {
            interface,
            our_mac,
            gateway_ip,
            gateway_mac,
            active_spoofs: Arc::new(Mutex::new(Vec::new())),
        })
    }

    pub async fn block_device(&self, target_ip: Ipv4Addr, target_mac: [u8; 6]) -> Result<()> {
        // Safety check: prevent self-blocking
        let our_ip = self.get_our_ip()?;
        if target_ip == our_ip {
            return Err(anyhow::anyhow!("Cannot block own device"));
        }

        // Add to active spoofs
        let mut spoofs = self.active_spoofs.lock().await;
        spoofs.push(ArpSpoof {
            target_ip,
            target_mac,
            active: true,
        });

        // Start spoofing in background task
        let gateway_ip = self.gateway_ip;
        let gateway_mac = self.gateway_mac;
        let our_mac = self.our_mac;
        let active_spoofs = Arc::clone(&self.active_spoofs);

        tokio::spawn(async move {
            let mut interval = interval(Duration::from_secs(1));
            loop {
                interval.tick().await;

                let spoofs = active_spoofs.lock().await;
                if !spoofs.iter().any(|s| s.target_ip == target_ip && s.active) {
                    break;
                }
                drop(spoofs);

                // Send poison packets
                // TODO: Implement actual packet sending
            }
        });

        Ok(())
    }

    pub async fn restore_device(&self, target_ip: Ipv4Addr) -> Result<()> {
        let mut spoofs = self.active_spoofs.lock().await;
        if let Some(spoof) = spoofs.iter_mut().find(|s| s.target_ip == target_ip) {
            spoof.active = false;

            // Send restoration packets
            // TODO: Implement restoration logic
        }

        Ok(())
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