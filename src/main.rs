mod tests;
pub mod cli;
pub mod logging;
pub mod netpulselib;


fn main() {
    cli::interfaces::print_network_interfaces();
    cli::ip_addresses::print_ip_address("tun0", false);
}
