#[allow(dead_code)]
pub fn fingerprint_device(device: &crate::commands::network::Device) -> String {
    // Device type detection based on various heuristics

    // Check if it's a gateway/router
    if device.is_gateway {
        return "router".to_string();
    }

    // Check MAC address manufacturer
    let device_type = match device.manufacturer.as_deref() {
        Some(manufacturer) => {
            let lower_manufacturer = manufacturer.to_lowercase();

            if lower_manufacturer.contains("apple") {
                // Further identify Apple devices by hostname patterns
                if let Some(ref name) = device.custom_name {
                    let lower_name = name.to_lowercase();
                    if lower_name.contains("iphone") || lower_name.contains("ios") {
                        "phone"
                    } else if lower_name.contains("ipad") {
                        "tablet"
                    } else if lower_name.contains("macbook") || lower_name.contains("imac") || lower_name.contains("mac") {
                        "computer"
                    } else if lower_name.contains("tv") || lower_name.contains("appletv") {
                        "iot"
                    } else if lower_name.contains("watch") {
                        "iot"
                    } else {
                        // Default Apple device
                        "computer"
                    }
                } else {
                    "computer"
                }
            } else if lower_manufacturer.contains("samsung") ||
                      lower_manufacturer.contains("xiaomi") ||
                      lower_manufacturer.contains("huawei") ||
                      lower_manufacturer.contains("oneplus") ||
                      lower_manufacturer.contains("oppo") ||
                      lower_manufacturer.contains("vivo") ||
                      lower_manufacturer.contains("realme") ||
                      lower_manufacturer.contains("nokia") ||
                      lower_manufacturer.contains("motorola") ||
                      lower_manufacturer.contains("lg") ||
                      lower_manufacturer.contains("sony") ||
                      lower_manufacturer.contains("blackberry") {
                "phone"
            } else if lower_manufacturer.contains("microsoft") ||
                      lower_manufacturer.contains("dell") ||
                      lower_manufacturer.contains("hp") ||
                      lower_manufacturer.contains("lenovo") ||
                      lower_manufacturer.contains("asus") ||
                      lower_manufacturer.contains("acer") ||
                      lower_manufacturer.contains("intel") {
                "computer"
            } else if lower_manufacturer.contains("netgear") ||
                      lower_manufacturer.contains("tp-link") ||
                      lower_manufacturer.contains("d-link") ||
                      lower_manufacturer.contains("cisco") ||
                      lower_manufacturer.contains("linksys") ||
                      lower_manufacturer.contains("ubiquiti") ||
                      lower_manufacturer.contains("asus") {
                "router"
            } else if lower_manufacturer.contains("roku") ||
                      lower_manufacturer.contains("amazon") ||
                      lower_manufacturer.contains("google") ||
                      lower_manufacturer.contains("nest") ||
                      lower_manufacturer.contains("ring") ||
                      lower_manufacturer.contains("philips") ||
                      lower_manufacturer.contains("sonos") {
                "iot"
            } else if lower_manufacturer.contains("vmware") ||
                      lower_manufacturer.contains("virtualbox") ||
                      lower_manufacturer.contains("parallels") ||
                      lower_manufacturer.contains("xen") {
                "computer"
            } else {
                "unknown"
            }
        },
        None => "unknown"
    };

    // Additional heuristics based on hostname patterns
    if device_type == "unknown" {
        if !device.name.is_empty() {
            let lower_name = device.name.to_lowercase();

            if lower_name.contains("android") || lower_name.contains("galaxy") || lower_name.contains("pixel") {
                return "phone".to_string();
            } else if lower_name.contains("iphone") || lower_name.contains("ipad") {
                return if lower_name.contains("ipad") { "tablet" } else { "phone" }.to_string();
            } else if lower_name.contains("macbook") || lower_name.contains("laptop") || lower_name.contains("desktop") {
                return "computer".to_string();
            } else if lower_name.contains("printer") || lower_name.contains("print") {
                return "iot".to_string();
            } else if lower_name.contains("tv") || lower_name.contains("roku") || lower_name.contains("chromecast") {
                return "iot".to_string();
            } else if lower_name.contains("echo") || lower_name.contains("alexa") || lower_name.contains("google-home") {
                return "iot".to_string();
            } else if lower_name.contains("playstation") || lower_name.contains("xbox") || lower_name.contains("nintendo") {
                return "iot".to_string();
            } else if lower_name.contains("camera") || lower_name.contains("cam") {
                return "iot".to_string();
            }
        }
    }

    device_type.to_string()
}

#[allow(dead_code)]
pub fn resolve_hostname(ip: &str) -> Option<String> {
    // Use DNS lookup to resolve hostname
    match dns_lookup::lookup_addr(&ip.parse().ok()?) {
        Ok(hostname) => Some(hostname),
        Err(_) => None
    }
}