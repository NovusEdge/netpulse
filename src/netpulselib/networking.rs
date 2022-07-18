use std::net;
use crate::logging;
use std::error::Error;
use std::process::Command;

pub fn get_ip(v6: bool) -> Result<net::IpAddr, &'static str> {
    let output = match Command::new("hostname").args(&["-I"]).output() {
        Ok(ok) => ok,
        Err(_) => {
            return Err("unable to fetch ip address");
        }
    };

    Ok(net::IpAddr::V4(net::Ipv4Addr::new(127, 0, 0, 1)))
}

