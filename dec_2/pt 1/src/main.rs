use std::env;
use std::fs;
extern crate common;
use common::day_two::Entry;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filepath = args.get(1).expect("No filepath given!");

    let content = match fs::read_to_string(filepath) {
        Ok(contents) => contents,
        Err(err) => panic!("Unable to read file: {}", err),
    };

    let valid_entries = content
        .split("\n")
        .collect::<Vec<&str>>()
        .iter()
        .filter(|&s| s.len() > 0)
        .map(|&s| Entry::new(s))
        .filter(|er| match er {
            Ok(e) => e.has_letter_enough(),
            Err(_) => false,
        })
        .collect::<Vec<_>>();

    print!("\n---\nValid Entries: {}\n", valid_entries.len());
}
