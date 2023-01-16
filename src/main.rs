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

    let ipv6: bool = matches.is_present("ipv6");
    let count: u64 = if matches.is_present("ping_count") { 
        matches.value_of("ping_count")
            .unwrap()
            .to_string()
            .parse::<u64>()
            .unwrap()
    } else { 
        10 
    };

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
        cli::ip_addresses::print_ip_address(interface, ipv6);
    }

    if matches.is_present("mac_address") {
        cli::mac_address::get_mac_address();
    }

    if matches.is_present("ping") {
        let service = match matches.value_of("ping") {
            Some(i) => { i },
            None => {
                logging::print_error("No service was specified for pinging, aborting...");
                exit(1);
            }
        };
        cli::ping::ping_service(service.to_string(), ipv6, count);
    }
}
