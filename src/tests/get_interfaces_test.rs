#[cfg(test)]

extern crate regex;

use regex::Regex;
use crate::netpulselib::sysutil;

#[test]
fn test_get_network_interfaces() {
    let interfaces = sysutil::get_network_interfaces().unwrap();
    let pattern = Regex::new(r".*(eth|lo|tun|tap|enp|wlan|wlo|lan).*").unwrap();
    assert_ne!(interfaces.len(), 0, "No interfaces found");
    assert_ne!(interfaces.into_iter()
        .filter(|x| pattern.is_match(x.as_str()))
        .count(), 0, "No familiar interfaces detected");
}
