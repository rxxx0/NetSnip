use std::net::{IpAddr, Ipv4Addr};
use pnet::datalink;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SafetyCheck {
    pub is_safe: bool,
    pub reason: Option<String>,
}

#[allow(dead_code)]
pub fn check_self_cut(target_ip: &str) -> SafetyCheck {
    // Get current machine's IPs
    let interfaces = datalink::interfaces();

    for interface in interfaces {
        for ip in interface.ips {
            if ip.ip().to_string() == target_ip {
                return SafetyCheck {
                    is_safe: false,
                    reason: Some("Cannot cut own device - this would disconnect NetSnip".to_string()),
                };
            }
        }
    }

    SafetyCheck {
        is_safe: true,
        reason: None,
    }
}

#[allow(dead_code)]
pub fn check_gateway_cut(target_ip: &str, gateway_ip: &str) -> SafetyCheck {
    if target_ip == gateway_ip {
        SafetyCheck {
            is_safe: false,
            reason: Some("Warning: Cutting the gateway will disconnect ALL devices on the network".to_string()),
        }
    } else {
        SafetyCheck {
            is_safe: true,
            reason: None,
        }
    }
}

#[allow(dead_code)]
pub fn is_private_ip(ip: &str) -> bool {
    match ip.parse::<IpAddr>() {
        Ok(IpAddr::V4(ipv4)) => {
            // RFC 1918 private IP ranges
            // 10.0.0.0/8
            if ipv4.octets()[0] == 10 {
                return true;
            }
            // 172.16.0.0/12
            if ipv4.octets()[0] == 172 && (16..=31).contains(&ipv4.octets()[1]) {
                return true;
            }
            // 192.168.0.0/16
            if ipv4.octets()[0] == 192 && ipv4.octets()[1] == 168 {
                return true;
            }
            // 169.254.0.0/16 (link-local)
            if ipv4.octets()[0] == 169 && ipv4.octets()[1] == 254 {
                return true;
            }
            false
        }
        _ => false
    }
}

#[allow(dead_code)]
pub fn validate_mac_address(mac: &str) -> bool {
    // MAC address format: XX:XX:XX:XX:XX:XX
    let parts: Vec<&str> = mac.split(':').collect();

    if parts.len() != 6 {
        return false;
    }

    for part in parts {
        if part.len() != 2 {
            return false;
        }
        if !part.chars().all(|c| c.is_ascii_hexdigit()) {
            return false;
        }
    }

    // Check for invalid MACs
    if mac == "00:00:00:00:00:00" || mac == "FF:FF:FF:FF:FF:FF" {
        return false;
    }

    true
}

#[allow(dead_code)]
pub fn is_multicast_mac(mac: &str) -> bool {
    if let Some(first_octet) = mac.split(':').next() {
        if let Ok(byte) = u8::from_str_radix(first_octet, 16) {
            // Multicast bit is the least significant bit of the first octet
            return (byte & 0x01) == 0x01;
        }
    }
    false
}

#[allow(dead_code)]
pub fn get_network_from_ip(ip: &str, subnet_mask: &str) -> Option<String> {
    let ip_addr: Ipv4Addr = ip.parse().ok()?;
    let mask_addr: Ipv4Addr = subnet_mask.parse().ok()?;

    let ip_octets = ip_addr.octets();
    let mask_octets = mask_addr.octets();

    let network_octets: Vec<u8> = ip_octets
        .iter()
        .zip(mask_octets.iter())
        .map(|(ip, mask)| ip & mask)
        .collect();

    // Calculate prefix length (CIDR notation)
    let prefix_len = mask_octets
        .iter()
        .map(|octet| octet.count_ones())
        .sum::<u32>();

    Some(format!(
        "{}.{}.{}.{}/{}",
        network_octets[0],
        network_octets[1],
        network_octets[2],
        network_octets[3],
        prefix_len
    ))
}