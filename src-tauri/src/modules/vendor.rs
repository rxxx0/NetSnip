use std::collections::HashMap;

pub struct VendorLookup {
    vendors: HashMap<String, String>,
}

impl VendorLookup {
    pub fn new() -> Self {
        let mut vendors = HashMap::new();

        // Common MAC OUI prefixes and their vendors
        // These are the first 3 bytes of MAC addresses (OUI - Organizationally Unique Identifier)

        // Apple devices
        vendors.insert("00:03:93".to_string(), "Apple".to_string());
        vendors.insert("00:05:02".to_string(), "Apple".to_string());
        vendors.insert("00:0A:27".to_string(), "Apple".to_string());
        vendors.insert("00:0A:95".to_string(), "Apple".to_string());
        vendors.insert("00:0D:93".to_string(), "Apple".to_string());
        vendors.insert("00:10:FA".to_string(), "Apple".to_string());
        vendors.insert("00:11:24".to_string(), "Apple".to_string());
        vendors.insert("00:14:51".to_string(), "Apple".to_string());
        vendors.insert("00:16:CB".to_string(), "Apple".to_string());
        vendors.insert("00:17:F2".to_string(), "Apple".to_string());
        vendors.insert("00:19:E3".to_string(), "Apple".to_string());
        vendors.insert("00:1B:63".to_string(), "Apple".to_string());
        vendors.insert("00:1C:B3".to_string(), "Apple".to_string());
        vendors.insert("00:1D:4F".to_string(), "Apple".to_string());
        vendors.insert("00:1E:52".to_string(), "Apple".to_string());
        vendors.insert("00:1F:5B".to_string(), "Apple".to_string());
        vendors.insert("00:1F:F3".to_string(), "Apple".to_string());
        vendors.insert("00:21:E9".to_string(), "Apple".to_string());
        vendors.insert("00:22:41".to_string(), "Apple".to_string());
        vendors.insert("00:23:12".to_string(), "Apple".to_string());
        vendors.insert("00:23:32".to_string(), "Apple".to_string());
        vendors.insert("00:23:6C".to_string(), "Apple".to_string());
        vendors.insert("00:23:DF".to_string(), "Apple".to_string());
        vendors.insert("00:24:36".to_string(), "Apple".to_string());
        vendors.insert("00:25:00".to_string(), "Apple".to_string());
        vendors.insert("00:25:4B".to_string(), "Apple".to_string());
        vendors.insert("00:25:BC".to_string(), "Apple".to_string());
        vendors.insert("00:26:08".to_string(), "Apple".to_string());
        vendors.insert("00:26:4A".to_string(), "Apple".to_string());
        vendors.insert("00:26:B0".to_string(), "Apple".to_string());
        vendors.insert("00:26:BB".to_string(), "Apple".to_string());
        vendors.insert("00:30:65".to_string(), "Apple".to_string());
        vendors.insert("00:3E:E1".to_string(), "Apple".to_string());
        vendors.insert("00:50:E4".to_string(), "Apple".to_string());
        vendors.insert("00:61:71".to_string(), "Apple".to_string());
        vendors.insert("00:88:65".to_string(), "Apple".to_string());
        vendors.insert("00:A0:40".to_string(), "Apple".to_string());
        vendors.insert("00:C6:10".to_string(), "Apple".to_string());
        vendors.insert("00:CD:FE".to_string(), "Apple".to_string());
        vendors.insert("00:D8:3B".to_string(), "Apple".to_string());
        vendors.insert("00:DB:70".to_string(), "Apple".to_string());
        vendors.insert("00:F4:B9".to_string(), "Apple".to_string());
        vendors.insert("00:F7:6F".to_string(), "Apple".to_string());

        // Samsung devices
        vendors.insert("00:00:F0".to_string(), "Samsung".to_string());
        vendors.insert("00:02:78".to_string(), "Samsung".to_string());
        vendors.insert("00:07:AB".to_string(), "Samsung".to_string());
        vendors.insert("00:09:18".to_string(), "Samsung".to_string());
        vendors.insert("00:0D:AE".to_string(), "Samsung".to_string());
        vendors.insert("00:0D:E5".to_string(), "Samsung".to_string());
        vendors.insert("00:12:47".to_string(), "Samsung".to_string());
        vendors.insert("00:12:FB".to_string(), "Samsung".to_string());
        vendors.insert("00:13:77".to_string(), "Samsung".to_string());
        vendors.insert("00:15:99".to_string(), "Samsung".to_string());
        vendors.insert("00:15:B9".to_string(), "Samsung".to_string());
        vendors.insert("00:16:32".to_string(), "Samsung".to_string());
        vendors.insert("00:16:6B".to_string(), "Samsung".to_string());
        vendors.insert("00:16:6C".to_string(), "Samsung".to_string());
        vendors.insert("00:16:DB".to_string(), "Samsung".to_string());
        vendors.insert("00:17:C9".to_string(), "Samsung".to_string());
        vendors.insert("00:17:D5".to_string(), "Samsung".to_string());
        vendors.insert("00:18:26".to_string(), "Samsung".to_string());
        vendors.insert("00:18:AF".to_string(), "Samsung".to_string());
        vendors.insert("00:1A:8A".to_string(), "Samsung".to_string());
        vendors.insert("00:1B:98".to_string(), "Samsung".to_string());
        vendors.insert("00:1C:43".to_string(), "Samsung".to_string());
        vendors.insert("00:1D:25".to_string(), "Samsung".to_string());
        vendors.insert("00:1D:F6".to_string(), "Samsung".to_string());
        vendors.insert("00:1E:7D".to_string(), "Samsung".to_string());
        vendors.insert("00:1E:E1".to_string(), "Samsung".to_string());
        vendors.insert("00:1E:E2".to_string(), "Samsung".to_string());
        vendors.insert("00:1F:CC".to_string(), "Samsung".to_string());
        vendors.insert("00:1F:CD".to_string(), "Samsung".to_string());
        vendors.insert("00:21:19".to_string(), "Samsung".to_string());
        vendors.insert("00:21:4C".to_string(), "Samsung".to_string());
        vendors.insert("00:21:D1".to_string(), "Samsung".to_string());
        vendors.insert("00:21:D2".to_string(), "Samsung".to_string());

        // Microsoft devices
        vendors.insert("00:03:FF".to_string(), "Microsoft".to_string());
        vendors.insert("00:0D:3A".to_string(), "Microsoft".to_string());
        vendors.insert("00:12:5A".to_string(), "Microsoft".to_string());
        vendors.insert("00:15:5D".to_string(), "Microsoft".to_string());
        vendors.insert("00:17:FA".to_string(), "Microsoft".to_string());
        vendors.insert("00:1D:D8".to_string(), "Microsoft".to_string());
        vendors.insert("00:22:48".to_string(), "Microsoft".to_string());
        vendors.insert("00:25:AE".to_string(), "Microsoft".to_string());
        vendors.insert("00:50:F2".to_string(), "Microsoft".to_string());

        // Sony devices
        vendors.insert("00:00:95".to_string(), "Sony".to_string());
        vendors.insert("00:01:4A".to_string(), "Sony".to_string());
        vendors.insert("00:04:1F".to_string(), "Sony".to_string());
        vendors.insert("00:0A:D9".to_string(), "Sony".to_string());
        vendors.insert("00:0E:07".to_string(), "Sony".to_string());
        vendors.insert("00:12:EE".to_string(), "Sony".to_string());
        vendors.insert("00:13:15".to_string(), "Sony".to_string());
        vendors.insert("00:13:A9".to_string(), "Sony".to_string());
        vendors.insert("00:15:C1".to_string(), "Sony".to_string());
        vendors.insert("00:16:20".to_string(), "Sony".to_string());
        vendors.insert("00:19:63".to_string(), "Sony".to_string());
        vendors.insert("00:1A:80".to_string(), "Sony".to_string());
        vendors.insert("00:1B:59".to_string(), "Sony".to_string());
        vendors.insert("00:1D:0D".to_string(), "Sony".to_string());
        vendors.insert("00:1D:28".to_string(), "Sony".to_string());
        vendors.insert("00:1D:BA".to_string(), "Sony".to_string());
        vendors.insert("00:1E:45".to_string(), "Sony".to_string());
        vendors.insert("00:1E:DC".to_string(), "Sony".to_string());
        vendors.insert("00:1F:A7".to_string(), "Sony".to_string());
        vendors.insert("00:21:9E".to_string(), "Sony".to_string());
        vendors.insert("00:22:98".to_string(), "Sony".to_string());
        vendors.insert("00:23:45".to_string(), "Sony".to_string());
        vendors.insert("00:24:8D".to_string(), "Sony".to_string());
        vendors.insert("00:24:BE".to_string(), "Sony".to_string());
        vendors.insert("00:24:EF".to_string(), "Sony".to_string());
        vendors.insert("00:25:E7".to_string(), "Sony".to_string());
        vendors.insert("00:D9:D1".to_string(), "Sony".to_string());
        vendors.insert("00:E4:21".to_string(), "Sony".to_string());
        vendors.insert("00:EB:2D".to_string(), "Sony".to_string());

        // Google devices
        vendors.insert("00:1A:11".to_string(), "Google".to_string());
        vendors.insert("3C:5A:B4".to_string(), "Google".to_string());
        vendors.insert("54:60:09".to_string(), "Google".to_string());
        vendors.insert("74:E5:43".to_string(), "Google".to_string());
        vendors.insert("94:B4:0F".to_string(), "Google".to_string());
        vendors.insert("94:EB:2C".to_string(), "Google".to_string());
        vendors.insert("A4:77:33".to_string(), "Google".to_string());
        vendors.insert("F4:F5:D8".to_string(), "Google".to_string());

        // Amazon devices
        vendors.insert("00:BB:3A".to_string(), "Amazon".to_string());
        vendors.insert("00:FC:8B".to_string(), "Amazon".to_string());
        vendors.insert("0C:47:C9".to_string(), "Amazon".to_string());
        vendors.insert("18:74:2E".to_string(), "Amazon".to_string());
        vendors.insert("34:D2:70".to_string(), "Amazon".to_string());
        vendors.insert("38:F7:3D".to_string(), "Amazon".to_string());
        vendors.insert("40:B4:CD".to_string(), "Amazon".to_string());
        vendors.insert("44:65:0D".to_string(), "Amazon".to_string());
        vendors.insert("50:DC:E7".to_string(), "Amazon".to_string());
        vendors.insert("68:37:E9".to_string(), "Amazon".to_string());
        vendors.insert("68:54:FD".to_string(), "Amazon".to_string());
        vendors.insert("6C:56:97".to_string(), "Amazon".to_string());
        vendors.insert("74:75:48".to_string(), "Amazon".to_string());
        vendors.insert("74:C2:46".to_string(), "Amazon".to_string());
        vendors.insert("78:E1:03".to_string(), "Amazon".to_string());
        vendors.insert("84:D6:D0".to_string(), "Amazon".to_string());
        vendors.insert("88:71:E5".to_string(), "Amazon".to_string());
        vendors.insert("8C:E7:48".to_string(), "Amazon".to_string());
        vendors.insert("A0:02:DC".to_string(), "Amazon".to_string());
        vendors.insert("AC:63:BE".to_string(), "Amazon".to_string());
        vendors.insert("B0:47:BF".to_string(), "Amazon".to_string());
        vendors.insert("B4:7C:9C".to_string(), "Amazon".to_string());
        vendors.insert("CC:F7:35".to_string(), "Amazon".to_string());
        vendors.insert("F0:27:2D".to_string(), "Amazon".to_string());
        vendors.insert("F0:4F:7C".to_string(), "Amazon".to_string());
        vendors.insert("F0:81:73".to_string(), "Amazon".to_string());
        vendors.insert("FC:65:DE".to_string(), "Amazon".to_string());
        vendors.insert("FC:A1:83".to_string(), "Amazon".to_string());

        // Intel devices
        vendors.insert("00:02:B3".to_string(), "Intel".to_string());
        vendors.insert("00:03:47".to_string(), "Intel".to_string());
        vendors.insert("00:04:23".to_string(), "Intel".to_string());
        vendors.insert("00:07:E9".to_string(), "Intel".to_string());
        vendors.insert("00:0C:F1".to_string(), "Intel".to_string());
        vendors.insert("00:0E:0C".to_string(), "Intel".to_string());
        vendors.insert("00:0E:35".to_string(), "Intel".to_string());
        vendors.insert("00:11:11".to_string(), "Intel".to_string());
        vendors.insert("00:12:F0".to_string(), "Intel".to_string());
        vendors.insert("00:13:02".to_string(), "Intel".to_string());
        vendors.insert("00:13:20".to_string(), "Intel".to_string());
        vendors.insert("00:13:CE".to_string(), "Intel".to_string());
        vendors.insert("00:13:E8".to_string(), "Intel".to_string());
        vendors.insert("00:15:00".to_string(), "Intel".to_string());
        vendors.insert("00:15:17".to_string(), "Intel".to_string());
        vendors.insert("00:16:6F".to_string(), "Intel".to_string());
        vendors.insert("00:16:76".to_string(), "Intel".to_string());
        vendors.insert("00:16:EA".to_string(), "Intel".to_string());
        vendors.insert("00:16:EB".to_string(), "Intel".to_string());
        vendors.insert("00:18:DE".to_string(), "Intel".to_string());
        vendors.insert("00:19:D1".to_string(), "Intel".to_string());
        vendors.insert("00:19:D2".to_string(), "Intel".to_string());
        vendors.insert("00:1B:21".to_string(), "Intel".to_string());
        vendors.insert("00:1B:77".to_string(), "Intel".to_string());
        vendors.insert("00:1C:BF".to_string(), "Intel".to_string());
        vendors.insert("00:1C:C0".to_string(), "Intel".to_string());
        vendors.insert("00:1D:E0".to_string(), "Intel".to_string());
        vendors.insert("00:1D:E1".to_string(), "Intel".to_string());
        vendors.insert("00:1E:64".to_string(), "Intel".to_string());
        vendors.insert("00:1E:65".to_string(), "Intel".to_string());
        vendors.insert("00:1E:67".to_string(), "Intel".to_string());
        vendors.insert("00:1F:3A".to_string(), "Intel".to_string());
        vendors.insert("00:1F:3B".to_string(), "Intel".to_string());
        vendors.insert("00:20:7B".to_string(), "Intel".to_string());
        vendors.insert("00:21:5C".to_string(), "Intel".to_string());
        vendors.insert("00:21:5D".to_string(), "Intel".to_string());
        vendors.insert("00:21:6A".to_string(), "Intel".to_string());
        vendors.insert("00:21:6B".to_string(), "Intel".to_string());
        vendors.insert("00:22:FA".to_string(), "Intel".to_string());
        vendors.insert("00:22:FB".to_string(), "Intel".to_string());
        vendors.insert("00:23:14".to_string(), "Intel".to_string());
        vendors.insert("00:23:15".to_string(), "Intel".to_string());
        vendors.insert("00:24:D6".to_string(), "Intel".to_string());
        vendors.insert("00:24:D7".to_string(), "Intel".to_string());
        vendors.insert("00:26:C6".to_string(), "Intel".to_string());
        vendors.insert("00:26:C7".to_string(), "Intel".to_string());
        vendors.insert("00:27:10".to_string(), "Intel".to_string());
        vendors.insert("00:27:13".to_string(), "Intel".to_string());
        vendors.insert("00:28:F8".to_string(), "Intel".to_string());
        vendors.insert("00:2B:BC".to_string(), "Intel".to_string());
        vendors.insert("00:2E:3B".to_string(), "Intel".to_string());
        vendors.insert("00:50:8B".to_string(), "Intel".to_string());
        vendors.insert("00:8C:FA".to_string(), "Intel".to_string());
        vendors.insert("00:90:27".to_string(), "Intel".to_string());
        vendors.insert("00:A0:C9".to_string(), "Intel".to_string());
        vendors.insert("00:AA:00".to_string(), "Intel".to_string());
        vendors.insert("00:AA:01".to_string(), "Intel".to_string());
        vendors.insert("00:AA:02".to_string(), "Intel".to_string());
        vendors.insert("00:C2:C6".to_string(), "Intel".to_string());
        vendors.insert("00:D0:B7".to_string(), "Intel".to_string());
        vendors.insert("00:DB:DF".to_string(), "Intel".to_string());
        vendors.insert("00:DD:00".to_string(), "Intel".to_string());
        vendors.insert("00:DD:01".to_string(), "Intel".to_string());
        vendors.insert("00:E0:81".to_string(), "Intel".to_string());

        // TP-Link
        vendors.insert("00:14:78".to_string(), "TP-Link".to_string());
        vendors.insert("00:19:E0".to_string(), "TP-Link".to_string());
        vendors.insert("00:1D:0F".to_string(), "TP-Link".to_string());
        vendors.insert("00:21:27".to_string(), "TP-Link".to_string());
        vendors.insert("00:23:CD".to_string(), "TP-Link".to_string());
        vendors.insert("00:25:86".to_string(), "TP-Link".to_string());
        vendors.insert("00:27:19".to_string(), "TP-Link".to_string());
        vendors.insert("10:FE:ED".to_string(), "TP-Link".to_string());
        vendors.insert("14:CC:20".to_string(), "TP-Link".to_string());
        vendors.insert("14:CF:92".to_string(), "TP-Link".to_string());
        vendors.insert("18:A6:F7".to_string(), "TP-Link".to_string());
        vendors.insert("1C:3B:F3".to_string(), "TP-Link".to_string());
        vendors.insert("1C:FA:68".to_string(), "TP-Link".to_string());
        vendors.insert("20:DC:E6".to_string(), "TP-Link".to_string());
        vendors.insert("24:69:68".to_string(), "TP-Link".to_string());
        vendors.insert("28:EE:52".to_string(), "TP-Link".to_string());
        vendors.insert("30:B5:C2".to_string(), "TP-Link".to_string());
        vendors.insert("30:DE:4B".to_string(), "TP-Link".to_string());
        vendors.insert("30:FC:68".to_string(), "TP-Link".to_string());
        vendors.insert("34:60:F9".to_string(), "TP-Link".to_string());
        vendors.insert("34:E8:94".to_string(), "TP-Link".to_string());
        vendors.insert("38:83:45".to_string(), "TP-Link".to_string());
        vendors.insert("40:F5:20".to_string(), "TP-Link".to_string());
        vendors.insert("48:7D:2E".to_string(), "TP-Link".to_string());
        vendors.insert("48:8F:5A".to_string(), "TP-Link".to_string());
        vendors.insert("4C:E6:76".to_string(), "TP-Link".to_string());
        vendors.insert("50:3E:AA".to_string(), "TP-Link".to_string());
        vendors.insert("50:64:2B".to_string(), "TP-Link".to_string());
        vendors.insert("50:C7:BF".to_string(), "TP-Link".to_string());
        vendors.insert("50:FA:84".to_string(), "TP-Link".to_string());
        vendors.insert("54:A7:03".to_string(), "TP-Link".to_string());
        vendors.insert("54:AF:97".to_string(), "TP-Link".to_string());
        vendors.insert("54:C8:0F".to_string(), "TP-Link".to_string());
        vendors.insert("54:E4:3A".to_string(), "TP-Link".to_string());
        vendors.insert("5C:63:BF".to_string(), "TP-Link".to_string());
        vendors.insert("5C:A6:E6".to_string(), "TP-Link".to_string());
        vendors.insert("5C:E9:31".to_string(), "TP-Link".to_string());
        vendors.insert("60:32:B1".to_string(), "TP-Link".to_string());
        vendors.insert("60:3A:7C".to_string(), "TP-Link".to_string());
        vendors.insert("60:A4:B7".to_string(), "TP-Link".to_string());
        vendors.insert("60:E3:27".to_string(), "TP-Link".to_string());
        vendors.insert("64:66:B3".to_string(), "TP-Link".to_string());
        vendors.insert("64:6E:97".to_string(), "TP-Link".to_string());
        vendors.insert("64:70:02".to_string(), "TP-Link".to_string());

        // Netgear
        vendors.insert("00:09:5B".to_string(), "Netgear".to_string());
        vendors.insert("00:0F:B5".to_string(), "Netgear".to_string());
        vendors.insert("00:14:6C".to_string(), "Netgear".to_string());
        vendors.insert("00:18:4D".to_string(), "Netgear".to_string());
        vendors.insert("00:1B:2F".to_string(), "Netgear".to_string());
        vendors.insert("00:1E:2A".to_string(), "Netgear".to_string());
        vendors.insert("00:1F:33".to_string(), "Netgear".to_string());
        vendors.insert("00:22:3F".to_string(), "Netgear".to_string());
        vendors.insert("00:24:B2".to_string(), "Netgear".to_string());
        vendors.insert("00:26:F2".to_string(), "Netgear".to_string());
        vendors.insert("00:8E:F2".to_string(), "Netgear".to_string());
        vendors.insert("04:A1:51".to_string(), "Netgear".to_string());
        vendors.insert("08:02:8E".to_string(), "Netgear".to_string());
        vendors.insert("08:BD:43".to_string(), "Netgear".to_string());
        vendors.insert("0C:F4:D5".to_string(), "Netgear".to_string());
        vendors.insert("10:0C:6B".to_string(), "Netgear".to_string());
        vendors.insert("10:0D:7F".to_string(), "Netgear".to_string());
        vendors.insert("10:DA:43".to_string(), "Netgear".to_string());
        vendors.insert("14:59:C0".to_string(), "Netgear".to_string());
        vendors.insert("20:0C:C8".to_string(), "Netgear".to_string());
        vendors.insert("20:4E:7F".to_string(), "Netgear".to_string());
        vendors.insert("20:9B:CD".to_string(), "Netgear".to_string());
        vendors.insert("20:E5:2A".to_string(), "Netgear".to_string());
        vendors.insert("28:C6:8E".to_string(), "Netgear".to_string());
        vendors.insert("2C:30:33".to_string(), "Netgear".to_string());
        vendors.insert("2C:B0:5D".to_string(), "Netgear".to_string());
        vendors.insert("30:46:9A".to_string(), "Netgear".to_string());
        vendors.insert("34:98:B5".to_string(), "Netgear".to_string());
        vendors.insert("38:94:ED".to_string(), "Netgear".to_string());
        vendors.insert("3C:37:86".to_string(), "Netgear".to_string());
        vendors.insert("40:5D:82".to_string(), "Netgear".to_string());
        vendors.insert("44:94:FC".to_string(), "Netgear".to_string());
        vendors.insert("48:5D:36".to_string(), "Netgear".to_string());
        vendors.insert("50:04:B8".to_string(), "Netgear".to_string());
        vendors.insert("50:6A:03".to_string(), "Netgear".to_string());
        vendors.insert("54:83:3A".to_string(), "Netgear".to_string());
        vendors.insert("58:EF:68".to_string(), "Netgear".to_string());
        vendors.insert("5C:DC:96".to_string(), "Netgear".to_string());
        vendors.insert("6C:19:8F".to_string(), "Netgear".to_string());
        vendors.insert("6C:B0:CE".to_string(), "Netgear".to_string());
        vendors.insert("6C:CD:D6".to_string(), "Netgear".to_string());
        vendors.insert("70:85:40".to_string(), "Netgear".to_string());
        vendors.insert("74:44:01".to_string(), "Netgear".to_string());
        vendors.insert("78:D2:94".to_string(), "Netgear".to_string());
        vendors.insert("80:37:73".to_string(), "Netgear".to_string());
        vendors.insert("84:1B:5E".to_string(), "Netgear".to_string());
        vendors.insert("88:DC:96".to_string(), "Netgear".to_string());
        vendors.insert("8C:3B:AD".to_string(), "Netgear".to_string());
        vendors.insert("8C:FE:B4".to_string(), "Netgear".to_string());
        vendors.insert("94:10:3E".to_string(), "Netgear".to_string());
        vendors.insert("9C:3D:CF".to_string(), "Netgear".to_string());
        vendors.insert("9C:D3:6D".to_string(), "Netgear".to_string());
        vendors.insert("A0:04:60".to_string(), "Netgear".to_string());
        vendors.insert("A0:21:B7".to_string(), "Netgear".to_string());
        vendors.insert("A0:40:A0".to_string(), "Netgear".to_string());
        vendors.insert("A0:63:91".to_string(), "Netgear".to_string());
        vendors.insert("A4:2B:8C".to_string(), "Netgear".to_string());
        vendors.insert("A4:56:02".to_string(), "Netgear".to_string());
        vendors.insert("B0:39:56".to_string(), "Netgear".to_string());
        vendors.insert("B0:7F:B9".to_string(), "Netgear".to_string());
        vendors.insert("B0:B9:8A".to_string(), "Netgear".to_string());
        vendors.insert("C0:3F:0E".to_string(), "Netgear".to_string());
        vendors.insert("C0:FF:D4".to_string(), "Netgear".to_string());
        vendors.insert("C4:04:15".to_string(), "Netgear".to_string());
        vendors.insert("C4:3D:C7".to_string(), "Netgear".to_string());
        vendors.insert("C8:9E:43".to_string(), "Netgear".to_string());
        vendors.insert("CC:40:D0".to_string(), "Netgear".to_string());
        vendors.insert("D8:EE:78".to_string(), "Netgear".to_string());
        vendors.insert("DC:3A:5E".to_string(), "Netgear".to_string());
        vendors.insert("DC:EF:09".to_string(), "Netgear".to_string());
        vendors.insert("E0:46:9A".to_string(), "Netgear".to_string());
        vendors.insert("E0:91:F5".to_string(), "Netgear".to_string());
        vendors.insert("E4:F4:C6".to_string(), "Netgear".to_string());
        vendors.insert("E8:FC:AF".to_string(), "Netgear".to_string());
        vendors.insert("F8:73:94".to_string(), "Netgear".to_string());
        vendors.insert("FC:7A:58".to_string(), "Netgear".to_string());

        Self { vendors }
    }

    pub fn lookup(&self, mac: &str) -> Option<String> {
        // Normalize the MAC address
        let mac_upper = mac.to_uppercase();
        let prefix = if mac_upper.len() >= 8 {
            &mac_upper[0..8]
        } else {
            return None;
        };

        self.vendors.get(prefix).cloned()
    }

    pub fn get_device_type(&self, mac: &str, hostname: Option<&String>) -> String {
        // Try to determine device type from vendor
        if let Some(vendor) = self.lookup(mac) {
            match vendor.as_str() {
                "Apple" => {
                    // Try to determine specific Apple device type from hostname
                    if let Some(name) = hostname {
                        let lower = name.to_lowercase();
                        if lower.contains("iphone") {
                            return "phone".to_string();
                        } else if lower.contains("ipad") {
                            return "tablet".to_string();
                        } else if lower.contains("macbook") || lower.contains("mac") {
                            return "computer".to_string();
                        } else if lower.contains("appletv") || lower.contains("apple-tv") {
                            return "tv".to_string();
                        } else if lower.contains("watch") {
                            return "iot".to_string();
                        } else if lower.contains("airport") || lower.contains("time-capsule") {
                            return "router".to_string();
                        }
                    }
                    return "computer".to_string(); // Default for Apple
                }
                "Samsung" | "Sony" => {
                    if let Some(name) = hostname {
                        let lower = name.to_lowercase();
                        if lower.contains("tv") || lower.contains("smart") {
                            return "tv".to_string();
                        } else if lower.contains("galaxy") || lower.contains("phone") {
                            return "phone".to_string();
                        } else if lower.contains("tab") {
                            return "tablet".to_string();
                        }
                    }
                    return "phone".to_string(); // Common for Samsung/Sony
                }
                "Microsoft" | "Intel" => return "computer".to_string(),
                "Google" => {
                    if let Some(name) = hostname {
                        let lower = name.to_lowercase();
                        if lower.contains("chromecast") {
                            return "tv".to_string();
                        } else if lower.contains("home") || lower.contains("nest") {
                            return "iot".to_string();
                        } else if lower.contains("pixel") {
                            return "phone".to_string();
                        }
                    }
                    return "iot".to_string();
                }
                "Amazon" => {
                    if let Some(name) = hostname {
                        let lower = name.to_lowercase();
                        if lower.contains("echo") || lower.contains("alexa") {
                            return "iot".to_string();
                        } else if lower.contains("fire") {
                            if lower.contains("tv") || lower.contains("stick") {
                                return "tv".to_string();
                            }
                            return "tablet".to_string();
                        }
                    }
                    return "iot".to_string();
                }
                "TP-Link" | "Netgear" => return "router".to_string(),
                _ => {}
            }
        }

        // Try to guess from hostname patterns
        if let Some(name) = hostname {
            let lower = name.to_lowercase();
            if lower.contains("router") || lower.contains("gateway") || lower.contains("ap") {
                return "router".to_string();
            } else if lower.contains("phone") || lower.contains("mobile") {
                return "phone".to_string();
            } else if lower.contains("tablet") || lower.contains("ipad") || lower.contains("tab") {
                return "tablet".to_string();
            } else if lower.contains("tv") || lower.contains("roku") || lower.contains("chromecast") {
                return "tv".to_string();
            } else if lower.contains("playstation") || lower.contains("xbox") || lower.contains("switch") {
                return "gaming".to_string();
            } else if lower.contains("printer") || lower.contains("cam") || lower.contains("hub") {
                return "iot".to_string();
            } else if lower.contains("pc") || lower.contains("desktop") || lower.contains("laptop") || lower.contains("macbook") {
                return "computer".to_string();
            }
        }

        "unknown".to_string()
    }
}