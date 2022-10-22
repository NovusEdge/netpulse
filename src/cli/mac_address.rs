extern crate mac_address;

use mac_address::get_mac_address;

use crate::logging;

pub fn print_mac_address() {
     match get_mac_address() {
        Ok(Some(ma)) => {
            logging::print_success("Mac Address Found!");
            logging::print_info(format!("\tMAC Address: {}", ma).as_str());
        }
        Ok(None) => logging::print_error("Mac address not found!"),
        Err(e) => logging::print_error(format!("Error: {}", e).as_str()),
    }   
}
