// network implementation functions

// modules
use default_net::interface::get_default_interface;
use default_net::mac::MacAddr; // Import the MacAddr struct

pub enum NetInfoRequest {
    FriendlyName,
    MACAddress,
    IPv4Address,
    IPv4Netmask,
    IPv4Gateway,
    IPv6Address,
}


fn format_mac_address(mac_addr: MacAddr) -> String {
    format!(
        "{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}",
        mac_addr.0, mac_addr.1, mac_addr.2, mac_addr.3, mac_addr.4, mac_addr.5
    )
}


// TODO this might break with multiple network adapters. need to test two active adapters
pub fn get_net_info(request: NetInfoRequest) -> String {
    let interface = get_default_interface().unwrap(); // Assuming this always succeeds for simplicity

    match request {
        NetInfoRequest::FriendlyName => interface.friendly_name.unwrap_or_else(|| "N/A".to_string()),
        NetInfoRequest::MACAddress => format_mac_address(interface.mac_addr.unwrap()), // Ensure this handles None case in actual code
        NetInfoRequest::IPv4Address => interface.ipv4[0].addr.to_string(),
        NetInfoRequest::IPv4Netmask => interface.ipv4[0].netmask.to_string(),
        NetInfoRequest::IPv4Gateway => interface.gateway.map_or("N/A".to_string(), |gateway| gateway.ip_addr.to_string()),
        NetInfoRequest::IPv6Address => interface.ipv6.iter().map(|addr| addr.to_string()).collect::<Vec<_>>().join(", "),
    }
}
