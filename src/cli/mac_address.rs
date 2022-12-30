use crate::logging;
use crate::netpulselib::sysutil;

pub fn get_mac_address() {
    let maddr = sysutil::get_device_mac_address();
    match maddr {
        Ok(addr) => logging::print_info(format!("The device MAC address is: {}", addr).as_str()),
        Err(e) => logging::print_error(e.as_str()),
    }
}
