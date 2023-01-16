extern crate ping;

use std::process::Command;
use crate::logging;

pub fn ping_service(service: String, ipv6: bool, count: u64) {
    let cmd = Command::new("ping")
        .arg(if ipv6 { "-6" } else { "-4" })
        .args(["-c", format!("{}",count).as_str()])
        .arg(service.as_str())
        .output();

    match cmd {
        Err(e) => {
            logging::print_error(format!(
                    "Error in pinging the service {}: {}", service, e
                    ).as_str()
            );
        }
        Ok(out) => {
            logging::print_info(String::from_utf8(out.stdout)
                .unwrap()
                .as_str()
            );
        }
    }
}
