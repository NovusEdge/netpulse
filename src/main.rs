pub mod logging;
pub mod netpulselib;

fn main() {
    logging::print_info("Information, Information, Information...");
    logging::print_error("Errors, Errors, Errors");
    logging::print_success("SUCCESS!!!!!!");
    logging::print_warning("You have been warned!");
}
