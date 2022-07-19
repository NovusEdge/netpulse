pub mod logging;
pub mod netpulselib;

use netpulselib::sysutil;

fn main() {
    println!("{:?}", sysutil::get_network_interfaces().unwrap());
}
