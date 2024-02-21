use scurvy::implement::impl_net::{get_net_info, NetInfoRequest};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_net_info() {
        let mac = get_net_info(NetInfoRequest::MACAddress);
        assert_ne!(mac, "");
        assert_eq!(mac.len(), 17);
    }
}
