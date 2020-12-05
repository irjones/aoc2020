extern crate common;

use common::day_four::parse_passports;
use std::fs;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1)
        .expect("No filename provided");
    let content = fs::read_to_string(filename)
        .expect("Failed to read file");

    let passports = parse_passports(&content);

    print!("\nTotal: {} passports\n", passports.len());

    let valid_passports = passports.iter()
        .filter(|p| p.is_valid())
        .count();
    
    print!("\n{} valid passports\n", valid_passports);
}
