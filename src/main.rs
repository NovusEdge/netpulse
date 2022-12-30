#[macro_use]
extern crate clap;
use clap::App;

mod tests;

pub mod cli;
pub mod logging;
pub mod netpulselib;

use std::process::exit;

fn main() {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from_yaml(yaml).get_matches();

    if matches.is_present("list_interfaces") {
        cli::interfaces::print_network_interfaces();
        exit(0);
    }

    if matches.is_present("fetch_ip") {
        let interface = match matches.value_of("interface") {
            Some(i) => { i },
            None => { 
                logging::print_warning("No interface specified, defaulting to \"lo\"");
                "lo" 
            }
        };

        if matches.is_present("ipv6") {
            cli::ip_addresses::print_ip_address(interface, true);
        } else {
            cli::ip_addresses::print_ip_address(interface, false);
        }
    }

    if matches.is_present("mac_address") {
        cli::mac_address::get_mac_address();
    }
}
