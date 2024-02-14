// implement/impl_ip.rs
// src/network_info.rs

use network_interface::NetworkInterface;
use network_interface::NetworkInterfaceConfig;
use network_interface::Addr;

pub fn list_all_interfaces() -> Vec<NetworkInterface> {
    let network_interfaces = NetworkInterface::show().unwrap();
    for itf in &network_interfaces {
        println!("{:?}", itf);
    }
    network_interfaces
}

pub fn process_network_interfaces() {
    println!("Listing network interfaces:");
    match get_if_addrs::get_if_addrs() {
        Ok(ifaces) => {
            for iface in ifaces {
                if !iface.is_loopback() {
                println!("{}: {:?}", iface.name, iface.addr);
                }
            }
        },
        Err(e) => eprintln!("Failed to get network interfaces: {}", e),
    }
}
