use colored::*;

use crate::logging;
use crate::netpulselib;

pub fn print_network_interfaces() {
    let interfaces = netpulselib::sysutil::get_network_interfaces().unwrap();
    
    if interfaces.len() > 0 {
        logging::print_success("Found familiar interfaces"); 
    } else {
        logging::print_error("Could not find any fimiliar interfaces");
    }

    for i in interfaces {
       logging::print_info(format!("\tFound Interface: {}", i.on_bright_white()
               .bold()
               .black())
           .as_str());
    }
}
