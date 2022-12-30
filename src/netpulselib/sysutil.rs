extern crate regex;
extern crate mac_address;

use std::env;
use std::str;
use regex::Regex;
use std::process::Command;


use crate::logging;

/// detect_os reports the operating system.
pub fn detect_os() -> &'static str {
    env::consts::OS
}

/// get_network_interfaces fetches the network interfaces present on the system
/// and returns a vector containing their names.
///
/// This uses the `netstat` command.
pub fn get_network_interfaces() -> Option<Vec<String>> {
    let output = Command::new("netstat")
        .args(["-i"])
        .output();
    
    match output {
        Err(error) => {
            logging::print_error(error.to_string().as_str());
            None
        },
        Ok(ifaces) => {
            let mut res: Vec<String> = Vec::new();
            let out = String::from_utf8(ifaces.stdout).unwrap();
            let pattern = Regex::new(r"^([\w\-]+)").unwrap();
            
            for line in (&out).split('\n').skip(2) {
                for m in pattern.find_iter(line) {
                    res.push(m.as_str().to_string());
                }
            }

            Some(res)
        }
    }
}

/// get_device_mac_address fetches the device's mac address and returns it as
/// a String.
///
/// Uses the mac_address crate.
pub fn get_device_mac_address() -> Result<[u8; 6], String> {
    let maddress = mac_address::get_mac_address();
    match maddress {
        Ok(maddr) => {
            if let Some(addr) = maddr {
                Ok(addr.bytes())
            } else {
                Err("Could not fetch MAC address".to_string())
            }
        }
        Err(e) => {
            Err(format!("{:?}", e))
        }
    }
}
