extern crate common;
use std::env;
use std::fs;
use common::day_three::{GeoMap, Slope};

fn main() {
    let args: Vec<String> = env::args().collect();

    let filepath = args.get(1).expect("Could not read file");

    let content = match fs::read_to_string(filepath) {
        Ok(s) => s,
        Err(_) => String::from("")
    };

    let geo_map = GeoMap::from_text(&content);
    let slope = Slope::of(3, 1);
    let count = geo_map.traverse_with_slope(slope);
    print!("\nTree count for slope is {}\n", count);
}
