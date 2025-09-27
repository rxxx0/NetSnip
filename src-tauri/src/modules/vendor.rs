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

        // Cisco/Linksys devices
        vendors.insert("00:00:0C".to_string(), "Cisco".to_string());
        vendors.insert("00:01:42".to_string(), "Cisco".to_string());
        vendors.insert("00:01:63".to_string(), "Cisco".to_string());
        vendors.insert("00:01:64".to_string(), "Cisco".to_string());
        vendors.insert("00:01:96".to_string(), "Cisco".to_string());
        vendors.insert("00:01:97".to_string(), "Cisco".to_string());
        vendors.insert("00:01:C7".to_string(), "Cisco".to_string());
        vendors.insert("00:01:C9".to_string(), "Cisco".to_string());
        vendors.insert("00:02:16".to_string(), "Cisco".to_string());
        vendors.insert("00:02:17".to_string(), "Cisco".to_string());
        vendors.insert("00:02:3D".to_string(), "Cisco".to_string());
        vendors.insert("00:02:4A".to_string(), "Cisco".to_string());
        vendors.insert("00:02:4B".to_string(), "Cisco".to_string());
        vendors.insert("00:02:7D".to_string(), "Cisco".to_string());
        vendors.insert("00:02:7E".to_string(), "Cisco".to_string());
        vendors.insert("00:02:B9".to_string(), "Cisco".to_string());
        vendors.insert("00:02:BA".to_string(), "Cisco".to_string());
        vendors.insert("00:02:FC".to_string(), "Cisco".to_string());
        vendors.insert("00:02:FD".to_string(), "Cisco".to_string());
        vendors.insert("00:03:32".to_string(), "Cisco".to_string());
        vendors.insert("00:03:6B".to_string(), "Cisco".to_string());
        vendors.insert("00:03:6C".to_string(), "Cisco".to_string());
        vendors.insert("00:03:9F".to_string(), "Cisco".to_string());
        vendors.insert("00:03:A0".to_string(), "Cisco".to_string());
        vendors.insert("00:03:E3".to_string(), "Cisco".to_string());
        vendors.insert("00:03:E4".to_string(), "Cisco".to_string());
        vendors.insert("00:03:FD".to_string(), "Cisco".to_string());
        vendors.insert("00:03:FE".to_string(), "Cisco".to_string());
        vendors.insert("00:04:27".to_string(), "Cisco".to_string());
        vendors.insert("00:04:28".to_string(), "Cisco".to_string());
        vendors.insert("00:04:4D".to_string(), "Cisco".to_string());
        vendors.insert("00:04:4E".to_string(), "Cisco".to_string());
        vendors.insert("00:04:6D".to_string(), "Cisco".to_string());
        vendors.insert("00:04:6E".to_string(), "Cisco".to_string());
        vendors.insert("00:04:9A".to_string(), "Cisco".to_string());
        vendors.insert("00:04:9B".to_string(), "Cisco".to_string());
        vendors.insert("00:04:C0".to_string(), "Cisco".to_string());
        vendors.insert("00:04:C1".to_string(), "Cisco".to_string());
        vendors.insert("00:04:DD".to_string(), "Cisco".to_string());
        vendors.insert("00:04:DE".to_string(), "Cisco".to_string());
        vendors.insert("00:05:00".to_string(), "Cisco".to_string());
        vendors.insert("00:05:01".to_string(), "Cisco".to_string());
        vendors.insert("00:05:31".to_string(), "Cisco".to_string());
        vendors.insert("00:05:32".to_string(), "Cisco".to_string());
        vendors.insert("00:05:5E".to_string(), "Cisco".to_string());
        vendors.insert("00:05:5F".to_string(), "Cisco".to_string());
        vendors.insert("00:05:73".to_string(), "Cisco".to_string());
        vendors.insert("00:05:74".to_string(), "Cisco".to_string());
        vendors.insert("00:05:9B".to_string(), "Cisco".to_string());
        vendors.insert("00:05:DC".to_string(), "Cisco".to_string());
        vendors.insert("00:05:DD".to_string(), "Cisco".to_string());
        vendors.insert("00:06:28".to_string(), "Cisco".to_string());
        vendors.insert("00:06:2A".to_string(), "Cisco".to_string());
        vendors.insert("00:06:52".to_string(), "Cisco".to_string());
        vendors.insert("00:06:53".to_string(), "Cisco".to_string());
        vendors.insert("00:06:7C".to_string(), "Cisco".to_string());
        vendors.insert("00:06:C1".to_string(), "Cisco".to_string());
        vendors.insert("00:06:D6".to_string(), "Cisco".to_string());
        vendors.insert("00:06:D7".to_string(), "Cisco".to_string());
        vendors.insert("00:06:F6".to_string(), "Cisco".to_string());
        vendors.insert("00:07:0D".to_string(), "Cisco".to_string());
        vendors.insert("00:07:0E".to_string(), "Cisco".to_string());
        vendors.insert("00:07:4F".to_string(), "Cisco".to_string());
        vendors.insert("00:07:50".to_string(), "Cisco".to_string());
        vendors.insert("00:07:7D".to_string(), "Cisco".to_string());
        vendors.insert("00:07:84".to_string(), "Cisco".to_string());
        vendors.insert("00:07:85".to_string(), "Cisco".to_string());
        vendors.insert("00:07:B3".to_string(), "Cisco".to_string());
        vendors.insert("00:07:B4".to_string(), "Cisco".to_string());
        vendors.insert("00:07:EB".to_string(), "Cisco".to_string());
        vendors.insert("00:07:EC".to_string(), "Cisco".to_string());
        vendors.insert("00:08:20".to_string(), "Cisco".to_string());
        vendors.insert("00:08:21".to_string(), "Cisco".to_string());
        vendors.insert("00:08:2F".to_string(), "Cisco".to_string());
        vendors.insert("00:08:30".to_string(), "Cisco".to_string());
        vendors.insert("00:08:7C".to_string(), "Cisco".to_string());
        vendors.insert("00:08:7D".to_string(), "Cisco".to_string());
        vendors.insert("00:08:A1".to_string(), "Cisco".to_string());
        vendors.insert("00:08:A3".to_string(), "Cisco".to_string());
        vendors.insert("00:08:A4".to_string(), "Cisco".to_string());
        vendors.insert("00:08:C2".to_string(), "Cisco".to_string());
        vendors.insert("00:08:E2".to_string(), "Cisco".to_string());
        vendors.insert("00:08:E3".to_string(), "Cisco".to_string());
        vendors.insert("00:09:11".to_string(), "Cisco".to_string());
        vendors.insert("00:09:12".to_string(), "Cisco".to_string());
        vendors.insert("00:09:43".to_string(), "Cisco".to_string());
        vendors.insert("00:09:44".to_string(), "Cisco".to_string());
        vendors.insert("00:09:7B".to_string(), "Cisco".to_string());
        vendors.insert("00:09:7C".to_string(), "Cisco".to_string());
        vendors.insert("00:09:B6".to_string(), "Cisco".to_string());
        vendors.insert("00:09:B7".to_string(), "Cisco".to_string());
        vendors.insert("00:09:E8".to_string(), "Cisco".to_string());
        vendors.insert("00:09:E9".to_string(), "Cisco".to_string());
        vendors.insert("00:0A:41".to_string(), "Cisco".to_string());
        vendors.insert("00:0A:42".to_string(), "Cisco".to_string());
        vendors.insert("00:0A:8A".to_string(), "Cisco".to_string());
        vendors.insert("00:0A:8B".to_string(), "Cisco".to_string());
        vendors.insert("00:0A:B7".to_string(), "Cisco".to_string());
        vendors.insert("00:0A:B8".to_string(), "Cisco".to_string());
        vendors.insert("00:0A:F3".to_string(), "Cisco".to_string());
        vendors.insert("00:0A:F4".to_string(), "Cisco".to_string());
        vendors.insert("00:0B:45".to_string(), "Cisco".to_string());
        vendors.insert("00:0B:46".to_string(), "Cisco".to_string());
        vendors.insert("00:0B:5F".to_string(), "Cisco".to_string());
        vendors.insert("00:0B:60".to_string(), "Cisco".to_string());
        vendors.insert("00:0B:85".to_string(), "Cisco".to_string());
        vendors.insert("00:0B:BE".to_string(), "Cisco".to_string());
        vendors.insert("00:0B:BF".to_string(), "Cisco".to_string());
        vendors.insert("00:0B:FC".to_string(), "Cisco".to_string());
        vendors.insert("00:0B:FD".to_string(), "Cisco".to_string());
        vendors.insert("00:0C:30".to_string(), "Cisco".to_string());
        vendors.insert("00:0C:31".to_string(), "Cisco".to_string());
        vendors.insert("00:0C:41".to_string(), "Cisco".to_string());
        vendors.insert("00:0C:85".to_string(), "Cisco".to_string());
        vendors.insert("00:0C:86".to_string(), "Cisco".to_string());
        vendors.insert("00:0C:CE".to_string(), "Cisco".to_string());
        vendors.insert("00:0C:CF".to_string(), "Cisco".to_string());
        vendors.insert("00:0D:28".to_string(), "Cisco".to_string());
        vendors.insert("00:0D:29".to_string(), "Cisco".to_string());
        vendors.insert("00:0D:65".to_string(), "Cisco".to_string());
        vendors.insert("00:0D:66".to_string(), "Cisco".to_string());
        vendors.insert("00:0D:BC".to_string(), "Cisco".to_string());
        vendors.insert("00:0D:BD".to_string(), "Cisco".to_string());
        vendors.insert("00:0D:EC".to_string(), "Cisco".to_string());
        vendors.insert("00:0D:ED".to_string(), "Cisco".to_string());
        vendors.insert("00:0E:08".to_string(), "Cisco".to_string());
        vendors.insert("00:0E:38".to_string(), "Cisco".to_string());
        vendors.insert("00:0E:39".to_string(), "Cisco".to_string());
        vendors.insert("00:0E:83".to_string(), "Cisco".to_string());
        vendors.insert("00:0E:84".to_string(), "Cisco".to_string());
        vendors.insert("00:0E:D6".to_string(), "Cisco".to_string());
        vendors.insert("00:0E:D7".to_string(), "Cisco".to_string());
        vendors.insert("00:0F:23".to_string(), "Cisco".to_string());
        vendors.insert("00:0F:24".to_string(), "Cisco".to_string());
        vendors.insert("00:0F:34".to_string(), "Cisco".to_string());
        vendors.insert("00:0F:35".to_string(), "Cisco".to_string());
        vendors.insert("00:0F:66".to_string(), "Cisco/Linksys".to_string());
        vendors.insert("00:0F:8F".to_string(), "Cisco".to_string());
        vendors.insert("00:0F:90".to_string(), "Cisco".to_string());
        vendors.insert("00:0F:F7".to_string(), "Cisco".to_string());
        vendors.insert("00:0F:F8".to_string(), "Cisco".to_string());
        vendors.insert("00:10:07".to_string(), "Cisco".to_string());
        vendors.insert("00:10:0B".to_string(), "Cisco".to_string());
        vendors.insert("00:10:0D".to_string(), "Cisco".to_string());
        vendors.insert("00:10:11".to_string(), "Cisco".to_string());
        vendors.insert("00:10:14".to_string(), "Cisco".to_string());
        vendors.insert("00:10:1F".to_string(), "Cisco".to_string());
        vendors.insert("00:10:29".to_string(), "Cisco".to_string());
        vendors.insert("00:10:2F".to_string(), "Cisco".to_string());
        vendors.insert("00:10:54".to_string(), "Cisco".to_string());
        vendors.insert("00:10:79".to_string(), "Cisco".to_string());
        vendors.insert("00:10:7B".to_string(), "Cisco".to_string());
        vendors.insert("00:10:A6".to_string(), "Cisco".to_string());
        vendors.insert("00:10:F6".to_string(), "Cisco".to_string());
        vendors.insert("00:10:FF".to_string(), "Cisco".to_string());
        vendors.insert("00:11:20".to_string(), "Cisco".to_string());
        vendors.insert("00:11:21".to_string(), "Cisco".to_string());
        vendors.insert("00:11:5C".to_string(), "Cisco".to_string());
        vendors.insert("00:11:5D".to_string(), "Cisco".to_string());
        vendors.insert("00:11:92".to_string(), "Cisco".to_string());
        vendors.insert("00:11:93".to_string(), "Cisco".to_string());
        vendors.insert("00:11:BB".to_string(), "Cisco".to_string());
        vendors.insert("00:11:BC".to_string(), "Cisco".to_string());
        vendors.insert("00:12:00".to_string(), "Cisco".to_string());
        vendors.insert("00:12:01".to_string(), "Cisco".to_string());
        vendors.insert("00:12:17".to_string(), "Linksys/Cisco".to_string());
        vendors.insert("00:12:43".to_string(), "Cisco".to_string());
        vendors.insert("00:12:44".to_string(), "Cisco".to_string());
        vendors.insert("00:12:7F".to_string(), "Cisco".to_string());
        vendors.insert("00:12:80".to_string(), "Cisco".to_string());
        vendors.insert("00:12:D9".to_string(), "Cisco".to_string());
        vendors.insert("00:12:DA".to_string(), "Cisco".to_string());
        vendors.insert("00:13:10".to_string(), "Linksys/Cisco".to_string());
        vendors.insert("00:13:19".to_string(), "Cisco".to_string());
        vendors.insert("00:13:1A".to_string(), "Cisco".to_string());
        vendors.insert("00:13:5F".to_string(), "Cisco".to_string());
        vendors.insert("00:13:60".to_string(), "Cisco".to_string());
        vendors.insert("00:13:7F".to_string(), "Cisco".to_string());
        vendors.insert("00:13:80".to_string(), "Cisco".to_string());
        vendors.insert("00:13:C3".to_string(), "Cisco".to_string());
        vendors.insert("00:13:C4".to_string(), "Cisco".to_string());
        vendors.insert("00:14:0A".to_string(), "Cisco".to_string());
        vendors.insert("00:14:1B".to_string(), "Cisco".to_string());
        vendors.insert("00:14:1C".to_string(), "Cisco".to_string());
        vendors.insert("00:14:69".to_string(), "Cisco".to_string());
        vendors.insert("00:14:6A".to_string(), "Cisco".to_string());
        vendors.insert("00:14:A8".to_string(), "Cisco".to_string());
        vendors.insert("00:14:A9".to_string(), "Cisco".to_string());
        vendors.insert("00:14:BF".to_string(), "Linksys/Cisco".to_string());
        vendors.insert("00:14:F1".to_string(), "Cisco".to_string());
        vendors.insert("00:14:F2".to_string(), "Cisco".to_string());
        vendors.insert("00:15:2B".to_string(), "Cisco".to_string());
        vendors.insert("00:15:2C".to_string(), "Cisco".to_string());
        vendors.insert("00:15:62".to_string(), "Cisco".to_string());
        vendors.insert("00:15:63".to_string(), "Cisco".to_string());
        vendors.insert("00:15:C6".to_string(), "Cisco".to_string());
        vendors.insert("00:15:C7".to_string(), "Cisco".to_string());
        vendors.insert("00:15:F9".to_string(), "Cisco".to_string());
        vendors.insert("00:15:FA".to_string(), "Cisco".to_string());
        vendors.insert("00:16:46".to_string(), "Cisco".to_string());
        vendors.insert("00:16:47".to_string(), "Cisco".to_string());
        vendors.insert("00:16:9C".to_string(), "Cisco".to_string());
        vendors.insert("00:16:9D".to_string(), "Cisco".to_string());
        vendors.insert("00:16:B6".to_string(), "Linksys/Cisco".to_string());
        vendors.insert("00:16:C7".to_string(), "Cisco".to_string());
        vendors.insert("00:16:C8".to_string(), "Cisco".to_string());
        vendors.insert("00:17:0E".to_string(), "Cisco".to_string());
        vendors.insert("00:17:0F".to_string(), "Cisco".to_string());
        vendors.insert("00:17:3B".to_string(), "Cisco".to_string());
        vendors.insert("00:17:59".to_string(), "Cisco".to_string());
        vendors.insert("00:17:5A".to_string(), "Cisco".to_string());
        vendors.insert("00:17:94".to_string(), "Cisco".to_string());
        vendors.insert("00:17:95".to_string(), "Cisco".to_string());
        vendors.insert("00:17:DF".to_string(), "Cisco".to_string());
        vendors.insert("00:17:E0".to_string(), "Cisco".to_string());
        vendors.insert("00:18:18".to_string(), "Cisco".to_string());
        vendors.insert("00:18:19".to_string(), "Cisco".to_string());
        vendors.insert("00:18:39".to_string(), "Linksys/Cisco".to_string());
        vendors.insert("00:18:68".to_string(), "Cisco".to_string());
        vendors.insert("00:18:73".to_string(), "Cisco".to_string());
        vendors.insert("00:18:74".to_string(), "Cisco".to_string());
        vendors.insert("00:18:B9".to_string(), "Cisco".to_string());
        vendors.insert("00:18:BA".to_string(), "Cisco".to_string());
        vendors.insert("00:18:F8".to_string(), "Linksys/Cisco".to_string());
        vendors.insert("00:19:06".to_string(), "Cisco".to_string());
        vendors.insert("00:19:07".to_string(), "Cisco".to_string());
        vendors.insert("00:19:2F".to_string(), "Cisco".to_string());
        vendors.insert("00:19:30".to_string(), "Cisco".to_string());
        vendors.insert("00:19:47".to_string(), "Cisco".to_string());
        vendors.insert("00:19:55".to_string(), "Cisco".to_string());
        vendors.insert("00:19:56".to_string(), "Cisco".to_string());
        vendors.insert("00:19:A9".to_string(), "Cisco".to_string());
        vendors.insert("00:19:AA".to_string(), "Cisco".to_string());
        vendors.insert("00:19:E7".to_string(), "Cisco".to_string());
        vendors.insert("00:19:E8".to_string(), "Cisco".to_string());
        vendors.insert("00:1A:2F".to_string(), "Cisco".to_string());
        vendors.insert("00:1A:30".to_string(), "Cisco".to_string());
        vendors.insert("00:1A:6C".to_string(), "Cisco".to_string());
        vendors.insert("00:1A:6D".to_string(), "Cisco".to_string());
        vendors.insert("00:1A:70".to_string(), "Linksys/Cisco".to_string());
        vendors.insert("00:1A:A1".to_string(), "Cisco".to_string());
        vendors.insert("00:1A:A2".to_string(), "Cisco".to_string());
        vendors.insert("00:1A:E2".to_string(), "Cisco".to_string());
        vendors.insert("00:1A:E3".to_string(), "Cisco".to_string());
        vendors.insert("00:1B:0C".to_string(), "Cisco".to_string());
        vendors.insert("00:1B:0D".to_string(), "Cisco".to_string());
        vendors.insert("00:1B:2A".to_string(), "Cisco".to_string());
        vendors.insert("00:1B:2B".to_string(), "Cisco".to_string());
        vendors.insert("00:1B:53".to_string(), "Cisco".to_string());
        vendors.insert("00:1B:54".to_string(), "Cisco".to_string());
        vendors.insert("00:1B:67".to_string(), "Cisco".to_string());
        vendors.insert("00:1B:8F".to_string(), "Cisco".to_string());
        vendors.insert("00:1B:90".to_string(), "Cisco".to_string());
        vendors.insert("00:1B:D4".to_string(), "Cisco".to_string());
        vendors.insert("00:1B:D5".to_string(), "Cisco".to_string());
        vendors.insert("00:1B:D7".to_string(), "Cisco".to_string());
        vendors.insert("00:1C:0E".to_string(), "Cisco".to_string());
        vendors.insert("00:1C:0F".to_string(), "Cisco".to_string());
        vendors.insert("00:1C:10".to_string(), "Linksys/Cisco".to_string());
        vendors.insert("00:1C:57".to_string(), "Cisco".to_string());
        vendors.insert("00:1C:58".to_string(), "Cisco".to_string());
        vendors.insert("00:1C:B0".to_string(), "Cisco".to_string());
        vendors.insert("00:1C:B1".to_string(), "Cisco".to_string());
        vendors.insert("00:1C:F6".to_string(), "Cisco".to_string());
        vendors.insert("00:1C:F9".to_string(), "Cisco".to_string());
        vendors.insert("00:1D:45".to_string(), "Cisco".to_string());
        vendors.insert("00:1D:46".to_string(), "Cisco".to_string());
        vendors.insert("00:1D:7E".to_string(), "Linksys/Cisco".to_string());
        vendors.insert("00:1D:A1".to_string(), "Cisco".to_string());
        vendors.insert("00:1D:A2".to_string(), "Cisco".to_string());
        vendors.insert("00:1D:E5".to_string(), "Cisco".to_string());
        vendors.insert("00:1D:E6".to_string(), "Cisco".to_string());
        vendors.insert("00:1E:13".to_string(), "Cisco".to_string());
        vendors.insert("00:1E:14".to_string(), "Cisco".to_string());
        vendors.insert("00:1E:49".to_string(), "Cisco".to_string());
        vendors.insert("00:1E:4A".to_string(), "Cisco".to_string());
        vendors.insert("00:1E:6B".to_string(), "Cisco".to_string());
        vendors.insert("00:1E:79".to_string(), "Cisco".to_string());
        vendors.insert("00:1E:7A".to_string(), "Cisco".to_string());
        vendors.insert("00:1E:BD".to_string(), "Cisco".to_string());
        vendors.insert("00:1E:BE".to_string(), "Cisco".to_string());
        vendors.insert("00:1E:E5".to_string(), "Linksys/Cisco".to_string());
        vendors.insert("00:1E:F6".to_string(), "Cisco".to_string());
        vendors.insert("00:1E:F7".to_string(), "Cisco".to_string());
        vendors.insert("00:1F:26".to_string(), "Cisco".to_string());
        vendors.insert("00:1F:27".to_string(), "Cisco".to_string());
        vendors.insert("00:1F:6C".to_string(), "Cisco".to_string());
        vendors.insert("00:1F:6D".to_string(), "Cisco".to_string());
        vendors.insert("00:1F:9D".to_string(), "Cisco".to_string());
        vendors.insert("00:1F:9E".to_string(), "Cisco".to_string());
        vendors.insert("00:1F:C9".to_string(), "Cisco".to_string());
        vendors.insert("00:1F:CA".to_string(), "Cisco".to_string());
        vendors.insert("00:21:1B".to_string(), "Cisco".to_string());
        vendors.insert("00:21:1C".to_string(), "Cisco".to_string());
        vendors.insert("00:21:29".to_string(), "Linksys/Cisco".to_string());
        vendors.insert("00:21:55".to_string(), "Cisco".to_string());
        vendors.insert("00:21:56".to_string(), "Cisco".to_string());
        vendors.insert("00:21:A0".to_string(), "Cisco".to_string());
        vendors.insert("00:21:A1".to_string(), "Cisco".to_string());
        vendors.insert("00:21:BE".to_string(), "Cisco".to_string());
        vendors.insert("00:21:D7".to_string(), "Cisco".to_string());
        vendors.insert("00:21:D8".to_string(), "Cisco".to_string());
        vendors.insert("00:22:0C".to_string(), "Cisco".to_string());
        vendors.insert("00:22:0D".to_string(), "Cisco".to_string());
        vendors.insert("00:22:55".to_string(), "Cisco".to_string());
        vendors.insert("00:22:56".to_string(), "Cisco".to_string());
        vendors.insert("00:22:6B".to_string(), "Linksys/Cisco".to_string());
        vendors.insert("00:22:90".to_string(), "Cisco".to_string());
        vendors.insert("00:22:91".to_string(), "Cisco".to_string());
        vendors.insert("00:22:BD".to_string(), "Cisco".to_string());
        vendors.insert("00:22:BE".to_string(), "Cisco".to_string());
        vendors.insert("00:22:CE".to_string(), "Cisco".to_string());
        vendors.insert("00:23:04".to_string(), "Cisco".to_string());
        vendors.insert("00:23:05".to_string(), "Cisco".to_string());
        vendors.insert("00:23:33".to_string(), "Cisco".to_string());
        vendors.insert("00:23:34".to_string(), "Cisco".to_string());
        vendors.insert("00:23:5D".to_string(), "Cisco".to_string());
        vendors.insert("00:23:5E".to_string(), "Cisco".to_string());
        vendors.insert("00:23:69".to_string(), "Linksys/Cisco".to_string());
        vendors.insert("00:23:AB".to_string(), "Cisco".to_string());
        vendors.insert("00:23:AC".to_string(), "Cisco".to_string());
        vendors.insert("00:23:BE".to_string(), "Cisco".to_string());
        vendors.insert("00:23:EA".to_string(), "Cisco".to_string());
        vendors.insert("00:23:EB".to_string(), "Cisco".to_string());
        vendors.insert("00:24:13".to_string(), "Cisco".to_string());
        vendors.insert("00:24:14".to_string(), "Cisco".to_string());
        vendors.insert("00:24:50".to_string(), "Cisco".to_string());
        vendors.insert("00:24:51".to_string(), "Cisco".to_string());
        vendors.insert("00:24:97".to_string(), "Cisco".to_string());
        vendors.insert("00:24:98".to_string(), "Cisco".to_string());
        vendors.insert("00:24:C3".to_string(), "Cisco".to_string());
        vendors.insert("00:24:C4".to_string(), "Cisco".to_string());
        vendors.insert("00:24:F7".to_string(), "Cisco".to_string());
        vendors.insert("00:24:F9".to_string(), "Cisco".to_string());
        vendors.insert("00:25:2E".to_string(), "Cisco".to_string());
        vendors.insert("00:25:45".to_string(), "Cisco".to_string());
        vendors.insert("00:25:46".to_string(), "Cisco".to_string());
        vendors.insert("00:25:83".to_string(), "Cisco".to_string());
        vendors.insert("00:25:84".to_string(), "Cisco".to_string());
        vendors.insert("00:25:9C".to_string(), "Linksys/Cisco".to_string());
        vendors.insert("00:25:B4".to_string(), "Cisco".to_string());
        vendors.insert("00:25:B5".to_string(), "Cisco".to_string());
        vendors.insert("00:26:0A".to_string(), "Cisco".to_string());
        vendors.insert("00:26:0B".to_string(), "Cisco".to_string());
        vendors.insert("00:26:51".to_string(), "Cisco".to_string());
        vendors.insert("00:26:52".to_string(), "Cisco".to_string());
        vendors.insert("00:26:98".to_string(), "Cisco".to_string());
        vendors.insert("00:26:99".to_string(), "Cisco".to_string());
        vendors.insert("00:26:CA".to_string(), "Cisco".to_string());
        vendors.insert("00:26:CB".to_string(), "Cisco".to_string());
        vendors.insert("00:27:0C".to_string(), "Cisco".to_string());
        vendors.insert("00:27:0D".to_string(), "Cisco".to_string());
        vendors.insert("00:27:0E".to_string(), "Cisco".to_string());
        vendors.insert("00:2A:10".to_string(), "Cisco".to_string());
        vendors.insert("00:2A:6A".to_string(), "Cisco".to_string());
        vendors.insert("00:2B:78".to_string(), "Cisco".to_string());
        vendors.insert("00:2C:31".to_string(), "Cisco".to_string());
        vendors.insert("00:30:19".to_string(), "Cisco".to_string());
        vendors.insert("00:30:24".to_string(), "Cisco".to_string());
        vendors.insert("00:30:40".to_string(), "Cisco".to_string());
        vendors.insert("00:30:71".to_string(), "Cisco".to_string());
        vendors.insert("00:30:78".to_string(), "Cisco".to_string());
        vendors.insert("00:30:7B".to_string(), "Cisco".to_string());
        vendors.insert("00:30:80".to_string(), "Cisco".to_string());
        vendors.insert("00:30:85".to_string(), "Cisco".to_string());
        vendors.insert("00:30:94".to_string(), "Cisco".to_string());
        vendors.insert("00:30:96".to_string(), "Cisco".to_string());
        vendors.insert("00:30:A3".to_string(), "Cisco".to_string());
        vendors.insert("00:30:B6".to_string(), "Cisco".to_string());
        vendors.insert("00:30:F2".to_string(), "Cisco".to_string());
        vendors.insert("00:35:1A".to_string(), "Cisco".to_string());
        vendors.insert("00:38:DF".to_string(), "Cisco".to_string());
        vendors.insert("00:3A:98".to_string(), "Cisco".to_string());
        vendors.insert("00:3A:99".to_string(), "Cisco".to_string());
        vendors.insert("00:3A:9A".to_string(), "Cisco".to_string());
        vendors.insert("00:3A:9B".to_string(), "Cisco".to_string());
        vendors.insert("00:3A:9C".to_string(), "Cisco".to_string());
        vendors.insert("00:40:0B".to_string(), "Cisco".to_string());
        vendors.insert("00:40:96".to_string(), "Cisco".to_string());
        vendors.insert("00:41:D2".to_string(), "Cisco".to_string());
        vendors.insert("00:42:5A".to_string(), "Cisco".to_string());
        vendors.insert("00:50:0B".to_string(), "Cisco".to_string());
        vendors.insert("00:50:0F".to_string(), "Cisco".to_string());
        vendors.insert("00:50:14".to_string(), "Cisco".to_string());
        vendors.insert("00:50:2A".to_string(), "Cisco".to_string());
        vendors.insert("00:50:3E".to_string(), "Cisco".to_string());
        vendors.insert("00:50:50".to_string(), "Cisco".to_string());
        vendors.insert("00:50:53".to_string(), "Cisco".to_string());
        vendors.insert("00:50:54".to_string(), "Cisco".to_string());
        vendors.insert("00:50:73".to_string(), "Cisco".to_string());
        vendors.insert("00:50:80".to_string(), "Cisco".to_string());
        vendors.insert("00:50:A2".to_string(), "Cisco".to_string());
        vendors.insert("00:50:A7".to_string(), "Cisco".to_string());
        vendors.insert("00:50:BD".to_string(), "Cisco".to_string());
        vendors.insert("00:50:D1".to_string(), "Cisco".to_string());
        vendors.insert("00:50:E2".to_string(), "Cisco".to_string());
        vendors.insert("00:50:F0".to_string(), "Cisco".to_string());
        vendors.insert("00:56:2B".to_string(), "Cisco".to_string());
        vendors.insert("00:57:D2".to_string(), "Cisco".to_string());
        vendors.insert("00:59:DC".to_string(), "Cisco".to_string());
        vendors.insert("00:5F:86".to_string(), "Cisco".to_string());
        vendors.insert("00:60:09".to_string(), "Cisco".to_string());
        vendors.insert("00:60:2F".to_string(), "Cisco".to_string());
        vendors.insert("00:60:3E".to_string(), "Cisco".to_string());
        vendors.insert("00:60:47".to_string(), "Cisco".to_string());
        vendors.insert("00:60:5C".to_string(), "Cisco".to_string());
        vendors.insert("00:60:70".to_string(), "Cisco".to_string());
        vendors.insert("00:60:83".to_string(), "Cisco".to_string());
        vendors.insert("00:62:EC".to_string(), "Cisco".to_string());
        vendors.insert("00:64:40".to_string(), "Cisco".to_string());
        vendors.insert("00:6B:F1".to_string(), "Cisco".to_string());
        vendors.insert("00:6C:BC".to_string(), "Cisco".to_string());
        vendors.insert("00:76:86".to_string(), "Cisco".to_string());
        vendors.insert("00:78:88".to_string(), "Cisco".to_string());
        vendors.insert("00:7C:AD".to_string(), "Cisco".to_string());
        vendors.insert("00:81:C4".to_string(), "Cisco".to_string());
        vendors.insert("00:87:31".to_string(), "Cisco".to_string());
        vendors.insert("00:8A:96".to_string(), "Cisco".to_string());
        vendors.insert("00:8E:73".to_string(), "Cisco".to_string());
        vendors.insert("00:90:0C".to_string(), "Cisco".to_string());
        vendors.insert("00:90:21".to_string(), "Cisco".to_string());
        vendors.insert("00:90:2B".to_string(), "Cisco".to_string());
        vendors.insert("00:90:6D".to_string(), "Cisco".to_string());
        vendors.insert("00:90:6F".to_string(), "Cisco".to_string());
        vendors.insert("00:90:86".to_string(), "Cisco".to_string());
        vendors.insert("00:90:92".to_string(), "Cisco".to_string());
        vendors.insert("00:90:A6".to_string(), "Cisco".to_string());
        vendors.insert("00:90:AB".to_string(), "Cisco".to_string());
        vendors.insert("00:90:B1".to_string(), "Cisco".to_string());
        vendors.insert("00:90:BF".to_string(), "Cisco".to_string());
        vendors.insert("00:90:D9".to_string(), "Cisco".to_string());
        vendors.insert("00:90:F2".to_string(), "Cisco".to_string());
        vendors.insert("00:9E:1E".to_string(), "Cisco".to_string());
        vendors.insert("00:A0:C9".to_string(), "Cisco".to_string());
        vendors.insert("00:A2:89".to_string(), "Cisco".to_string());
        vendors.insert("00:A2:EE".to_string(), "Cisco".to_string());
        vendors.insert("00:A6:CA".to_string(), "Cisco".to_string());
        vendors.insert("00:A7:42".to_string(), "Cisco".to_string());
        vendors.insert("00:AD:24".to_string(), "Cisco".to_string());
        vendors.insert("00:AF:1F".to_string(), "Cisco".to_string());
        vendors.insert("00:B0:4A".to_string(), "Cisco".to_string());
        vendors.insert("00:B0:64".to_string(), "Cisco".to_string());
        vendors.insert("00:B0:C2".to_string(), "Cisco".to_string());
        vendors.insert("00:B0:E1".to_string(), "Cisco".to_string());
        vendors.insert("00:B4:A8".to_string(), "Cisco".to_string());
        vendors.insert("00:C0:1D".to_string(), "Cisco".to_string());
        vendors.insert("00:C0:7A".to_string(), "Cisco".to_string());
        vendors.insert("00:C1:64".to_string(), "Cisco".to_string());
        vendors.insert("00:C8:8B".to_string(), "Cisco".to_string());
        vendors.insert("00:CA:E5".to_string(), "Cisco".to_string());
        vendors.insert("00:D0:06".to_string(), "Cisco".to_string());
        vendors.insert("00:D0:28".to_string(), "Cisco".to_string());
        vendors.insert("00:D0:2B".to_string(), "Cisco".to_string());
        vendors.insert("00:D0:3C".to_string(), "Cisco".to_string());
        vendors.insert("00:D0:54".to_string(), "Cisco".to_string());
        vendors.insert("00:D0:58".to_string(), "Cisco".to_string());
        vendors.insert("00:D0:63".to_string(), "Cisco".to_string());
        vendors.insert("00:D0:79".to_string(), "Cisco".to_string());
        vendors.insert("00:D0:90".to_string(), "Cisco".to_string());
        vendors.insert("00:D0:97".to_string(), "Cisco".to_string());
        vendors.insert("00:D0:A8".to_string(), "Cisco".to_string());
        vendors.insert("00:D0:BA".to_string(), "Cisco".to_string());
        vendors.insert("00:D0:BB".to_string(), "Cisco".to_string());
        vendors.insert("00:D0:BC".to_string(), "Cisco".to_string());
        vendors.insert("00:D0:C0".to_string(), "Cisco".to_string());
        vendors.insert("00:D0:D3".to_string(), "Cisco".to_string());
        vendors.insert("00:D0:E4".to_string(), "Cisco".to_string());
        vendors.insert("00:D0:FF".to_string(), "Cisco".to_string());
        vendors.insert("00:D5:8B".to_string(), "Cisco".to_string());
        vendors.insert("00:D6:32".to_string(), "Cisco".to_string());
        vendors.insert("00:DA:55".to_string(), "Cisco".to_string());
        vendors.insert("00:DE:FB".to_string(), "Cisco".to_string());
        vendors.insert("00:E0:14".to_string(), "Cisco".to_string());
        vendors.insert("00:E0:1E".to_string(), "Cisco".to_string());
        vendors.insert("00:E0:34".to_string(), "Cisco".to_string());
        vendors.insert("00:E0:4F".to_string(), "Cisco".to_string());
        vendors.insert("00:E0:8F".to_string(), "Cisco".to_string());
        vendors.insert("00:E0:A3".to_string(), "Cisco".to_string());
        vendors.insert("00:E0:B0".to_string(), "Cisco".to_string());
        vendors.insert("00:E0:F7".to_string(), "Cisco".to_string());
        vendors.insert("00:E0:F9".to_string(), "Cisco".to_string());
        vendors.insert("00:E0:FE".to_string(), "Cisco".to_string());
        vendors.insert("00:E1:6D".to_string(), "Cisco".to_string());
        vendors.insert("00:E3:E4".to_string(), "Cisco".to_string());
        vendors.insert("00:EB:D5".to_string(), "Cisco".to_string());
        vendors.insert("00:F2:8B".to_string(), "Cisco".to_string());
        vendors.insert("00:F6:63".to_string(), "Cisco".to_string());
        vendors.insert("00:FE:C8".to_string(), "Cisco".to_string());

        // Roku devices
        vendors.insert("00:0D:4B".to_string(), "Roku".to_string());
        vendors.insert("08:05:81".to_string(), "Roku".to_string());
        vendors.insert("10:59:32".to_string(), "Roku".to_string());
        vendors.insert("20:F5:43".to_string(), "Roku".to_string());
        vendors.insert("3C:DF:BD".to_string(), "Roku".to_string());
        vendors.insert("48:3F:7A".to_string(), "Roku".to_string());
        vendors.insert("4C:55:B8".to_string(), "Roku".to_string());
        vendors.insert("54:65:DE".to_string(), "Roku".to_string());
        vendors.insert("5C:7D:5E".to_string(), "Roku".to_string());
        vendors.insert("60:61:71".to_string(), "Roku".to_string());
        vendors.insert("88:3D:24".to_string(), "Roku".to_string());
        vendors.insert("8C:49:62".to_string(), "Roku".to_string());
        vendors.insert("98:05:D8".to_string(), "Roku".to_string());
        vendors.insert("AC:3A:7A".to_string(), "Roku".to_string());
        vendors.insert("AC:AE:01".to_string(), "Roku".to_string());
        vendors.insert("B8:3E:59".to_string(), "Roku".to_string());
        vendors.insert("B8:A1:75".to_string(), "Roku".to_string());
        vendors.insert("BC:D7:D4".to_string(), "Roku".to_string());
        vendors.insert("C4:41:7E".to_string(), "Roku".to_string());
        vendors.insert("C8:3A:6B".to_string(), "Roku".to_string());
        vendors.insert("CC:5D:4E".to_string(), "Roku".to_string());
        vendors.insert("D0:4D:2C".to_string(), "Roku".to_string());
        vendors.insert("D4:3A:2E".to_string(), "Roku".to_string());
        vendors.insert("D8:13:99".to_string(), "Roku".to_string());
        vendors.insert("D8:31:34".to_string(), "Roku".to_string());
        vendors.insert("DC:3A:5E".to_string(), "Roku".to_string());
        vendors.insert("FC:C7:7F".to_string(), "Roku".to_string());

        // Raspberry Pi
        vendors.insert("B8:27:EB".to_string(), "Raspberry Pi".to_string());
        vendors.insert("DC:A6:32".to_string(), "Raspberry Pi".to_string());
        vendors.insert("E4:5F:01".to_string(), "Raspberry Pi".to_string());
        vendors.insert("28:CD:C1".to_string(), "Raspberry Pi".to_string());
        vendors.insert("2C:CF:67".to_string(), "Raspberry Pi".to_string());
        vendors.insert("D8:3A:DD".to_string(), "Raspberry Pi".to_string());

        // ASUS devices
        vendors.insert("00:0C:6E".to_string(), "ASUS".to_string());
        vendors.insert("00:0E:A6".to_string(), "ASUS".to_string());
        vendors.insert("00:11:2F".to_string(), "ASUS".to_string());
        vendors.insert("00:11:D8".to_string(), "ASUS".to_string());
        vendors.insert("00:13:D4".to_string(), "ASUS".to_string());
        vendors.insert("00:15:F2".to_string(), "ASUS".to_string());
        vendors.insert("00:17:31".to_string(), "ASUS".to_string());
        vendors.insert("00:18:F3".to_string(), "ASUS".to_string());
        vendors.insert("00:1A:92".to_string(), "ASUS".to_string());
        vendors.insert("00:1B:FC".to_string(), "ASUS".to_string());
        vendors.insert("00:1D:60".to_string(), "ASUS".to_string());
        vendors.insert("00:1E:8C".to_string(), "ASUS".to_string());
        vendors.insert("00:1F:C6".to_string(), "ASUS".to_string());
        vendors.insert("00:22:15".to_string(), "ASUS".to_string());
        vendors.insert("00:23:54".to_string(), "ASUS".to_string());
        vendors.insert("00:24:8C".to_string(), "ASUS".to_string());
        vendors.insert("00:25:D3".to_string(), "ASUS".to_string());
        vendors.insert("00:26:18".to_string(), "ASUS".to_string());
        vendors.insert("00:E0:18".to_string(), "ASUS".to_string());
        vendors.insert("04:92:26".to_string(), "ASUS".to_string());
        vendors.insert("04:9F:CA".to_string(), "ASUS".to_string());
        vendors.insert("04:D4:C4".to_string(), "ASUS".to_string());
        vendors.insert("04:D9:F5".to_string(), "ASUS".to_string());
        vendors.insert("08:1F:F3".to_string(), "ASUS".to_string());
        vendors.insert("08:60:6E".to_string(), "ASUS".to_string());
        vendors.insert("08:62:66".to_string(), "ASUS".to_string());
        vendors.insert("08:BF:B8".to_string(), "ASUS".to_string());
        vendors.insert("0C:9D:92".to_string(), "ASUS".to_string());
        vendors.insert("10:7B:44".to_string(), "ASUS".to_string());
        vendors.insert("10:BF:48".to_string(), "ASUS".to_string());
        vendors.insert("10:C3:7B".to_string(), "ASUS".to_string());
        vendors.insert("14:4D:67".to_string(), "ASUS".to_string());
        vendors.insert("14:DA:E9".to_string(), "ASUS".to_string());
        vendors.insert("14:DD:A9".to_string(), "ASUS".to_string());
        vendors.insert("18:31:BF".to_string(), "ASUS".to_string());
        vendors.insert("1C:87:2C".to_string(), "ASUS".to_string());
        vendors.insert("1C:B7:2C".to_string(), "ASUS".to_string());
        vendors.insert("20:CF:30".to_string(), "ASUS".to_string());
        vendors.insert("24:4B:FE".to_string(), "ASUS".to_string());
        vendors.insert("28:7F:CF".to_string(), "ASUS".to_string());
        vendors.insert("2C:4D:54".to_string(), "ASUS".to_string());
        vendors.insert("2C:56:DC".to_string(), "ASUS".to_string());
        vendors.insert("2C:FD:A1".to_string(), "ASUS".to_string());
        vendors.insert("30:5A:3A".to_string(), "ASUS".to_string());
        vendors.insert("30:85:A9".to_string(), "ASUS".to_string());
        vendors.insert("34:97:F6".to_string(), "ASUS".to_string());
        vendors.insert("38:2C:4A".to_string(), "ASUS".to_string());
        vendors.insert("38:D5:47".to_string(), "ASUS".to_string());
        vendors.insert("3C:7C:3F".to_string(), "ASUS".to_string());
        vendors.insert("3C:98:72".to_string(), "ASUS".to_string());
        vendors.insert("40:16:7E".to_string(), "ASUS".to_string());
        vendors.insert("40:B0:76".to_string(), "ASUS".to_string());
        vendors.insert("48:B0:2D".to_string(), "ASUS".to_string());
        vendors.insert("48:E2:44".to_string(), "ASUS".to_string());
        vendors.insert("4C:ED:FB".to_string(), "ASUS".to_string());
        vendors.insert("50:46:5D".to_string(), "ASUS".to_string());
        vendors.insert("50:EB:71".to_string(), "ASUS".to_string());
        vendors.insert("54:04:A6".to_string(), "ASUS".to_string());
        vendors.insert("54:A0:50".to_string(), "ASUS".to_string());
        vendors.insert("54:B8:0A".to_string(), "ASUS".to_string());
        vendors.insert("58:11:22".to_string(), "ASUS".to_string());
        vendors.insert("5C:FF:35".to_string(), "ASUS".to_string());
        vendors.insert("60:45:CB".to_string(), "ASUS".to_string());
        vendors.insert("60:A4:4C".to_string(), "ASUS".to_string());
        vendors.insert("68:1C:A2".to_string(), "ASUS".to_string());
        vendors.insert("6C:71:D9".to_string(), "ASUS".to_string());
        vendors.insert("6C:FD:B9".to_string(), "ASUS".to_string());
        vendors.insert("70:4D:7B".to_string(), "ASUS".to_string());
        vendors.insert("70:8B:CD".to_string(), "ASUS".to_string());
        vendors.insert("74:D0:2B".to_string(), "ASUS".to_string());
        vendors.insert("78:24:AF".to_string(), "ASUS".to_string());
        vendors.insert("78:DD:12".to_string(), "ASUS".to_string());
        vendors.insert("7C:10:C9".to_string(), "ASUS".to_string());
        vendors.insert("80:C5:F2".to_string(), "ASUS".to_string());
        vendors.insert("84:C9:B2".to_string(), "ASUS".to_string());
        vendors.insert("88:D7:F6".to_string(), "ASUS".to_string());
        vendors.insert("8C:88:2B".to_string(), "ASUS".to_string());
        vendors.insert("90:2B:34".to_string(), "ASUS".to_string());
        vendors.insert("90:E6:BA".to_string(), "ASUS".to_string());
        vendors.insert("94:D7:23".to_string(), "ASUS".to_string());
        vendors.insert("94:DB:C9".to_string(), "ASUS".to_string());
        vendors.insert("9C:5C:8E".to_string(), "ASUS".to_string());
        vendors.insert("9C:8E:CD".to_string(), "ASUS".to_string());
        vendors.insert("9C:BC:F0".to_string(), "ASUS".to_string());
        vendors.insert("A0:F3:C1".to_string(), "ASUS".to_string());
        vendors.insert("A4:DB:30".to_string(), "ASUS".to_string());
        vendors.insert("A8:5E:45".to_string(), "ASUS".to_string());
        vendors.insert("AA:5E:45".to_string(), "ASUS".to_string());
        vendors.insert("AC:22:0B".to_string(), "ASUS".to_string());
        vendors.insert("AC:9E:17".to_string(), "ASUS".to_string());
        vendors.insert("B0:5A:DA".to_string(), "ASUS".to_string());
        vendors.insert("B0:6E:BF".to_string(), "ASUS".to_string());
        vendors.insert("B4:AE:2B".to_string(), "ASUS".to_string());
        vendors.insert("B8:6B:23".to_string(), "ASUS".to_string());
        vendors.insert("BC:52:B7".to_string(), "ASUS".to_string());
        vendors.insert("BC:AE:C5".to_string(), "ASUS".to_string());
        vendors.insert("BC:EE:7B".to_string(), "ASUS".to_string());
        vendors.insert("C4:71:54".to_string(), "ASUS".to_string());
        vendors.insert("C8:60:00".to_string(), "ASUS".to_string());
        vendors.insert("C8:BE:19".to_string(), "ASUS".to_string());
        vendors.insert("D0:17:C2".to_string(), "ASUS".to_string());
        vendors.insert("D4:5D:64".to_string(), "ASUS".to_string());
        vendors.insert("D8:50:E6".to_string(), "ASUS".to_string());
        vendors.insert("DC:85:DE".to_string(), "ASUS".to_string());
        vendors.insert("E0:3F:49".to_string(), "ASUS".to_string());
        vendors.insert("E0:CB:4E".to_string(), "ASUS".to_string());
        vendors.insert("E4:C6:2B".to_string(), "ASUS".to_string());
        vendors.insert("E8:9C:25".to_string(), "ASUS".to_string());
        vendors.insert("F0:2F:74".to_string(), "ASUS".to_string());
        vendors.insert("F0:79:59".to_string(), "ASUS".to_string());
        vendors.insert("F4:32:17".to_string(), "ASUS".to_string());
        vendors.insert("F4:6D:04".to_string(), "ASUS".to_string());
        vendors.insert("F8:32:E4".to_string(), "ASUS".to_string());
        vendors.insert("FC:34:97".to_string(), "ASUS".to_string());

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

        // Extract the OUI (first 3 octets)
        let parts: Vec<&str> = mac_upper.split(':').collect();
        if parts.len() < 3 {
            return None;
        }

        // Create the OUI prefix with proper formatting
        let prefix = format!("{}:{}:{}", parts[0], parts[1], parts[2]);

        self.vendors.get(&prefix).cloned()
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
                "TP-Link" | "Netgear" | "Cisco" | "Linksys/Cisco" | "Cisco/Linksys" | "ASUS" => return "router".to_string(),
                "Roku" => return "tv".to_string(),
                "Raspberry Pi" => return "computer".to_string(),
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