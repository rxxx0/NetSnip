use anyhow::Result;
use ipnetwork::IpNetwork;
use pnet::datalink::{self, NetworkInterface};
use pnet::packet::arp::{ArpHardwareTypes, ArpOperations, ArpPacket, MutableArpPacket};
use pnet::packet::ethernet::{EtherTypes, EthernetPacket, MutableEthernetPacket};
use pnet::packet::{MutablePacket, Packet};
use std::net::{IpAddr, Ipv4Addr};
use std::time::Duration;
use tokio::time::sleep;

pub struct NetworkScanner {
    interface: NetworkInterface,
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

        Ok(Self { interface })
    }

    pub async fn scan_subnet(&self, subnet: &str) -> Result<Vec<String>> {
        let network: IpNetwork = subnet.parse()?;
        let mut devices = Vec::new();

        for ip in network.iter() {
            if let IpAddr::V4(ipv4) = ip {
                // TODO: Send ARP request and collect responses
                devices.push(ipv4.to_string());
            }
        }

        Ok(devices)
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