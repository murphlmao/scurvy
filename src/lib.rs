// brings the `implement` module into scope.
pub mod implement {
    // brings specified modules into scope. (rust files in the `implement` directory)
    pub mod impl_net;
    pub mod impl_bs;
    pub mod impl_file_system;
    pub mod impl_hardware;
}


pub mod functionality_wrapper {
    use crate::implement::impl_net::{get_net_info, NetInfoRequest};
    use crate::implement::impl_bs::argh;
    use crate::implement::impl_file_system::print_dir_tree;
    use crate::implement::impl_hardware::get_hardware_info;

    pub fn print_mac_address() {
        let mac = get_net_info(NetInfoRequest::MACAddress);
        println!("{}", mac);
    }

    pub fn print_network_info() {
        let interface_name = get_net_info(NetInfoRequest::FriendlyName);
        let ip = get_net_info(NetInfoRequest::IPv4Address);
        let mac = get_net_info(NetInfoRequest::MACAddress);
        let gw = get_net_info(NetInfoRequest::IPv4Gateway);
        let sn = get_net_info(NetInfoRequest::IPv4Netmask);
        let hostname = get_net_info(NetInfoRequest::Hostname);

        println!("Primary Interface: {}", interface_name);
        println!("IP Address: {}", ip);
        println!("MAC Address: {}", mac);
        println!("Gateway: {}", gw);
        println!("Subnet: {}", sn);
        println!("Hostname: {}", hostname);
    }

    pub fn print_ip_address() {
        let ip = get_net_info(NetInfoRequest::IPv4Address);
        println!("{}", ip);
    }

    pub fn print_gateway() {
        let gw = get_net_info(NetInfoRequest::IPv4Gateway);
        println!("{}", gw);
    }

    pub fn print_subnet() {
        let sn = get_net_info(NetInfoRequest::IPv4Netmask);
        println!("{}", sn);
    }

    pub fn print_argh() {
        argh();
    }

    pub fn print_hostname() {
        let hostname = get_net_info(NetInfoRequest::IPv4Netmask);
        println!("{}", hostname);
    }

    pub fn directory_tree(path: &str, depth: Option<usize>, current_depth: usize, excludes: Vec<String>) {
        let _dir_tree = print_dir_tree(path, depth, current_depth, "".to_string(), &excludes);

    }

    pub fn display_message(message: &str) {
        println!("The message is: {}", message);
    }

    pub fn print_hardware() {
        let (memory, swap) = get_hardware_info();
        println!("Memory: {:.2}GB", memory);
        println!("Swap: {:.2}GB", swap);
    }
}
