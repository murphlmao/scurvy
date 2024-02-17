// brings the `implement` module into scope.
pub mod implement {
    // brings specified modules into scope. (rust files in the `implement` directory)
    pub mod impl_net;
    pub mod impl_bs;
    pub mod impl_file_system;
}


pub mod functionality_wrapper {
    use crate::implement::impl_net::{get_net_info, NetInfoRequest};
    use crate::implement::impl_bs::argh;
    use crate::implement::impl_file_system::print_dir_tree;

    pub fn print_mac_address() {
        let mac = get_net_info(NetInfoRequest::MACAddress);
        println!("{}", mac);
    }

    pub fn print_ip_address() {
        // Directly use `get_net_info` and `NetInfoRequest` here.
        let ip = get_net_info(NetInfoRequest::IPv4Address);
        println!("{}", ip);
    }

    pub fn print_argh() {
        argh();
    }

    pub fn directory_tree(path: &str, depth: Option<usize>, current_depth: usize, excludes: Vec<String>) {
        let _dir_tree = print_dir_tree(path, depth, current_depth, "".to_string(), &excludes);

    }

    pub fn display_message(message: &str) {
        println!("The message is: {}", message);
    }
}
