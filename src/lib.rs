// brings the `implement` module into scope.
pub mod implement {
    // brings specified modules into scope. (rust files in the `implement` directory)
    pub mod impl_net;
    pub mod impl_bs;
}


pub mod functionality_wrapper {
    use crate::implement::impl_net::{get_net_info, NetInfoRequest};
    use crate::implement::impl_bs::argh;

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

    pub fn display_message(message: &str) {
        println!("The message is: {}", message);
    }
}
