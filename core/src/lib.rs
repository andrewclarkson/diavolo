use std::net::Ipv4Addr;

pub fn hello(name: String) {
    println!("Hello {}", name);
}

pub const ADDRESS: (Ipv4Addr, u16) = (Ipv4Addr::new(127, 0, 0, 1), 8080);
