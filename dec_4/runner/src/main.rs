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

    let has_all_fields = passports.iter()
        .filter(|p| p.has_required_fields())
        .count();
    let are_valid = passports.iter()
        .filter(|p| p.has_required_fields())
        .filter(|p| p.fields_are_valid())
        .count();
    
    print!("\n{} passports with all fields present\n", has_all_fields);
    print!("\n{} passports with all fields valid\n", are_valid);
}
