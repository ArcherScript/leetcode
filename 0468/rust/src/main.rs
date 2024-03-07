use std::net::IpAddr;

pub fn valid_ip_address(query_ip: String) -> String {
    match query_ip.parse::<IpAddr>() {
        _ if query_ip.contains("::") =>  "Neither".to_string(),
        Ok(IpAddr::V4(_)) => "IPv4".to_string(),
        Ok(IpAddr::V6(_))  => "IPv6".to_string(),
        _ => "Neither".to_string(),
    }
}

fn main() {
    println!("Hello, world!");
}
