use colored::*;

pub fn print_info(s: &str) {
    println!("{}", format!("[I]: {}", s).italic().dimmed().white());
}

pub fn print_error(s: &str) {
    println!("{}", format!("[-]: {}", s).red());
}

pub fn print_success(s: &str) {
    println!("{}", format!("[+]: {}", s).green());
}

pub fn print_warning(s: &str) {
    println!("{}", format!("[W]: {}", s).yellow());
}

