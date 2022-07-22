use serde_json;
use crate::logging;

use std::process::Command;

pub fn print_ip_address(interface: &str, ipv6: bool) {
    let cmd = if !ipv6 {
        Command::new("ip")
       .args(["-4", "-a", "-p", "-j", "addr"])
       .output()
       .unwrap()
    } else {
        Command::new("ip")
       .args(["-6", "-a", "-p", "-j", "addr"])
       .output()
       .unwrap()
    };

    let json_string = String::from_utf8(cmd.stdout).unwrap();
    let parsed: Result<serde_json::Value, serde_json::Error> = serde_json::from_str(json_string.as_str());

    match parsed {
        Ok(json_vals) => {
            for vals in json_vals.as_array() {
                for v in vals {
                    if interface == v.as_object().unwrap()["ifname"] {
                        let ip = &v.as_object()
                            .unwrap()["addr_info"]
                            .get(0).unwrap()
                            .as_object().unwrap()["local"].as_str().unwrap();
                        logging::print_info(
                            format!("IP address on interface {}:\n\t{}", interface, ip).as_str());
                    }
                }
            }
        },
        Err(error) => {
            logging::print_error("Could not fetch information about IP\n");
            logging::print_error(error.to_string().as_str());
        }
    }
}

