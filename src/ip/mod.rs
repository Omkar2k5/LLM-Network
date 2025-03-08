use ipconfig::get_adapters;
use std::{collections::HashSet, net::IpAddr};

pub fn is_my_ip(ip: &str) -> bool {
    if let Ok(parsed_ip) = ip.parse::<IpAddr>() {
        if let Ok(adapters) = get_adapters() {
            for adapter in adapters {
                if adapter.oper_status() == ipconfig::OperStatus::IfOperStatusUp {
                    for my_ip in adapter.ip_addresses() {
                        if *my_ip == parsed_ip {
                            return true;
                        }
                    }
                }
            }
        }
    }
    false
}
pub fn get_my_ips() -> HashSet<String> {
    let mut ips = HashSet::new();
    if let Ok(adapters) = get_adapters() {
        for adapter in adapters {
            if adapter.oper_status() == ipconfig::OperStatus::IfOperStatusUp {
                for ip in adapter.ip_addresses() {
                    ips.insert(ip.to_string());
                }
            }
        }
    }
    ips
}